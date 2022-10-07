use serde_json::{Value, Result as SResult};
use crate::owl::thick_triple as tt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

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

/// Given a path to a file (containing LDTab ThickTriples),
/// return a vector of ThickTriples (serialised as JSON serde values).
pub fn load_thick_triples(path : &str) -> Vec<Value> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut triples = Vec::new();

    for line in reader.lines() {
        let content : String = line.unwrap(); 
        let thick_triple: Value = serde_json::from_str(content.as_str()).unwrap();

        triples.push(thick_triple);

    }
    triples
} 

/// Given a path to a file (containing LDTab ThickTriples),
/// return a vector of ThinTriples, i.e., ThickTriples (serialised as JSON serde values)
/// that do not contain nested blank node structures.
pub fn load_thin_triples(path : &str) -> Vec<Value> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut triples = Vec::new();

    for line in reader.lines() {
        let content : String = line.unwrap(); 
        let thick_triple: Value = serde_json::from_str(content.as_str()).unwrap();

        if !is_thick_triple(&thick_triple){//TODO: test that all three things are strings
            triples.push(thick_triple);
        }
    }
    triples 
}

fn is_thick_triple(v: &Value) -> bool {

    let nesting = v["subject"].is_object() | v["predicate"].is_object() | v["object"].is_object();
    if nesting  {
        true
    } else {
        v["predicate"].as_str().unwrap().eq("rdfs:subClassOf")
    } 
}


/// Given a path to a file (containing LDTab ThickTriples),
/// return a vector of ThickTriples (serialised as JSON serde values)
/// that encode class expression axioms.
pub fn load_class_expression_axioms(path : &str) -> Vec<Value> { 
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut axioms = Vec::new();

    for line in reader.lines() {
        let content : String = line.unwrap(); 
        let thick_triple: Value = serde_json::from_str(content.as_str()).unwrap();

        //filter for class expressions
        if is_class_expression_axiom(&thick_triple){
            axioms.push(thick_triple);
        }
    }
    axioms 
}

fn is_class_expression_axiom(v: &Value) -> bool {

    let predicate = v["predicate"].as_str();

     match predicate {
         Some("rdfs:subClassOf")  => true,
         Some("owl:equivalentClass")  => true,
         Some("owl:disjointWith")  => true,
         Some("owl:AllDisjointClasses")  => true,
         Some("owl:disjointUnionOf")  => true,
         Some(_) => false,
         None => false,
     } 
} 
