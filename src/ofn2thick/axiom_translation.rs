use serde_json::{Value};
use serde_json::json; 
//use crate::ofn2thick::class_translation as class_translation; 
use crate::ofn2thick::owl as owl; 

pub fn translate_subclass_of_axiom(v : &Value) -> String {

    //translate OWL classes
    let subclass = owl::translate_owl(&v[1]);
    let superclass = owl::translate_owl(&v[2]); 

    //serialise to JSON
    let sub_json = json!(subclass);
    let sup_json = json!(superclass);

    //convert to string
    let sub_string = sub_json.to_string();
    let sup_string = sup_json.to_string();

    //putting it all together
    let expression = format!("{{\"subject\": {}, \"predicate\": \"rdfs:subClassOf\", \"object\": {}}}", sub_string, sup_string);
    expression
}
