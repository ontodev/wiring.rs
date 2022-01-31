use serde_json::{Value};
use crate::owl::typing as owl;
use crate::ofn2thick::property_translation as property_translation;

//Note that (thick) triples are not OWL
pub fn translate(v : &Value) -> owl::OWL {

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
        Some(_) => panic!(),
        None => owl::OWL::Named(String::from(v.as_str().unwrap())),
    }
}

pub fn get_object(owl : owl::OWL) -> owl::Object {
    owl::Object{object : owl }
}

pub fn translate_some_values_from(v : &Value) -> owl::OWL {

    //translate recursively
    //let op: &Value = &v[0];//don't need OWL constructor 
    let property: owl::OWL = translate(&v[1]); 
    let filler: owl::OWL = translate(&v[2]); 

    //build objects
    let property_o : owl::Object = get_object(property);
    let filler_o : owl::Object = get_object(filler);
    let type_o : owl::Object = get_object(owl::OWL::Named("owl:Restriction".to_string()));

    //build struct
    let res : owl::SomeValuesFrom = owl::SomeValuesFrom{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_some_values_from  : vec![filler_o]}; 
    //return type
    owl::OWL::SomeValuesFrom(res) 
} 


pub fn translate_all_values_from(v : &Value) -> owl::OWL {

    let property: owl::OWL = translate(&v[1]); 
    let filler: owl::OWL = translate(&v[2]); 

    let property_o : owl::Object = get_object(property);
    let filler_o : owl::Object = get_object(filler);
    let type_o : owl::Object = get_object(owl::OWL::Named("owl:Restriction".to_string()));

    let res : owl::AllValuesFrom = owl::AllValuesFrom{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_all_values_from  : vec![filler_o]}; 
    owl::OWL::AllValuesFrom(res) 
} 

pub fn translate_has_value(v : &Value) -> owl::OWL {

    let property: owl::OWL = translate(&v[1]); 
    let filler: owl::OWL = translate(&v[2]); 

    let property_o : owl::Object = get_object(property);
    let filler_o : owl::Object = get_object(filler);
    let type_o : owl::Object = get_object(owl::OWL::Named("owl:Restriction".to_string()));

    let res : owl::HasValue = owl::HasValue{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_has_value  : vec![filler_o]}; 
    owl::OWL::HasValue(res) 
} 

pub fn translate_has_self(v : &Value) -> owl::OWL {

    let property: owl::OWL = translate(&v[1]); 

    let property_o : owl::Object = get_object(property);
    let has_self_o : owl::Object = get_object(owl::OWL::Named("true^^xsd:boolean".to_string()));
    let type_o : owl::Object = get_object(owl::OWL::Named("owl:Restriction".to_string()));

    let res : owl::HasSelf = owl::HasSelf{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_has_self  : vec![has_self_o]}; 
    owl::OWL::HasSelf(res) 
} 



pub fn translate_min_cardinality(v : &Value) -> owl::OWL {

    let property: owl::OWL = translate(&v[1]); 
    let cardinliaty: owl::OWL = translate(&v[2]); 

    let property_o : owl::Object = get_object(property);
    let cardinality_o : owl::Object = get_object(cardinliaty);
    let type_o : owl::Object = get_object(owl::OWL::Named("owl:Restriction".to_string()));

    let res : owl::MinCardinality = owl::MinCardinality{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_min_cardinality  : vec![cardinality_o]}; 
    owl::OWL::MinCardinality(res) 
} 

pub fn translate_min_qualified_cardinality(v : &Value) -> owl::OWL {

    let property: owl::OWL = translate(&v[1]); 
    let cardinliaty: owl::OWL = translate(&v[2]); 
    let filler: owl::OWL = translate(&v[3]); 

    let property_o : owl::Object = get_object(property);
    let cardinality_o : owl::Object = get_object(cardinliaty);
    let filler_o : owl::Object = get_object(filler);
    let type_o : owl::Object = get_object(owl::OWL::Named("owl:Restriction".to_string()));

    let res : owl::MinQualifiedCardinality = owl::MinQualifiedCardinality{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_min_qualified_cardinality : vec![cardinality_o],
                                                          owl_on_class : vec![filler_o]}; 
    owl::OWL::MinQualifiedCardinality(res) 
} 

pub fn translate_max_cardinality(v : &Value) -> owl::OWL {

    let property: owl::OWL = translate(&v[1]); 
    let cardinliaty: owl::OWL = translate(&v[2]); 

    let property_o : owl::Object = get_object(property);
    let cardinality_o : owl::Object = get_object(cardinliaty);
    let type_o : owl::Object = get_object(owl::OWL::Named("owl:Restriction".to_string()));

    let res : owl::MaxCardinality = owl::MaxCardinality{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_max_cardinality  : vec![cardinality_o]}; 
    owl::OWL::MaxCardinality(res) 
} 

pub fn translate_max_qualified_cardinality(v : &Value) -> owl::OWL {

    let property: owl::OWL = translate(&v[1]); 
    let cardinliaty: owl::OWL = translate(&v[2]); 
    let filler: owl::OWL = translate(&v[3]); 

    let property_o : owl::Object = get_object(property);
    let cardinality_o : owl::Object = get_object(cardinliaty);
    let filler_o : owl::Object = get_object(filler);
    let type_o : owl::Object = get_object(owl::OWL::Named("owl:Restriction".to_string()));

    let res : owl::MaxQualifiedCardinality = owl::MaxQualifiedCardinality{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_max_qualified_cardinality : vec![cardinality_o],
                                                          owl_on_class : vec![filler_o]}; 
    owl::OWL::MaxQualifiedCardinality(res) 
} 

pub fn translate_exact_cardinality(v : &Value) -> owl::OWL {

    let property: owl::OWL = translate(&v[1]); 
    let cardinliaty: owl::OWL = translate(&v[2]); 

    let property_o : owl::Object = get_object(property);
    let cardinality_o : owl::Object = get_object(cardinliaty);
    let type_o : owl::Object = get_object(owl::OWL::Named("owl:Restriction".to_string()));

    let res : owl::ExactCardinality = owl::ExactCardinality{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_cardinality  : vec![cardinality_o]}; 
    owl::OWL::ExactCardinality(res) 
} 

pub fn translate_exact_qualified_cardinality(v : &Value) -> owl::OWL {

    let property: owl::OWL = translate(&v[1]); 
    let cardinliaty: owl::OWL = translate(&v[2]); 
    let filler: owl::OWL = translate(&v[3]); 

    let property_o : owl::Object = get_object(property);
    let cardinality_o : owl::Object = get_object(cardinliaty);
    let filler_o : owl::Object = get_object(filler);
    let type_o : owl::Object = get_object(owl::OWL::Named("owl:Restriction".to_string()));

    let res : owl::ExactQualifiedCardinality = owl::ExactQualifiedCardinality{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_qualified_cardinality : vec![cardinality_o],
                                                          owl_on_class : vec![filler_o]}; 
    owl::OWL::ExactQualifiedCardinality(res) 
} 

pub fn translate_list(v : &[Value]) -> owl::OWL {

    //TODO: refactor common parts
    if v.len() == 1 {
        let first: owl::OWL = translate(&v[0]); 
        let rest = owl::OWL::Named("rdf:nil".to_string());

        let first_o : owl::Object = get_object(first);
        let rest_o : owl::Object = get_object(rest);

        let res : owl::RDFList = owl::RDFList { rdf_first : vec![first_o],
                       rdf_rest : vec![rest_o]};
        owl::OWL::RDFList(res)

    } else { 

        let first: owl::OWL = translate(&v[0]); 
        let rest: owl::OWL = translate_list(&v[1..]);

        let first_o : owl::Object = get_object(first);
        let rest_o : owl::Object = get_object(rest);

        let res : owl::RDFList = owl::RDFList { rdf_first : vec![first_o],
                       rdf_rest : vec![rest_o]};
        owl::OWL::RDFList(res) 
    } 
}

pub fn translate_intersection_of(v : &Value) -> owl::OWL {

    let operands : owl::OWL = translate_list(&(v.as_array().unwrap())[1..]);

    let operands_o : owl::Object = get_object(operands);
    let type_o : owl::Object = get_object(owl::OWL::Named("owl:Class".to_string()));

    let res : owl::IntersectionOf = owl::IntersectionOf{ rdf_type : Some(vec![type_o]),
                                                          owl_intersection_of : vec![operands_o]}; 
    owl::OWL::IntersectionOf(res) 
} 

pub fn translate_union_of(v : &Value) -> owl::OWL {

    let operands : owl::OWL = translate_list(&(v.as_array().unwrap())[1..]);

    let operands_o : owl::Object = get_object(operands);
    let type_o : owl::Object = get_object(owl::OWL::Named("owl:Class".to_string()));

    let res : owl::UnionOf = owl::UnionOf{ rdf_type : Some(vec![type_o]),
                                                          owl_union_of : vec![operands_o]}; 
    owl::OWL::UnionOf(res) 
} 

pub fn translate_one_of(v : &Value) -> owl::OWL {

    let operands : owl::OWL = translate_list(&(v.as_array().unwrap())[1..]);

    let operands_o : owl::Object = get_object(operands);
    let type_o : owl::Object = get_object(owl::OWL::Named("owl:Class".to_string()));

    let res : owl::OneOf = owl::OneOf{ rdf_type : Some(vec![type_o]),
                                                          owl_one_of : vec![operands_o]}; 
    owl::OWL::OneOf(res) 
} 

pub fn translate_complement_of(v : &Value) -> owl::OWL {

    let argument: owl::OWL = translate(&v[1]); 

    let argument_o : owl::Object = get_object(argument);
    let type_o : owl::Object = get_object(owl::OWL::Named("owl:Class".to_string()));

    let res : owl::ComplementOf = owl::ComplementOf{ rdf_type : Some(vec![type_o]),
                                                          owl_complement_of : vec![argument_o]}; 
    owl::OWL::ComplementOf(res) 
} 
