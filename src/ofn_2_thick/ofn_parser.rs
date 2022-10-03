use serde_json::{Value};

pub fn parse_ofn(t: &str) -> Value {
    let thick_triple: Value = serde_json::from_str(t).unwrap(); 
    thick_triple
}

