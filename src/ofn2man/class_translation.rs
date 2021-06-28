use serde_json::{Value};
use crate::ofn2man::property_translation as property_translation;

pub fn translate(v : &Value) -> String { 
    let owl_operator: String = v[0].to_string();

     match owl_operator.as_str() {
         "\"ObjectSomeValuesFrom\"" => translate_some_values_from(v), 
         "\"ObjectAllValuesFrom\"" => translate_all_values_from(v), 
         "\"ObjectHasValue\"" => translate_has_value(v), 
         "\"ObjectMinCardinality\"" => translate_min_cardinality(v), 
         "\"ObjectMinQualifiedCardinality\"" => translate_min_qualified_cardinality(v), 
         "\"ObjectMaxCardinality\"" => translate_max_cardinality(v), 
         "\"ObjectMaxQualifiedCardinality\"" => translate_max_qualified_cardinality(v), 
         "\"ObjectExactCardinality\"" => translate_exact_cardinality(v), 
         "\"ObjectExactQualifiedCardinality\"" => translate_exact_qualified_cardinality(v), 
         "\"ObjectHasSelf\"" => translate_has_self(v), 
         "\"ObjectIntersectionOf\"" => translate_intersection_of(v), 
         "\"ObjectUnionOf\"" => translate_union_of(v), 
         "\"ObjectOneOf\"" => translate_one_of(v), 
         "\"ObjectComplementOf\"" => translate_complement_of(v), 
         "\"ObjectInverseOf\"" => property_translation::translate_inverse_of(v), 
         _ => v.to_string().replace("\"",""),//return named entity (without quotes)
         //TODO: this way of removing quotes is somewhat crude
     }
} 

pub fn is_named_class(v : &Value) -> bool { 
    let owl_operator: String = v[0].to_string();

     match owl_operator.as_str() {
         "\"ObjectSomeValuesFrom\"" => false, 
         "\"ObjectAllValuesFrom\"" => false, 
         "\"ObjectHasValue\"" => false, 
         "\"ObjectMinCardinality\"" => false, 
         "\"ObjectMinQualifiedCardinality\"" => false, 
         "\"ObjectMaxCardinality\"" => false, 
         "\"ObjectMaxQualifiedCardinality\"" => false, 
         "\"ObjectExactCardinality\"" => false, 
         "\"ObjectExactQualifiedCardinality\"" => false, 
         "\"ObjectHasSelf\"" => false, 
         "\"ObjectIntersectionOf\"" => false, 
         "\"ObjectUnionOf\"" => false, 
         "\"ObjectOneOf\"" => false, 
         "\"ObjectComplementOf\"" => false, 
         _ => true,
     } 
}

pub fn translate_some_values_from(v : &Value) -> String { 
    //let op: &Value = &v[0];//don't need OWL constructor 
    let property: String = translate(&v[1]); 
    let filler: String = translate(&v[2]); 

    if is_named_class(&v[2]) {
        format!("{} some {}", property, filler)
    } else { 
        format!("{} some ({})", property, filler)
    } 
} 

pub fn translate_all_values_from(v : &Value) -> String { 
    let property: String = translate(&v[1]); 
    let filler: String = translate(&v[2]); 

    if is_named_class(&v[2]) {
        format!("{} only {}", property, filler)
    } else { 
        format!("{} only ({})", property, filler)
    } 
} 

pub fn translate_has_value(v : &Value) -> String { 
    let property: String = translate(&v[1]); 
    let filler: String = translate(&v[2]); 

    format!("{} value {}", property, filler) 
} 

pub fn translate_has_self(v : &Value) -> String {

    let property: String = translate(&v[1]); 
    format!("{} some Self", property) 
} 

pub fn translate_min_cardinality(v : &Value) -> String { 
    let property: String = translate(&v[1]); 
    let cardinality: String = translate(&v[2]); 

    format!("{} min {} owl:Thing", property, cardinality) 
} 

pub fn translate_min_qualified_cardinality(v : &Value) -> String { 
    let property: String = translate(&v[1]); 
    let cardinality: String = translate(&v[2]); 
    let filler: String = translate(&v[3]); 

    if is_named_class(&v[2]) {
        format!("{} min {} {}", cardinality, property, filler)
    } else {
        format!("{} min {} ({})", cardinality, property, filler)
    }
}

pub fn translate_max_cardinality(v : &Value) -> String { 
    let cardinality: String = translate(&v[1]); 
    let property: String = translate(&v[2]); 

    format!("{} max {} owl:Thing", property, cardinality) 
} 

pub fn translate_max_qualified_cardinality(v : &Value) -> String { 
    let property: String = translate(&v[1]); 
    let cardinality: String = translate(&v[2]); 
    let filler: String = translate(&v[3]); 

    if is_named_class(&v[2]) {
        format!("{} max {} {}", cardinality, property, filler)
    } else {
        format!("{} max {} ({})", cardinality, property, filler)
    }
} 

pub fn translate_exact_cardinality(v : &Value) -> String { 
    let property: String = translate(&v[1]); 
    let cardinality: String = translate(&v[2]); 

    format!("{} exactly {} owl:Thing", cardinality, property) 
} 

pub fn translate_exact_qualified_cardinality(v : &Value) -> String { 
    let property: String = translate(&v[1]); 
    let cardinality: String = translate(&v[2]); 
    let filler: String = translate(&v[3]); 

    if is_named_class(&v[2]) {
        format!("{} exactly {} {}", cardinality, property, filler)
    } else {
        format!("{} exactly {} ({})", cardinality, property, filler)
    }
} 

pub fn translate_intersection_of(v : &Value) -> String {
    let operands: Vec<String> = (&(v.as_array().unwrap())[1..]).into_iter().map(|x| translate(&x)).collect(); 
    let merged = operands.join(" and ");
    format!("( {} )", merged) 
} 

pub fn translate_union_of(v : &Value) -> String {
    let operands: Vec<String> = (&(v.as_array().unwrap())[1..]).into_iter().map(|x| translate(&x)).collect(); 
    let merged = operands.join(" or ");
    format!("( {} )", merged) 
} 

pub fn translate_one_of(v : &Value) -> String {
    let operands: Vec<String> = (&(v.as_array().unwrap())[1..]).into_iter().map(|x| translate(&x)).collect(); 
    let merged = operands.join(" , ");
    format!("{{ {} }}", merged) 
} 

pub fn translate_complement_of(v : &Value) -> String { 
    let argument: String = translate(&v[1]); 
    format!("not ( {} )", argument) 
} 
