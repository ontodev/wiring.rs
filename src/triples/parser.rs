use std::fs::File;
use std::io::{prelude::*, BufReader};
use serde_json::{Value};

/// Given a path to a file (containing LDTab ThickTriples),
/// return a vector of ThickTriples (serialised as JSON serde values).
pub fn get_thick_triples(path : &str) -> Vec<Value> {
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
/// return a vector of ThickTriples (serialised as JSON serde values)
/// that encode class expression axioms.
pub fn extract_class_expression_axioms(path : &str) -> Vec<Value> { 
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

/// Given a path to a file (containing LDTab ThickTriples),
/// return a vector of ThinTriples, i.e., ThickTriples (serialised as JSON serde values)
/// that do not contain nested blank node structures.
pub fn extract_thin_triples(path : &str) -> Vec<Value> {
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
