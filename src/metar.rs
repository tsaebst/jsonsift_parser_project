use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
// metar grammar via pest, see grammar.pest
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SiftParser;
//part for detecting known patts
#[derive(Clone, Copy)]
pub enum SimplePattern {TempDew, Wind,Pressure,Time,Visibility,Cloud, FlightCategory,}//simple patts

// parse full metar string via pest into flat map
pub fn decode_metar(s: &str) -> Option<HashMap<String, String>>{
    let pairs = SiftParser::parse(Rule::metar_report, s).ok()?;//trying to parse via pest
    let mut out = HashMap::new();
    for p in pairs {
        visit_metar(&p, &mut out);
    }
    if out.is_empty(){ None } else { Some(out) }//return decoded/none
}

//split and simple patterns
pub fn complex_key_value(s:&str)->Vec<String>{
    let s = s.trim();
    if s.is_empty() {
        return vec![];
    }
    let mut res=Vec::new();
    for c in s.split(|c: char| c.is_whitespace()||c ==';'||c == ','||c == '|'){
        //split by whitespace + common separators
        let t = c.trim();
        if !t.is_empty()
        {res.push(t.to_string());
        }
    }
    if res.is_empty(){
        res.push(s.to_string());
    }
    res
}

//check if token is uppercase + digits only
fn is_code_like_token(t:&str)->bool{
    let core: String = t.chars().filter(|c| c.is_ascii_alphanumeric()).collect();//keep only chars+dig
    if core.is_empty(){ return false;}
    core.chars().all(|c|c.is_ascii_uppercase()||c.is_ascii_digit())
}

//check if all tokens are is_code_like_token
pub fn all_tokens_code_like(tokens:&[String])->bool{
    !tokens.is_empty() && tokens.iter().all(|t| is_code_like_token(t))
}

pub fn holds_pattern_value(t:&str)->Option<SimplePattern>{
    let t = t.trim();
    if t.is_empty(){return None;}

    if t.contains('/') && t.len()<=6 && t.split('/').count() ==2{
        return Some(SimplePattern::TempDew);
    }

    if t.ends_with("KT") && t.len()>= 5{
        let core = &t[..t.len()- 2];
        if core.len()>=5 && core[..3].chars().all(|c|c.is_ascii_digit()){
            return Some(SimplePattern::Wind);
    }
    }
    if t.starts_with('A')&& t.len() == 5&& t[1..].chars().all(|c| c.is_ascii_digit()){
        return Some(SimplePattern::Pressure);
    }
    if t.ends_with('Z')&& t.len() == 7 && t[..6].chars().all(|c| c.is_ascii_digit()){
        return Some(SimplePattern::Time);
    }
    if t.ends_with("SM"){
        return Some(SimplePattern::Visibility);
    }
    // clouds
    if t.starts_with("BKN") || t.starts_with("SCT") || t.starts_with("FEW") || t.starts_with("OVC")
    {
        return Some(SimplePattern::Cloud);
    }
    if t == "CLR" || t == "SKC"{
        return Some(SimplePattern::Cloud);
    }
    // allow diff variants containing VFR
    if t.contains("VFR") && t.chars().all(|c| c.is_ascii_alphabetic()) {
        return Some(SimplePattern::FlightCategory);
    }None
}

pub fn apply_pattern(prefix: &str,token: &str, pat: SimplePattern,out: &mut HashMap<String, String>,){
    let base=if prefix.is_empty(){
        String::new()
    } else{format!("{prefix}.")}; //amking base name for col
    let col =|name: &str|{
        if base.is_empty(){
            name.to_string()
        } else{
            format!("{base}{name}")
    }};
    match pat{
        SimplePattern::TempDew=>{
            let p: Vec<&str> = token.split('/').collect(); //split into 2 hlfs
            if p.len()==2{
                out.insert(col("temp_c"),p[0].replace('M', "-")); //M == minus
                out.insert(col("dewpoint_c"),p[1].replace('M', "-"));
            } else {
                out.insert(col("tempdew_raw"),token.into()); //if more than 2 parts
        }
        } //same logic for next known patterns
        SimplePattern::Wind=>{
            let core= &token[..token.len() - 2];// drop KT
            let (dir,rest) = core.split_at(3);
            out.insert(col("wind_direction"), dir.into());
            if let Some(g) =rest.find('G'){
                out.insert(col("wind_speed"),rest[..g].into());
                out.insert(col("wind_gust"), rest[g + 1..].into());
            } else{
            out.insert(col("wind_speed"), rest.into());
            }
           out.insert(col("wind_units"), "KT".into());
        }
        SimplePattern::Pressure=>{
            if let Ok(v) =token[1..].parse::<f32>(){
                out.insert(col("pressure_inhg"), format!("{:.2}", v / 100.0));
            } else {
                out.insert(col("pressure_raw"), token.into()); // into is same as to_str
        }}
        SimplePattern::Time=>{out.insert(col("time"), token.into());}
        SimplePattern::Visibility=>{
            let v=token.trim_end_matches("SM").trim();
            out.insert(col("visibility_sm"), v.into());
        }

        SimplePattern::Cloud=>{
            let code = if token.len() >= 3 { &token[..3] } else { token };
            let cover_str = match code {"BKN" => "broken","SCT" => "scattered","FEW" => "few","OVC" => "overcast","CLR" => "clear","SKC" => "clear",_ => code,};
            out.insert("cloud_cover".into(), cover_str.into());
            if token.len()>3{
                if let Ok(v) = token[3..].parse::<u32>(){
                    out.insert("cloud_altitude_ft".into(),(v * 100).to_string());
                } else {
                    out.insert("cloud_raw".into(),token.into());
                }
            }
        }
        SimplePattern::FlightCategory =>{
            // store raw token like VFR / MVFR etc
            out.insert("flight_category".into(),token.into());
    }}
}

fn visit_metar(pair: &pest::iterators::Pair<Rule>, out: &mut HashMap<String, String>){
    let text = norm(pair.as_str()); // normalize raw text from this node
    match pair.as_rule() {
        // station already validated by grammar
        Rule::station => {
        out.insert("station".into(), text.clone());
        }
        //time already validated by grammar
        Rule::time =>{
        apply_pattern("", &text, SimplePattern::Time, out);
        }
        //wind: use same logic as for tokens
        Rule::wind => {
        apply_pattern("", &text, SimplePattern::Wind, out);
        }
        Rule::visibility=>{
        apply_pattern("", &text, SimplePattern::Visibility, out);
        }
        Rule::clouds =>{
        apply_pattern("", &text, SimplePattern::Cloud, out);
        }
        Rule::temp_dew=>{
        apply_pattern("", &text, SimplePattern::TempDew, out);
        }
        Rule::pressure=> {
        apply_pattern("", &text, SimplePattern::Pressure, out);
        }
        _ => {}//ignore else
    }
    //recursion for into_inner== children of this node in pest parse tree
    for inner in pair.clone().into_inner(){
        visit_metar(&inner, out);
}
}

//utils
#[inline]
//normalize
fn norm(s:&str)->String{
    let mut t=s.trim().trim_end_matches(['=','+']).trim().to_string();
    t=t.replace(char::is_whitespace," ");
    while t.contains("  "){ t=t.replace("  "," ");}
    t
}