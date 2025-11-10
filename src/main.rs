use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use json_sift_parser::{convert_to_csv, parse_json}; //, print_structure};
use std::{fs, fs::File, io::Write, path::PathBuf};

#[derive(Parser, Debug)]
#[command(name = "jsonsift", version = "1.0", about = "JsonSift is my first parser. It processes aviation weather METAR data used in civil flights")]
struct Cli{
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    Decode 
    {    file: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    Credits,
}

fn main() -> Result<()>{
    let cli = Cli::parse();
    match cli.cmd{
        Cmd::Decode { file, output } => {
            let s = fs::read_to_string(&file).with_context(||format!("read {:?}", file))?;
            let json = parse_json(&s)?;
            let csv = convert_to_csv(&json)?;
            if let Some(p) = output {
                let mut f = File::create(&p).with_context(||format!("create {:?}", p))?;
                f.write_all(csv.as_bytes())?;
                println!("saved: {:?}", p);
            } else 
            {print!("{csv}");
           }
        }
        Cmd::Credits =>{
            println!("json_sift_parser");
            println!("Author: Vladyslava Spitkovska <spitkovskavlada@gmail.com>");
            println!("Description:JsonSift is my first parser. It processes aviation weather data (METAR) used in civil flights,");
            println!("decoding abbreviations and transforming raw API data into structured CSV format for easier analysis.");
            println!();
        }
    }Ok(())
}
