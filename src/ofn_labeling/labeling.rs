//parse entire file and extract labelling triples 
//substitute entities with lables where possible
//
use std::fs::File;
use std::io::{prelude::*, BufReader};
use serde_json::{Value};
use std::collections::HashMap;

pub fn substitute(v : &Value, e2l : &HashMap<String, String>) -> String {
    let element : String = v.to_string();
    if e2l.contains_key(&element) {
        let inter  = e2l.get(&element).unwrap().to_string().replace("\"","");
        format!("\"'{}'\"", inter) //introduce single quotes
    } else {
        element
    } 
} 

//returns a map from entity names to their labels
pub fn extract_labeling(path : String) -> HashMap<String,String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut entity_2_label = HashMap::new();

    for line in reader.lines() {
        let content : String = line.unwrap();
        if is_labeling_triple(content.clone().as_str()){
            let (entity, label) : (String, String) = get_label_mapping(content.clone().as_str());
            entity_2_label.insert(entity, label);
            //println!("{}", content.clone());
        }
    } 

    entity_2_label 
}

fn is_labeling_triple(t: &str) -> bool {
    let thick_triple: Value = serde_json::from_str(t).unwrap();

    let predicate : String = thick_triple["predicate"].to_string();

     predicate == "\"rdfs:label\"" 
}

fn get_label_mapping(t: &str) -> (String, String) {

    let thick_triple: Value = serde_json::from_str(t).unwrap();

    let subj_helper : String  = thick_triple["subject"].to_string();
    //let subj : &str = subj_helper.as_str();

    let obj_helper : String  = thick_triple["object"].to_string();
    //let obj : &str = obj_helper.as_str();

    (subj_helper, obj_helper) 
}
