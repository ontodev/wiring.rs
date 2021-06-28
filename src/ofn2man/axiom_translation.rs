use serde_json::{Value};
//use serde_json::json; 
//use crate::owl::typing as owl;
use crate::ofn2man::class_translation as class_translation; 

//use rand::Rng; 

//use crate::ofn2thick::owl as owl; 

pub fn translate_subclass_of_axiom(v : &Value) -> String {

    //translate OWL classes
    let subclass = class_translation::translate(&v[1]);
    let superclass = class_translation::translate(&v[2]); 

    //putting it all together
    let expression = format!("Class: {} SubClassOf: {}", subclass, superclass);
    expression
}

//pub fn translate_disjoint_classes_axiom(v : &Value) -> String {
//
//    // TODO make sure generated blank node ID's are unique
//    // this will be done as part of some post-processing
//    let mut rng = rand::thread_rng(); 
//    let blank_id: u8 = rng.gen();
//
//    let operands : owl::OWL = class_translation::translate_list(&(v.as_array().unwrap())[1..]); 
//    let operads_json = json!(operands); 
//    let operands_string = operads_json.to_string();
//
//    let expression = format!("{{\"subject\": _:gen{}, \"predicate\": \"owl:AllDisjointClasses\", \"object\": {{\"owl:members\": {}}}}}", blank_id, operands_string);
//    expression 
//}
//
//pub fn translate_disjoint_union_of_axiom(v : &Value) -> String {
//
//    let lhs = class_translation::translate(&v[1]);
//    let operands : owl::OWL = class_translation::translate_list(&(v.as_array().unwrap())[2..]); 
//
//    let lhs_json = json!(lhs);
//    let operads_json = json!(operands); 
//
//    let lhs_string =  lhs_json.to_string();
//    let operands_string = operads_json.to_string();
//
//    let expression = format!("{{\"subject\": {}, \"predicate\": \"owl:disjointUnionOf\", \"object\": {}}}", lhs_string, operands_string);
//    expression 
//}
//
//
////TODO::   equivalent classe  (we have a custom encoding for this and need a case distinction
////between binary axioms and n-ary axioms)
//pub fn translate_equivalent_classes_axiom(v : &Value) -> String {
//    let number_of_operands =  (v.as_array().unwrap())[1..].len();
//    if number_of_operands == 2 {
//        let lhs = class_translation::translate(&v[1]);
//        let rhs = class_translation::translate(&v[2]);
//
//        let lhs_json = json!(lhs);
//        let rhs_json = json!(rhs);
//
//        let lhs_string = lhs_json.to_string();
//        let rhs_string = rhs_json.to_string();
//
//        let expression = format!("{{\"subject\": {}, \"predicate\": \"owl:equivalentClass\", \"object\": {}}}", lhs_string, rhs_string);
//        expression 
//
//    } else {
//
//        let mut rng = rand::thread_rng(); 
//        let blank_id: u8 = rng.gen();
//
//        let operands : owl::OWL = class_translation::translate_list(&(v.as_array().unwrap())[1..]); 
//        let operads_json = json!(operands); 
//        let operands_string = operads_json.to_string();
//
//        let expression = format!("{{\"subject\": _:gen{}, \"predicate\": \"owl:equivalentClass\", \"object\": {}}}", blank_id, operands_string);
//        expression 
//    }
//}
