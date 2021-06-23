use serde_json::{Value};
use crate::ofn2thick::class_translation as class_translation; 
use crate::ofn2thick::axiom_translation as axiom_translation; 
use crate::owl::typing as owl;

pub fn parse_ofn(t: &str) -> String {
    //deserialise JSON as a (serde) Value
    let thick_triple: Value = serde_json::from_str(t).unwrap(); 
    //start the translation process
    let out = translate_triple(&thick_triple); 
    out 
}

pub fn translate_triple(v : &Value) -> String {

    let owl_operator: String = v[0].to_string();

     let res : String = match owl_operator.as_str() {
         "\"SubClassOf\"" => axiom_translation::translate_subclass_of_axiom(v),
         _ => v.to_string(),//return named entity TODO: this should be an error
     };

     res
}

//Note that (thick) triples are not OWL
pub fn translate_owl(v : &Value) -> owl::OWL {

    let owl_operator: String = v[0].to_string();

     match owl_operator.as_str() {
         "\"ObjectSomeValuesFrom\"" => class_translation::translate_some_values_from(v), 
         "\"ObjectAllValuesFrom\"" => class_translation::translate_all_values_from(v), 
         "\"ObjectHasValue\"" => class_translation::translate_has_value(v), 
         "\"ObjectMinCardinality\"" => class_translation::translate_min_cardinality(v), 
         "\"ObjectMinQualifiedCardinality\"" => class_translation::translate_min_qualified_cardinality(v), 
         "\"ObjectMaxCardinality\"" => class_translation::translate_max_cardinality(v), 
         "\"ObjectMaxQualifiedCardinality\"" => class_translation::translate_max_qualified_cardinality(v), 
         "\"ObjectExactCardinality\"" => class_translation::translate_exact_cardinality(v), 
         "\"ObjectExactQualifiedCardinality\"" => class_translation::translate_exact_qualified_cardinality(v), 
         "\"ObjectHasSelf\"" => class_translation::translate_has_self(v), 
         "\"ObjectIntersectionOf\"" => class_translation::translate_intersection_of(v), 
         "\"ObjectUnionOf\"" => class_translation::translate_union_of(v), 
         "\"ObjectOneOf\"" => class_translation::translate_one_of(v), 
         "\"ObjectComplementOf\"" => class_translation::translate_complement_of(v), 
         "\"ObjectInverseOf\"" => class_translation::translate_inverse_of(v), 
         _ => owl::OWL::Named(v.to_string().replace("\"","")),//return named entity (without quotes)
         //TODO: this way of removing quotes is somewhat crude
     }
}

