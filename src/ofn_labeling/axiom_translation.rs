use serde_json::{Value};
use crate::ofn_labeling::class_translation as class_translation; //TODO: class translation
use std::collections::HashMap;



//todo return expression
pub fn translate_subclass_of_axiom(v : &Value, m : &HashMap<String,String>) -> String {

    //translate OWL classes
    let subclass : String = class_translation::translate(&v[1], m);
    let superclass : String = class_translation::translate(&v[2], m); 

    let expression = format!("[\"SubClassOf\",{},{}]", subclass, superclass); 
    expression
}

pub fn translate_disjoint_classes_axiom(v : &Value, m : &HashMap<String,String>) -> String {

    let operands : String = class_translation::translate_list(&(v.as_array().unwrap())[1..], m); 
    let expression = format!("[\"DisjointClasses\",{}]", operands);
    expression
}

pub fn translate_disjoint_union_of_axiom(v : &Value, m : &HashMap<String,String>) -> String {

    let lhs = class_translation::translate(&v[1], m);
    let operands : String = class_translation::translate_list(&(v.as_array().unwrap())[2..], m); 
    let expression = format!("[\"DisjointUnionOf\",{},{}]", lhs, operands);
    expression 
}


//TODO::   equivalent classe  (we have a custom encoding for this and need a case distinction
//between binary axioms and n-ary axioms)
pub fn translate_equivalent_classes_axiom(v : &Value, m : &HashMap<String,String>) -> String {
    let number_of_operands =  (v.as_array().unwrap())[1..].len();
    if number_of_operands == 2 {
        let lhs = class_translation::translate(&v[1],m);
        let rhs = class_translation::translate(&v[2],m); 
        let expression = format!("[\"EquivalentClasses\",{},{}]", lhs, rhs);
        expression 
    } else {

        let operands : String = class_translation::translate_list(&(v.as_array().unwrap())[1..],m); 
        let expression = format!("[\"EquivalentClasses\",{}]", operands);
        expression 
    }
}
