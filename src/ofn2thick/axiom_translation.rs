use serde_json::{Value};
use serde_json::json; 
use crate::owl::typing as owl;
use crate::ofn2thick::class_translation as class_translation; 
use rand::Rng; 

pub fn translate_subclass_of_axiom(v : &Value) -> Value {

    //translate OWL classes
    //NB: this returns owl:OWL
    let subclass = class_translation::translate(&v[1]);
    let superclass = class_translation::translate(&v[2]); 

    let triple = json!({"subject":subclass,
                     "predicate":"rdfs:subClassOf", 
                     "object":superclass}); 
    triple 
}

pub fn translate_disjoint_classes_axiom(v : &Value) -> Value {

    // TODO sort json strings + generate hashes as blank node IDs
    let mut rng = rand::thread_rng(); 
    let blank_id: u8 = rng.gen();
    let blank_node = format!("_:gen{}",blank_id);

    let operands : owl::OWL = class_translation::translate_list(&(v.as_array().unwrap())[1..]); 

    let triple = json!({"subject":blank_node,
                        "predicate":"owl:AllDisjointClasses",
                        "object": {"owl:members":operands}}); 
    triple
}

pub fn translate_disjoint_union_of_axiom(v : &Value) -> Value {

    let lhs = class_translation::translate(&v[1]);
    let operands : owl::OWL = class_translation::translate_list(&(v.as_array().unwrap())[2..]); 

    let triple = json!({"subject":lhs,
                        "predicate":"owl:disjointUnionOf",
                        "object":operands});
    triple
}


//TODO:: equivalent classe  (we have a custom encoding for this and need a case distinction
//between binary axioms and n-ary axioms)
pub fn translate_equivalent_classes_axiom(v : &Value) -> Value {
    let number_of_operands =  (v.as_array().unwrap())[1..].len();
    if number_of_operands == 2 {
        let lhs = class_translation::translate(&v[1]);
        let rhs = class_translation::translate(&v[2]);

        let triple = json!({"subject":lhs,
                            "predicate":"owl:equivalentClass",
                            "object":rhs});
        triple 
    } else {

        let mut rng = rand::thread_rng(); 
        let blank_id: u8 = rng.gen();
        let blank_node = format!("_:gen{}",blank_id);

        let operands : owl::OWL = class_translation::translate_list(&(v.as_array().unwrap())[1..]); 
        let triple = json!({"subject":blank_node,
                            "predicate":"owl:equivalentClass",
                            "object":operands});
        triple 
    }
}
