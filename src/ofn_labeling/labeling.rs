//parse entire file and extract labelling triples 
//substitute entities with lables where possible
//
use std::fs::File;
use std::io::{prelude::*, BufReader};
use serde_json::{Value};
use std::collections::HashMap;


pub fn substitute(v : &Value, e2l : &HashMap<String, String>) -> Value {
    //substitute is only called when v is a string
    //let element : String = v.to_string();

    let element = match v.as_str() {
        Some(s) => String::from(s),
        None => v.to_string(), 
    };

    if e2l.contains_key(&element) {
        let label = e2l.get(&element).unwrap().as_str(); 
        let single_quote = format!("'{}'", &label); //introduce single quotes
        Value::String(String::from(single_quote)) 
    } else {
        //return input
        let iri = element.as_str();
        Value::String(String::from(iri)) 
    }
}

//returns a map from entity names to their labels
//TODO: this currently assumes that there is exactly one label for each term 
pub fn extract_labeling(path : &str) -> HashMap<String,String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut entity_2_label = HashMap::new();

    for line in reader.lines() {
        let content : String = line.unwrap();
        if is_labeling_triple(content.clone().as_str()){
            let (entity, label) : (String, String) = get_label_mapping(content.clone().as_str());
            entity_2_label.insert(entity, label);
        }
    } 

    entity_2_label 
}

fn is_labeling_triple(t: &str) -> bool {
    let thick_triple: Value = serde_json::from_str(t).unwrap();

    let predicate : &str = thick_triple["predicate"].as_str().unwrap();

    predicate.eq("rdfs:label")
}

fn get_label_mapping(t: &str) -> (String, String) {

    let thick_triple: Value = serde_json::from_str(t).unwrap();

    let subject = match thick_triple["subject"].as_str() {
        Some(s) => String::from(s),
        None => thick_triple["subject"].to_string(), 
    };

    let object = match thick_triple["object"].as_str() {
        Some(s) => String::from(s),
        None => thick_triple["object"].to_string(), 
    };

    (subject, object) 
}
