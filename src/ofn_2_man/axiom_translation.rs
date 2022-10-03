use serde_json::{Value};
use crate::ofn_2_man::class_translation as class_translation; 

pub fn translate_thin_triple(v : &Value) -> String {

    //translate OWL classes
    let subject = String::from(v[1].as_str().unwrap());
    let predicate = String::from(v[2].as_str().unwrap());
    let object = String::from(v[3].as_str().unwrap());

    //putting it all together
    let expression = format!("ThinTriple: {} {} {}", subject, predicate, object);
    expression
}

pub fn translate_subclass_of_axiom(v : &Value) -> String {

    //translate OWL classes
    let subclass = class_translation::translate(&v[1]);
    let superclass = class_translation::translate(&v[2]); 

    //putting it all together
    let expression = format!("Class: {} SubClassOf: {}", subclass, superclass);
    expression
}

pub fn translate_disjoint_classes_axiom(v : &Value) -> String { 
    let operands: Vec<String> = (&(v.as_array().unwrap())[1..]).into_iter().map(|x| class_translation::translate(&x)).collect(); 
    let merged = operands.join(", ");
    format!("DisjointClasses: {} ", merged) 
}

pub fn translate_disjoint_union_of_axiom(v : &Value) -> String {

    let lhs: String = class_translation::translate(&v[1]); 
    let operands: Vec<String> = (&(v.as_array().unwrap())[2..]).into_iter().map(|x| class_translation::translate(&x)).collect(); 
    let merged = operands.join(", ");
    format!("Class: {} DisjointUnionOf: {}", lhs, merged) 

}

// equivalent classes
// Note: Manchester Syntax does not encode Equivalence Classes as lists
// but as binary equivalence axiom chains - I don't see why we want to do that here
// so I don't
// (we treat them in the same manner as DisjointClasses since we don't care much about
// class frames to begin with)
pub fn translate_equivalent_classes_axiom(v : &Value) -> String {
    let operands: Vec<String> = (&(v.as_array().unwrap())[1..]).into_iter().map(|x| class_translation::translate(&x)).collect(); 
    let merged = operands.join(", ");
    format!("EquivalentClasses: {} ", merged) 
}
