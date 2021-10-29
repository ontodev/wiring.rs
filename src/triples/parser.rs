use std::fs::File;
use std::io::{prelude::*, BufReader};
use serde_json::{Value};

///Returns a vector of class expression axioms (serialised as JSON serde values)
///for a given ontology serialised as thick triples
///(would have preferred to use a set instead of a vector, but Value doesn't implement the trait Hash)
pub fn extract_class_expression_axioms(path : &str) -> Vec<Value> {

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut axioms = Vec::new();

    for line in reader.lines() {
        let content : String = line.unwrap(); 
        let thick_triple: Value = serde_json::from_str(content.as_str()).unwrap();

        if is_class_expression_axiom(&thick_triple){
            axioms.push(thick_triple);
        }
    }
    axioms 
}

fn is_class_expression_axiom(v: &Value) -> bool {

    let predicate : String = v["predicate"].to_string();

     match predicate.as_str() {
         "\"rdfs:subClassOf\""  => true,
         "\"owl:equivalentClass\""  => true,
         "\"owl:disjointWith\""  => true,
         "\"owl:AllDisjointClasses\""  => true,
         "\"owl:disjointUnionOf\""  => true,
         _ => false,
     } 
}
