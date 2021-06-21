use serde_json::{Value};
//use serde::{Deserialize, Serialize};

use crate::axiom_translation as axiom_translation; 

//#[derive(Debug,Serialize, Deserialize)]
//pub struct ThickTriple {
//    subject: String,
//    predicate: String,
//    object: String, //This causes an issue because the value of objects cannot be parsed as a
//    string
//}

pub fn parse_tiple(t: &str) -> String {

    //println!("{}", t);
    let thick_triple: Value = serde_json::from_str(t).unwrap();
    //println!("{}", thick_triple["subject"]);

    //TODO: I cannot chain to_string() and as_str() - why?
    let subj_helper : String  = thick_triple["subject"].to_string();
    let subj : &str = subj_helper.as_str();

    let predicate : String = thick_triple["predicate"].to_string();

    let obj_helper : String  = thick_triple["object"].to_string();
    let obj : &str = obj_helper.as_str();

    match predicate.as_str() {
        "\"rdfs:subClassOf\"" => axiom_translation::translate_subclass_of_axiom(subj, obj),
        "\"owl:equivalentClass\"" => axiom_translation::translate_equivalent_class(subj, obj),
        "\"owl:AllDisjointClasses\"" => {
                let members_helper : String = thick_triple["object"]["owl:members"].to_string();
                let members : &str = members_helper.as_str(); 
                axiom_translation::translate_disjoint_classes(members) 
            }, 
        "\"owl:disjointUnionOf\"" => axiom_translation::translate_disjoint_union(subj, obj),
        _ => String::from("Fail"),
    } 
}
