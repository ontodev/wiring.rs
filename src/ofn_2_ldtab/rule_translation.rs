use crate::ofn_2_ldtab::util;
use crate::ofn_2_ldtab::property_translation;
use crate::ofn_2_ldtab::class_translation;
use serde_json::json;
use serde_json::{Map, Value};


pub fn translate(v: &Value) -> Value {
    match v[0].as_str() {
        Some("Body") => translate_body(v),
        Some("Head") => translate_head(v),
        Some("ObjectPropertyAtom") => translate_object_property_atom(v),
        Some(_) => json!("TODO"),
        //None => owl::OWL::Named(String::from(v.as_str().unwrap())),

        //Some("Variable") => axiom_translation::translate_ontology(v),
        //Some("SameIndividualAtom") => axiom_translation::translate_ontology(v),
        //Some("DifferentIndividualsAtom") => axiom_translation::translate_ontology(v),
        //Some("DataRangeAtom") => axiom_translation::translate_ontology(v),
        //Some("ClassAtom") => axiom_translation::translate_ontology(v),
        //Some("BuiltInAtom") => axiom_translation::translate_ontology(v),
        //
        None => translate_named_entity(&v),
    }
}

pub fn translate_named_entity(v: &Value) -> Value {
    let o: String = String::from(v.as_str().unwrap());
    json!(o)
}

pub fn get_object(v: &Value) -> Value {
    let o: Value = translate(&v);
    let d: String = String::from(util::translate_datatype(&v).as_str().unwrap());

    json!({"object" : o,
           "datatype" : d})
}

pub fn translate_object_property_atom(v: &Value) -> Value {
    let type_o = get_object(&json!("swrl:IndividualPropertyAtom"));
    let property_o = get_object(&v[1]);
    let arg1_o = get_object(&v[2]);
    let arg2_o = get_object(&v[3]);
    json!( {"datatype" : "_JSON",
            "object": vec![json!({"rdf:type" : vec![type_o],
                            "swrl:propertyPredicate" : vec![property_o],
                            "swrl:argument1" : vec![arg1_o],
                            "swrl:argument2" : vec![arg2_o]})]})
}

pub fn translate_body(v: &Value) -> Value {

    let array = v.as_array().unwrap();
    let args = array[1..].to_vec();

    let mut map = Map::new();

    for arg in args.iter().rev() {
        if map.is_empty() { //first element
            let rest_o: Value = get_object(&json!("rdf:nil"));
            let type_o: Value = get_object(&json!("swrl:AtomList"));
            map.insert(String::from("rdf:rest"),json!(vec![rest_o]));
            map.insert(String::from("rdf:type"),json!(vec![type_o]));
            map.insert(String::from("rdf:first"),json!(vec![translate(arg)]));
        } else {
            let inner = map.clone();
            map.clear();
            let type_o: Value = get_object(&json!("swrl:AtomList"));
            map.insert(String::from("rdf:type"),json!(vec![type_o]));
            map.insert(String::from("rdf:first"),json!(vec![translate(arg)]));
            map.insert(String::from("rdf:rest"),json!(vec![json!(inner)]));
        }
    }

    json!({"swrl:body" : vec![map]})
}

pub fn translate_head(v: &Value) -> Value {

    let array = v.as_array().unwrap();
    let args = array[1..].to_vec();

    let mut map = Map::new();

    for arg in args.iter().rev() {
        if map.is_empty() { //first element
            let rest_o: Value = get_object(&json!("rdf:nil"));
            let type_o: Value = get_object(&json!("swrl:AtomList"));
            map.insert(String::from("rdf:rest"),json!(vec![rest_o]));
            map.insert(String::from("rdf:type"),json!(vec![type_o]));
            map.insert(String::from("rdf:first"),json!(vec![translate(arg)]));
        } else {
            let inner = map.clone();
            map.clear();
            let type_o: Value = get_object(&json!("swrl:AtomList"));
            map.insert(String::from("rdf:type"),json!(vec![type_o]));
            map.insert(String::from("rdf:first"),json!(vec![translate(arg)]));
            map.insert(String::from("rdf:rest"),json!(vec![json!(inner)]));
        }
    }

    json!({"swrl:head" : vec![map]})
}
