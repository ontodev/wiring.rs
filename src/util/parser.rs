use serde_json::{Value, Result as SResult};
use crate::owl::thick_triple as tt;

/// Given the object of an LDTab ThickTriple (encoded as a string),
/// return a thick_triple::OWL struct 
/// 
/// #Examples
/// 
/// let object = r#"{"owl:someValuesFrom": [{"object": "obo:OBI_0500000",
///                                          "datatype":"_iri",
///                                          "meta":null}],
///                  "rdf:type": [{"object": "owl:Restriction",
///                                "datatype":"_iri",
///                                "meta":null}],
///                  "owl:onProperty": [{"object": "obo:BFO_0000050",
///                                      "datatype":"_iri",
///                                      "meta":null}]}"#;
///
/// let owl = ldtab_2_ofn::parser::parse_thick_triple_object(&object); 
/// println!("{:?}", owl);
pub fn parse_thick_triple_object(object : &str) -> tt::OWL {
    let triple_json: SResult<tt::OWL> = serde_json::from_str(object); 

    match triple_json {
        Err(_) => tt::OWL::Named(String::from(object)),
        Ok(x) => x,
    } 
}

/// Given a string, return a JSON string
///
/// # Examples
/// let string = "\"test\""; //String encoding a JSON string
/// let json_string = ldtab_2_ofn::parser::parse_string(&s);
///
/// println!("{}", json_string);
/// 
/// # Panics
/// 
/// Panics if the input string is not a JSON string
pub fn parse_string(input : &str) -> String {

    let input: Value = serde_json::from_str(input).unwrap(); 

    match input {
        Value::String(x) => String::from(x),
        _ => panic!("Not a string"),
    }
} 

pub fn parse_json(t: &str) -> Value {
    let thick_triple: Value = serde_json::from_str(t).unwrap(); 
    thick_triple 
}
