use serde_json::{Value};
use serde_json::json;
use crate::ofn2ldtab::property_translation as property_translation;
use crate::ofn2ldtab::util as util;

pub fn translate(v : &Value) -> Value {

    match v[0].as_str() {
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
        Some("InverseOf") => property_translation::translate_inverse_of(v), 


        Some(_) => panic!(),
        //None => owl::OWL::Named(String::from(v.as_str().unwrap())),
        None => translate_named_entity(&v),
    }
}

pub fn translate_named_entity(v: &Value) -> Value {
        let o: String = String::from(v.as_str().unwrap());
        json!(o) 
}

pub fn get_object(v : &Value) -> Value {
    let o: Value = translate(&v);
    let d: String = String::from(util::translate_datatype(&v).as_str().unwrap());

    json!({"object" : o,
           "datatype" : d}) 
}

pub fn translate_some_values_from(v : &Value) -> Value {

    let property_o : Value = get_object(&v[1]);
    let filler_o : Value = get_object(&v[2]);
    let type_o : Value = get_object(&json!("owl:Restriction"));

    //build struct
    let res : Value = json!({"rdf:type" : vec![type_o],
                             "owl:onProperty" : vec![property_o],
                             "owl:someValuesFrom" : vec![filler_o]});
        
    //return type
    res 
} 


pub fn translate_all_values_from(v : &Value) -> Value {

    let property_o : Value = get_object(&v[1]);
    let filler_o : Value = get_object(&v[2]);
    let type_o : Value = get_object(&json!("owl:Restriction"));

    json!({"rdf:type" : vec![type_o],
           "owl:onProperty" : vec![property_o],
           "owl:allValuesFrom": vec![filler_o]}) 
} 

pub fn translate_has_value(v : &Value) -> Value {

    let property_o : Value = get_object(&v[1]);
    let filler_o : Value = get_object(&v[2]);
    let type_o : Value = get_object(&json!("owl:Restriction"));

    json!({"rdf:type" : vec![type_o],
           "owl:onProperty" : vec![property_o],
           "owl:hasValue" : vec![filler_o]})

} 

pub fn translate_has_self(v : &Value) -> Value {

    let property_o : Value = get_object(&v[1]);
    let has_self_o : Value = get_object(&json!("true^^xsd:boolean"));
    let type_o : Value = get_object(&json!("owl:Restriction"));

    json!({"rdf:type" : vec![type_o],
           "owl:onProperty" : vec![property_o],
           "owl:hasSelf" : vec![has_self_o]})
} 


pub fn translate_min_cardinality(v : &Value) -> Value {

    let property_o : Value = get_object(&v[1]);
    let cardinality_o : Value = get_object(&v[2]);
    let type_o : Value = get_object(&json!("owl:Restriction"));

    json!({"rdf:type" : vec![type_o],
           "owl:onProperty" : vec![property_o],
           "owl:minCardinality" : vec![cardinality_o]})
} 

pub fn translate_min_qualified_cardinality(v : &Value) -> Value {

    let property_o : Value = get_object(&v[1]);
    let cardinality_o : Value = get_object(&v[2]);
    let filler_o : Value = get_object(&v[3]);
    let type_o : Value = get_object(&json!("owl:Restriction"));

    json!({"rdf:type" : vec![type_o],
           "owl:onProperty" : vec![property_o],
           "owl:minQualifiedCardinality" : vec![cardinality_o],
           "owl:onClass" : vec![filler_o] }) 
} 

pub fn translate_max_cardinality(v : &Value) -> Value {

    let property_o : Value = get_object(&v[1]);
    let cardinality_o : Value = get_object(&v[2]);
    let type_o : Value = get_object(&json!("owl:Restriction"));

    json!({"rdf:type" : vec![type_o],
           "owl:onProperty" : vec![property_o],
           "owl:maxCardinality" : vec![cardinality_o]}) 
} 

pub fn translate_max_qualified_cardinality(v : &Value) -> Value {

    let property_o : Value = get_object(&v[1]);
    let cardinality_o : Value = get_object(&v[2]);
    let filler_o : Value = get_object(&v[3]);
    let type_o : Value = get_object(&json!("owl:Restriction"));

    json!({"rdf:type" : vec![type_o],
           "owl:onProperty" : vec![property_o],
           "owl:maxQualifiedCardinality" : vec![cardinality_o],
           "owl:onClass" : vec![filler_o] })
} 

pub fn translate_exact_cardinality(v : &Value) -> Value {

    let property_o : Value = get_object(&v[1]);
    let cardinality_o : Value = get_object(&v[2]);
    let type_o : Value = get_object(&json!("owl:Restriction"));

    json!({"rdf:type" : vec![type_o],
           "owl:onProperty" : vec![property_o],
           "owl:cardinality" : vec![cardinality_o]})
} 

pub fn translate_exact_qualified_cardinality(v : &Value) -> Value {

    let property_o : Value = get_object(&v[1]);
    let cardinality_o : Value = get_object(&v[2]);
    let filler_o : Value = get_object(&v[3]);
    let type_o : Value = get_object(&json!("owl:Restriction"));

    json!({"rdf:type" : vec![type_o],
           "owl:onProperty" : vec![property_o],
           "owl:qualifiedCardinality" : vec![cardinality_o],
           "owl:onClass" : vec![filler_o] })
} 

pub fn translate_list(v : &[Value]) -> Value {

    //TODO: refactor common parts
    if v.len() == 1 {

        let first_o : Value = get_object(&v[0]);
        let rest_o : Value = get_object(&json!("rdf:nil"));


        json!({"rdf:first" : vec![first_o],
               "rdf:rest" : vec![rest_o],
               "datatype" : "_JSON"}) 
    } else { 

        //let first: Value = translate(&v[0]); 
        let rest: Value = translate_list(&v[1..]);//datatype is necessarily _JSON?

        let first_o : Value = get_object(&v[0]);
        let rest_o : Value = json!({"object" : rest, "datatype" : String::from("_JSON")});
        //let rest_o : Value = get_object(rest);
        //
        json!({"rdf:first" : vec![first_o],
               "rdf:rest" : vec![rest_o],
               "datatype" : "_JSON"}) 
    }
}

pub fn translate_intersection_of(v : &Value) -> Value {

    let operands : Value = translate_list(&(v.as_array().unwrap())[1..]);

    let operands_o : Value = json!({"object" : operands,
                                   "datatype" : "_JSON"}); 

    let type_o : Value = get_object(&json!("owl:Class"));

    json!({"rdf:type" : vec![type_o],
           "owl:intersectionOf" : vec![operands_o]}) 
}

pub fn translate_union_of(v : &Value) -> Value {

    let operands : Value = translate_list(&(v.as_array().unwrap())[1..]);

    //let operands_o : Value = get_object(operands);
    let operands_o : Value = json!({"object" : operands,
                                    "datatype" : "_JSON"}); 
    let type_o : Value = get_object(&json!("owl:Class"));

    json!({"rdf:type" : vec![type_o],
           "owl:unionOf" : vec![operands_o]})
} 

pub fn translate_one_of(v : &Value) -> Value {

    let operands : Value = translate_list(&(v.as_array().unwrap())[1..]);

    //let operands_o : Value = get_object(operands);
    let operands_o : Value = json!({"object" : operands,
                                    "datatype" : "_JSON"}); 
    let type_o : Value = get_object(&json!("owl:Class"));

    json!({"rdf:type" : vec![type_o],
           "owl:oneOf" : vec![operands_o]}) 
} 

pub fn translate_complement_of(v : &Value) -> Value {

    let argument_o : Value = get_object(&v[1]);
    let type_o : Value = get_object(&json!("owl:Class"));

    json!({"rdf:type" : vec![type_o],
           "owl:complementOf" : vec![argument_o]}) 
} 
