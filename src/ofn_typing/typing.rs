use std::fs::File;
use std::io::{prelude::*, BufReader};
use serde_json::{Value};
use std::collections::HashMap;
use std::collections::HashSet;

//returns a map from entity names to their types
//TODO: think about whether you want this mapping to work on Values too
pub fn extract_typing(path : &str) -> HashMap<String,HashSet<String>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut entity_2_type = HashMap::new();

    for line in reader.lines() {
        //we read the input from a file as a string
        let content : String = line.unwrap();
        if is_typing_triple(content.clone().as_str()){
            let (entity, rdf_type) : (String, String) = get_type_mapping(content.clone().as_str());
            if !entity_2_type.contains_key(&entity) {
                entity_2_type.insert(entity.clone(), HashSet::new()); 
            } 
            entity_2_type.get_mut(&entity).unwrap().insert(rdf_type); 
        }
    } 
    entity_2_type 
}

fn is_typing_triple(t: &str) -> bool {
    let thick_triple: Value = serde_json::from_str(t).unwrap(); 
    //let predicate : String = thick_triple["predicate"].to_string(); 
    //predicate == "\"rdf:type\"" 
    let predicate : &str = thick_triple["predicate"].as_str().unwrap();
    predicate.eq("rdf:type")
}

fn get_type_mapping(t: &str) -> (String, String) {

    let thick_triple: Value = serde_json::from_str(t).unwrap(); 

    //let subj_helper : String  = thick_triple["subject"].to_string(); 
    //let obj_helper : String  = thick_triple["object"].to_string();
    //(subj_helper, obj_helper) 

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
