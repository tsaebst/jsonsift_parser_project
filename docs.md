# JsonSift Documentation

This document defines the project's features, setup, and usage.

# About project

**JSON-Sift** is a parser that works with weather data of civil air flights that come from APIs in JSON format.  
Such data contain various specific notations and a particular way of arrangement. It decrypts particular data and transforms it into a CSV format.

## Table of Contents

*   [Purspose](#purpose)
*   [Installation](#installation)
*   [Logic](#logic)
*   [Grammar](#grammar)

## Purpose

This parser is for METAR weather reports using `pest`,and helper utilities to export the parsed structure into JSON and CSV formats. 
It is intended for tools and services that need reliable, machine-readable METAR data in CSV format, which is the most suitable for analysis.

## Installation

To get started, follow these instructions: 

```
bash
git clone https://github.com/tsaebst/json_sift_parser_upd
cd json_sift_parser
cargo build
cargo install --path .
```

To see all commands availible in parser do:

```
jsonsift --help

```


## Logic

Detailed pipeline of my padser is divided into 2 parts: lib.rs and metar.rs for metar part.
It can be displayed as :

JSON input
*  -> parse_json
*  ->flatten
*  -> parse_scalar
*      -> Metar data:
*          decode_metar -> SiftParser ->visit_metar -> SimplePattern -> apply_pattern ->normalized METAR fields
*      -> not Metar:
*          heuristics or token_n
*  -> merge 
*  -> convert_to_csv
*  -> CSV output



## `src/lib.rs`
My parser tries to be as flexible as possble, so I made it friendly to variations of Metar data

* `parse_json()`
Parses input string as JSON using `serde_json::from_str`

* `convert_to_csv()`
gets JSON object or array. flattens each entry, collects all keys as CSV headers, and writes rows via `csv::Writer` using sorted columns

* `flatten()`
Recursively walks though objects, arrays, scalars in json, builds indexed keys, and redirects string vals to `parse_scalar`

* `parse_scalar()`
Normalizes str, tries to decode it as METAR via `metar::decode_metar`. if not - tokenizes and uses simple metar patterns or creates `token_n` columns

---

## `src/metar.rs`

* `SiftParser`
Pest-generated parser using `grammar.pest` rules for METAR reports.

* `decode_metar()`
Parses a full METAR string with `SiftParser`, walks through parse tree, and returns a flat map of normalized METAR fields/`None`

* `visit_metar()`
visits Pest parse pairs, matches basic rules, and fills the output map by using `apply_pattern` where possible 

* `complex_key_value()`
Splits a random string into tokens by whitespace and basic separators before pattern detection

* `is_code_like_token()` / `all_tokens_code_like()`
Detects whether tokens look like uppercase/number codes to decide if there's a pattern

* `SimplePattern`
Enum for recognized token types `TempDew`, `Wind`, `Pressure`, `Time`, `Visibility`, `Cloud`, `FlightCategory`.

* `holds_pattern_value()`
Classifies a single token into one of the `SimplePattern` variants

* `apply_pattern()`
Expands a recognized pattern token into one or more well-named columns 

* `norm()`
Normalizes raw text
---


## Grammar

Parser uses grammar down below for METAR-like data:

```
WHITESPACE = _{ " " | "\t" | "\r" | "\n" }
ASCII_UPPER_ALPHA = _{ 'A'..'Z' }

metar_report = { SOI ~ token* ~ EOI }

time = { ASCII_DIGIT{6} ~ "Z" }

station = { SOI ~ ASCII_UPPER_ALPHA{4} ~ EOI }

wind_dir= {ASCII_DIGIT{3}} // 3 digits
wind_speed= { ASCII_DIGIT{2,3} } // 2 or 3 digits
wind_gust = {"G"~ASCII_DIGIT{2,3} } // optional G+num
wind_units= {"KT" | "MPS"}

wind = {wind_dir~wind_speed ~ wind_gust?~wind_units}//? baceuse might be absent

visibility = {(ASCII_ALPHA)?~ // one upperc prefix char
    //num + "" + num + "/" + num;  num + "/" + num; or num
    ((ASCII_DIGIT+ ~ " " ~ ASCII_DIGIT+ ~ "/" ~ ASCII_DIGIT+) | (ASCII_DIGIT+ ~ "/" ~ ASCII_DIGIT+)
    | (ASCII_DIGIT+)) ~ "SM" // num + somethin
}

cloud_cover = { "FEW" | "SCT" | "BKN" | "OVC" }
cloud_alt = { ASCII_DIGIT{3} } //altitude 
clouds = {cloud_cover~cloud_alt | "CLR" | "SKC"}


temp = { "M"? ~ ASCII_DIGIT{2} }
dew = { "M"? ~ ASCII_DIGIT{2} }
temp_dew = { temp ~ "/" ~ dew }

pressure = { "A" ~ ASCII_DIGIT{4} }

remarks = { "RMK" ~ (!NEWLINE ~ ANY)* }

known_keyword = {"COR" | "AUTO" | "AMD" | "TEMPO" | "NOSIG" }

uppercase_token = @{ ASCII_UPPER_ALPHA{2,} }

separator = _{ WHITESPACE+ }

token = _{station| time| wind|visibility|clouds|temp_dew| pressure|remarks| known_keyword| uppercase_token| separator| unknown_token}

unknown_token = @{ (!WHITESPACE ~ ANY)+ }
```
