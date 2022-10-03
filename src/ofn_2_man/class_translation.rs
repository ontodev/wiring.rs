use serde_json::{Value};
use crate::ofn_2_man::property_translation as property_translation;

pub fn translate(v : &Value) -> String { 
     match v[0].as_str() {
         Some("SomeValuesFrom") => translate_some_values_from(v), 
         Some("AllValuesFrom") => translate_all_values_from(v), 
         Some("HasValue") => translate_has_value(v), 
         Some("MinCardinality") => translate_min_cardinality(v), 
         Some("MinQualifiedCardinality") => translate_min_qualified_cardinality(v), 
         Some("MaxCardinality") => translate_max_cardinality(v), 
         Some("MaxQualifiedCardinality") => translate_max_qualified_cardinality(v), 
         Some("ExactCardinality") => translate_exact_cardinality(v), 
         Some("ExactQualifiedCardinality") => translate_exact_qualified_cardinality(v), 
         Some("HasSelf") => translate_has_self(v), 
         Some("IntersectionOf") => translate_intersection_of(v), 
         Some("UnionOf") => translate_union_of(v), 
         Some("OneOf") => translate_one_of(v), 
         Some("ComplementOf") => translate_complement_of(v), 
         Some("InverseOf") => property_translation::translate_inverse_of(v),  //this is necessarily an ObjectInverseOf

         Some("ObjectSomeValuesFrom") => translate_some_values_from(v), 
         Some("ObjectAllValuesFrom") => translate_all_values_from(v), 
         Some("ObjectHasValue") => translate_has_value(v), 
         Some("ObjectMinCardinality") => translate_min_cardinality(v), 
         Some("ObjectMinQualifiedCardinality") => translate_min_qualified_cardinality(v), 
         Some("ObjectMaxCardinality") => translate_max_cardinality(v), 
         Some("ObjectMaxQualifiedCardinality") => translate_max_qualified_cardinality(v), 
         Some("ObjectExactCardinality") => translate_exact_cardinality(v), 
         Some("ObjectExactQualifiedCardinality") => translate_exact_qualified_cardinality(v), 
         Some("ObjectHasSelf") => translate_has_self(v), 
         Some("ObjectIntersectionOf") => translate_intersection_of(v), 
         Some("ObjectUnionOf") => translate_union_of(v), 
         Some("ObjectOneOf") => translate_one_of(v), 
         Some("ObjectComplementOf") => translate_complement_of(v), 
         Some("ObjectInverseOf") => property_translation::translate_inverse_of(v), 

         Some("DataSomeValuesFrom") => translate_some_values_from(v), 
         Some("DataAllValuesFrom") => translate_all_values_from(v), 
         Some("DataHasValue") => translate_has_value(v), 
         Some("DataMinCardinality") => translate_min_cardinality(v), 
         Some("DataMinQualifiedCardinality") => translate_min_qualified_cardinality(v), 
         Some("DataMaxCardinality") => translate_max_cardinality(v), 
         Some("DataMaxQualifiedCardinality") => translate_max_qualified_cardinality(v), 
         Some("DataExactCardinality") => translate_exact_cardinality(v), 
         Some("DataExactQualifiedCardinality") => translate_exact_qualified_cardinality(v), 
         Some("DataHasSelf") => translate_has_self(v), 
         Some("DataIntersectionOf") => translate_intersection_of(v), 
         Some("DataUnionOf") => translate_union_of(v), 
         Some("DataOneOf") => translate_one_of(v), 
         Some("DataComplementOf") => translate_complement_of(v), 

         Some(_) => panic!(),
         None => String::from(v.as_str().unwrap()),
     }
} 

pub fn is_named_class(v : &Value) -> bool { 
    //let owl_operator: String = v[0].to_string();

     //match owl_operator.as_str() {

    match v[0].as_str() {
         Some("SomeValuesFrom") => false, 
         Some("AllValuesFrom") => false, 
         Some("HasValue") => false, 
         Some("MinCardinality") => false, 
         Some("MinQualifiedCardinality") => false, 
         Some("MaxCardinality") => false, 
         Some("MaxQualifiedCardinality") => false, 
         Some("ExactCardinality") => false, 
         Some("ExactQualifiedCardinality") => false, 
         Some("HasSelf") => false, 
         Some("IntersectionOf") => false, 
         Some("UnionOf") => false, 
         Some("OneOf") => false, 
         Some("ComplementOf") => false, 

         Some("ObjectSomeValuesFrom") => false, 
         Some("ObjectAllValuesFrom") => false, 
         Some("ObjectHasValue") => false, 
         Some("ObjectMinCardinality") => false, 
         Some("ObjectMinQualifiedCardinality") => false, 
         Some("ObjectMaxCardinality") => false, 
         Some("ObjectMaxQualifiedCardinality") => false, 
         Some("ObjectExactCardinality") => false, 
         Some("ObjectExactQualifiedCardinality") => false, 
         Some("ObjectHasSelf") => false, 
         Some("ObjectIntersectionOf") => false, 
         Some("ObjectUnionOf") => false, 
         Some("ObjectOneOf") => false, 
         Some("ObjectComplementOf") => false, 

         Some("DataSomeValuesFrom") => false, 
         Some("DataAllValuesFrom") => false, 
         Some("DataHasValue") => false, 
         Some("DataMinCardinality") => false, 
         Some("DataMinQualifiedCardinality") => false, 
         Some("DataMaxCardinality") => false, 
         Some("DataMaxQualifiedCardinality") => false, 
         Some("DataExactCardinality") => false, 
         Some("DataExactQualifiedCardinality") => false, 
         Some("DataHasSelf") => false, 
         Some("DataIntersectionOf") => false, 
         Some("DataUnionOf") => false, 
         Some("DataOneOf") => false, 
         Some("DataComplementOf") => false, 
         Some(_) => panic!(),
         None => true,
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

pub fn parenthesis(v : &Value) -> String {
    if is_named_class(v) { 
        translate(v)
    } else {
        format!("({})", translate(v))
    }
}

pub fn translate_intersection_of(v : &Value) -> String {
    let operands: Vec<String> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               //.map(|x| translate(&x))
                                               .map(|x| parenthesis(&x))
                                               .collect(); 
    let merged = operands.join(" and ");
    format!("{}", merged) 
} 

pub fn translate_union_of(v : &Value) -> String {
    let operands: Vec<String> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               //map(|x| translate(&x)).
                                               .map(|x| parenthesis(&x))
                                               .collect(); 
    let merged = operands.join(" or ");
    format!("{}", merged) 
} 

pub fn translate_one_of(v : &Value) -> String {
    let operands: Vec<String> = (&(v.as_array().unwrap())[1..])
                                               .into_iter()
                                               .map(|x| translate(&x))
                                               .collect(); 
    let merged = operands.join(", ");
    format!("{{{}}}", merged) 
} 

pub fn translate_complement_of(v : &Value) -> String { 
    let argument: String = translate(&v[1]); 
    format!("not ( {} )", argument) 
} 
