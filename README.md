# JSON-Sift

**JSON-Sift** is a parser that works with weather data of civil air flights that come from APIs in JSON format.  
Such data contain various specific notations and a particular way of arrangement.  
This parser deals with recognizing embedded codes and transforming JSON into **CSV**,  
which is the most common format for working with data, processing, and analysis.  

I often work with data, and such a parser would make my work easier if, for example,  
I wanted to train a **model** on it or perform **EDA**.

---
> [!NOTE]
> Name selection

“Sift” in ukrainian means *просіювати*.  
Our data come in a very unclear format — sometimes presented just as a line of abbreviations and numbers,  
which is not visually understandable.  
My parser sifts this data through its filters and outputs data that can be worked with.  
That is why I named my project this way.

---
## Data Source
> [!NOTE]
> At the moment, the parser works with data from corresponding APIs.  
For demonstration purposes, the data are taken from the **AviationWeather (METAR)** API:  
[https://aviationweather.gov/help/data/#metar](https://aviationweather.gov/help/data/#metar)

---

## Purpose of the project

Currently, as I have already mentioned, the parser works with data from civil aviation flights.  
In general, the parser can be adapted to decode flight data of other flying devices such as **drones**,  
since this is a relevant topic in Ukraine.  

Since I don’t have access to real drone flight data, I use alternative data sources.  
In the future, if desired, the parser may include the possibility of configuration via a config file,  
in case the incoming data have a slightly different structure.

---
## Example of transformed data
Below is an example of how the raw aviation weather data looks **after being parsed and converted** into structured CSV format :

![Example of transformed data](example_transformed_data.png)
 --- 
## Getting Started

> [!TIP]
> To download the project use commands:
```
bash
git clone https://github.com/tsaebst/json_sift_parser.git
cd json_sift_parser
cargo build
cargo run

```

To start working, you need to install the project locally  
*(add detailed installation and run instructions here)*.

To begin, type:
```
make help
```

# Project files:

```
json_sift_parser/
├── Cargo.toml              # metadata and dependencies
├── Makefile                # CLI build + tests
├── README.md               # project doumentation
├── config.json             # parser patterns and rules config
├── src/
│   ├── grammar.pest        # Metar grammar defining
│   ├── lib.rs              # parsing and transformation logic (to be done !!!!)
│   └── main.rs             # cli entry point (to be done !!!!)
├── tests/
│   └── parser_tests.rs     # unit-tests for grammar (to be aaded for parsing logic)
├── result.csv              # outout CSV
└── test.json               # json input data
```


## grammar.pest
The METAR grammar describes how the parser recognizes weather observation strings.  
These strings typically consist of compact tokens( combinations of letters, digits, and abbreviations) — that encode different metrics

---
## About grammar

> [!IMPORTANT]
> typical input looks like this:
> UKBB 121200Z 18005KT 10SM FEW020 15/10 A2992 RMK TEST

## Each segment is a token representing a distinct type of weather information.  
The grammar processes them using `pest` rules as follows:

| Rule | Meaning | Example |
|------|----------|---------|
| `station` | 4-letter station code | `UKBB`, `KJFK`, `EGLL` |
| `time` | UTC timestamp in `HHMMSSZ` format | `121200Z` |
| `wind` | Wind direction, speed, optional gust, and units | `18005KT`, `25010G15KT` |
| `visibility` | Horizontal visibility with optional prefixes | `10SM`, `M1/2SM`, `P6SM` |
| `clouds` | Cloud layers or clear condition | `FEW020`, `BKN100`, `CLR` |
| `temp_dew` | Temperature / dew point pair | `15/10`, `M02/M05` |
| `pressure` | Atmospheric pressure (inHg) | `A2992` |
| `remarks` | Free-text remarks | `RMK AO2 SLP123` |
| `known_keyword` | Recognized control words | `COR`, `AUTO`, `NOSIG` |
| `uppercase_token` | Any unknown uppercase abbreviation | `VV`, `CB`, `TS` |
| `separator` | Whitespace or line breaks | `" "` or `"\n"` |
| `unknown_token` | Fallback for any unrecognized token | `XYZ123` |

---

## Tests

JSON-Sift includes a set of unit tests (written via cargo)to verify the correctness of the METAR grammar and the future parsing logic implemented in `lib.rs`.  

| Test Type | Description |
|------------|-------------|
| **Grammar tests (`parser_tests.rs`)** | Validate the grammar rules defined in `grammar.pest`. Each METAR component (station, time, wind, etc.) is parsed and checked for correctness. |
| **Parsing logic tests** *(planned)* | Will validate transformation from raw METAR strings into structured JSON or CSV. |
| **JSON/CSV conversion tests** *(future work)* | Ensure flattened JSON structure and correct CSV export. |


To run all unit tests:

make test

---
> [!WARNING]
> to be done
## Parsing logic in `lib.rs` 

This part of program is built on next key ideas:

1. Everything starts as JSON  
2. Complex JSON trees are converted into flat key–value pairs for export.  
3. Transformation — parsed data are transformed into csv

---

### `parse_json()`

- **Goal:** validate and load incoming API data.  
- **If successful:** returns a `serde_json::Value` (can be `Object`, `Array`, etc.).  
- **If failed:** returns a `ParseError::JsonError`.

### `print_structure()`
Recursively prints the internal structure of a JSON value.  
Used for inspeting input data and understanding its nesting.

### `flatten_json()`
Flattens a nested JSON object into simple key–value pairs.  
When it finds `"rawOb"` (a raw METAR string), it automatically decodes it using `parse_raw_ob`.

### `convert_to_csv()`
Takes structured JSON and produces a CSV string.

### `parse_raw_ob()`

Parses a single METAR weather string using the grammar in grammar.pest. Recognizes patterns, dechipers them, so that we could separae detected values into different columns.

### ParseError

* JsonError — invalid or malformed JSON
* StructureError — grammar parsing or format mismatch


---
> [!WARNING]
> to be done

## MAIN file

The `main.rs` file defines the **CLI interface** and the high-level program flow.  
Its main goal is to connect logic from `lib.rs` with  user commands.

Before executing any of the commands, the program uses functions from `lib.rs`  to perform all data handling.

As a concept for now, but a plan for the future i might add: 
- Loads a configuration file (`config.json`) if available.



## How to run

You can interact with **JSON-Sift** directly from the terminal using Cargo or Make commands.

- **Run the program:**
``` 
bash
cargo run
```

- **Parse and save**
```
make decode FILE=test.json OUT=result.csv CONFIG=config.json
```

- **Credits**
```
cargo run -- credits

```
## Author
**Vladyslava Spitkovska** – [GitHub](https://github.com/tsaebst)
