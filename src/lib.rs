#![doc = include_str!("../docs.md")]

mod metar;
pub use metar::{Rule, SiftParser};
use csv::WriterBuilder;
use serde_json::Value;
use std::collections::{BTreeSet, HashMap};
use thiserror::Error;

// errors: json + structure + optional detector/pattern
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("JSON: {0}")]
    Json(String),
    #[error("Structure: {0}")]
    Structure(String),
}

//parse raw json string into serde value
pub fn parse_json(s: &str) -> Result<Value, ParseError>{
    serde_json::from_str(s).map_err(|e| ParseError::Json(e.to_string()))
    // if json parse fails, i wrap error into the custom ParseError
}

// main logic == flatten json =>rows=>csv
pub fn convert_to_csv(v: &Value)->Result<String, ParseError>{
    //hashing all rows
    let mut rows = Vec::<HashMap<String, String>>::new();
    //using tree set to keep keys sorted + uniqe
    let mut keys = BTreeSet::new();
    match v{
        // if array of obj
        Value::Array(a)=>{
            for it in a{
                // new flat map for each element
                let mut m= HashMap::new();
                flatten(it, "".into(), &mut m)?;
                // remember all col names from row
                for k in m.keys()
                {keys.insert(k.clone()); // clone because set owns the Str
                }
                rows.push(m);
            }
        }
        // if it's a single object
        Value::Object(_)=>{
            //same as above but only once
            let mut m =HashMap::new();
            flatten(v, "".into(), &mut m)?;
            //col names
            for k in m.keys(){
                keys.insert(k.clone());
            }
            rows.push(m);
        }
        // everything else == err
        _ =>return Err(ParseError::Structure("expect object or array".into())),
    }
    // starting a header row from all keys
    let hdr: Vec<String> =keys.into_iter().collect();

    // use csv writer so it handles quoting/escaping
    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_writer(Vec::new());
    // header creation (for csv
    wtr.write_record(&hdr).map_err(|e| ParseError::Structure(e.to_string()))?;
    // rows
    for row in rows {
        // for each column in fixed order
        let record = hdr.iter().map(|col| {
            // get cell value or empty string if missing
            row.get(col).map_or("", |v| v.as_str())});
        wtr.write_record(record).map_err(|e| ParseError::Structure(e.to_string()))?;
    }
    // get underlying vec and convert to Str
    let buf = wtr.into_inner().map_err(|e| ParseError::Structure(e.to_string()))?;
    let out = String::from_utf8(buf).map_err(|e| ParseError::Structure(e.to_string()))?;
    Ok(out)
}

//detectors look only at str content not field name !!!!!  for METAR etc
//if true == metar with be in col name
const PREFIX_WITH_DETECTOR_NAME: bool =true;

// flatten json recursively
//v is curr val
//prefix is col name prefix
fn flatten(v: &Value, prefix:String,out:&mut HashMap<String, String>)->Result<(), ParseError>{
    match v{
        Value::Object(m)=>{
            for (k, vv) in m{
                //if prefix empty, key is k, else prefix.k
                let key = if prefix.is_empty(){
                    k.clone()
                } else{
                    format!("{prefix}.{k}")
                };
                flatten(vv, key, out)?;
            }
        }
        Value::Array(a)=>{
            //if arr = [v0,v1] then cols [pref[0], pref[1]]
            for (i, vv) in a.iter().enumerate(){
                flatten(vv, format!("{}[{}]", prefix, i), out)?;
            }
        }
        Value::String(s)=>parse_scalar(prefix, s, out)?,
        Value::Number(n) =>{
            out.insert(prefix, n.to_string());
        }
        Value::Bool(b) => {
            out.insert(prefix, b.to_string());
        }
        Value::Null => {
            out.insert(prefix,"".into());
        }
    }
    Ok(())
}

//scalar pipeline for str val
fn parse_scalar(prefix: String,s:&str,out:&mut HashMap<String, String>,)->Result<(), ParseError>{
    let text = s.trim();
    if text.is_empty(){
        out.insert(prefix, String::new());
        return Ok(());
    }
    //full-string detectors (only metar for now)
    if let Some(mut decoded)=metar::decode_metar(text){
        //if metar parse ok
        let det_name="metar";
        for (dk, dv) in decoded.drain(){
            //drain() puts out decoded key-values
            // build column name with/without detector prefix
            let col = if prefix.is_empty(){
                if PREFIX_WITH_DETECTOR_NAME{
                    format!("{det_name}.{dk}")
                } else{dk}
            } else if PREFIX_WITH_DETECTOR_NAME{
                format!("{prefix}.{det_name}.{dk}")
            } else
            {format!("{prefix}.{dk}")
            };
            out.insert(col,dv);//insert in out map
        }
        return Ok(());
    }
    // cuts into toeksn
    let tokens = metar::complex_key_value(text);

    // if nothing or 1 token then keep string or try single-pattern
    if tokens.is_empty(){
        out.insert(prefix, text.to_string());
        return Ok(());
    }
    if tokens.len()==1{
        let t = tokens[0].trim();
        if let Some(pat) = metar::holds_pattern_value(t) 
        { metar::apply_pattern(&prefix, t, pat, out);
        } else{
            out.insert(prefix,text.to_string());
        }
        return Ok(());
    }
    //if looks like normal human phrase
    if !metar::all_tokens_code_like(&tokens){
        out.insert(prefix, text.to_string());
        return Ok(());
    }
    //for code alike  tokens apply patterns
    let mut i = 0; //counter
    for t in tokens {
        let t = t.trim();
        if t.is_empty(){
            continue;}
        //check for simple patterns
        if let Some(pat) = metar::holds_pattern_value(t){
            metar::apply_pattern(&prefix, t, pat, out);
            continue;
        }
        // if not write as token_n under
        let col = if prefix.is_empty(){
            format!("token_{i}")
        } else{format!("{prefix}.token_{i}")};
        out.insert(col, t.to_string());
        i += 1;
    }
    Ok(())
}
