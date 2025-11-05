use pest::Parser;
use pest_derive::Parser;

//defining the parser sctuct
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SiftParser;

//cehck if metar report is parsed
#[test]
fn parse_basic() {
    let input = "UKBB 121200Z 18005KT 10SM FEW020 15/10 A2992 RMK TEST";
    let result = SiftParser::parse(Rule::metar_report, input);
    assert!(result.is_ok(), "Parse failed: {:?}", result.err());
}

//if known words are recognised
#[test]
fn parse_known() {
    for s in ["COR", "AUTO", "AMD", "TEMPO", "NOSIG"] {
        assert!(
            SiftParser::parse(Rule::known_keyword, s).is_ok(),
            "Failed on{s}"
        );
    }
}

//uppercase token test
#[test]
fn parse_uppercase() {
    for s in ["ABC", "XYZ", "RMKX"] {
        assert!(
            SiftParser::parse(Rule::uppercase_token, s).is_ok(),
            "Failed on{s}"
        );
    }
}

//wind test
#[test]
fn parse_wind() {
    let input = "18005KT";
    let result = SiftParser::parse(Rule::wind, input);
    assert!(result.is_ok(), "Wind parse failed: {:?}", result.err());
}

//station test
#[test]
fn parse_station() {
    let ok = ["UKBB", "EGLL", "KJFK"];
    for s in ok {
        assert!(SiftParser::parse(Rule::station, s).is_ok(), "Failed on{s}");
    }
}

//invalid station test
#[test]
fn parse_station_invalid() {
    let bad = ["UKB", "UKBBB", "12AB", "aaaa"];
    for s in bad {
        assert!(
            SiftParser::parse(Rule::station, s).is_err(),
            "Should fail on {s}"
        );
    }
}

//time valid+invalid tests
#[test]
fn parse_time_valid() {
    assert!(SiftParser::parse(Rule::time, "121200Z").is_ok());
}

#[test]
fn parse_time_invalid() {
    assert!(SiftParser::parse(Rule::time, "1212Z").is_err());
}

// same for wind: valid+invalid
#[test]
fn parse_wind_valid() {
    for s in ["18005KT", "25010G15KT", "00015MPS"] {
        assert!(SiftParser::parse(Rule::wind, s).is_ok(), "Failed on {s}");
    }
}
#[test]
fn parse_wind_invalid() {
    for s in ["180KT", "0505KT", "99999", "18005"] {
        assert!(
            SiftParser::parse(Rule::wind, s).is_err(),
            "Should fail on {s}"
        );
    }
}

// also visibility tests
#[test]
fn parse_visibility_valid() {
    for s in ["10SM", "3/4SM", "M1/2SM", "P6SM"] {
        assert!(
            SiftParser::parse(Rule::visibility, s).is_ok(),
            "Failed on {s}"
        );
    }
}

#[test]
fn parse_visibility_invalid() {
    for s in ["10", "10S", "SM10", "MM1SM"] {
        assert!(
            SiftParser::parse(Rule::visibility, s).is_err(),
            "Should fail on {s}"
        );
    }
}

//clouds
#[test]
fn parse_clouds_valid() {
    for s in ["FEW020", "BKN100", "OVC200", "SCT030", "CLR", "SKC"] {
        assert!(SiftParser::parse(Rule::clouds, s).is_ok(), "Failed on {s}");
    }
}

#[test]
fn parse_clouds_invalid() {
    for s in ["FEW", "BKN20", "CLOUDY", "FEW02O"] {
        assert!(
            SiftParser::parse(Rule::clouds, s).is_err(),
            "Should fail on {s}"
        );
    }
}

//twmp dew
#[test]
fn parse_temp_dew_valid() {
    for s in ["15/10", "M02/M05", "00/00"] {
        assert!(
            SiftParser::parse(Rule::temp_dew, s).is_ok(),
            "Failed on {s}"
        );
    }
}
#[test]
fn parse_temp_dew_invalid() {
    for s in ["15/", "/10", "15-10"] {
        assert!(
            SiftParser::parse(Rule::temp_dew, s).is_err(),
            "Should fail on {s}"
        );
    }
}

// pressure
#[test]
fn parse_pressure_valid() {
    for s in ["A2992", "A1000", "A0000"] {
        assert!(
            SiftParser::parse(Rule::pressure, s).is_ok(),
            "Failed on {s}"
        );
    }
}

#[test]
fn parse_pressure_invalid() {
    for s in ["2992", "AA992", "A29"] {
        assert!(
            SiftParser::parse(Rule::pressure, s).is_err(),
            "Should fail on {s}"
        );
    }
}

// remarks
#[test]
fn parse_remarks_valid() {
    for s in ["RMK TEST", "RMK AO2 SLP123", "RMK"] {
        assert!(SiftParser::parse(Rule::remarks, s).is_ok(), "Failed on {s}");
    } // must work
}

//full metar example
#[test]
fn parse_full_metar_report() {
    let input = "UKBB 121200Z 18005KT 10SM FEW020 15/10 A2992 RMK TEST";
    let result = SiftParser::parse(Rule::metar_report, input);
    assert!(result.is_ok(), "Full METAR failed: {:?}", result.err());
}
