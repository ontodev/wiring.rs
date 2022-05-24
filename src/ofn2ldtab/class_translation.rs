use serde_json::{Value};
use serde_json::json;
use crate::owl::thick_triple as owl;
use crate::ofn2ldtab::property_translation as property_translation;
use crate::ofn2ldtab::util as util;

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

pub fn translate_named_entity(v: &Value) -> owl::OWL {
        let o: String = String::from(v.as_str().unwrap());
        let d: String = String::from(util::translate_datatype(&v).as_str().unwrap());

        let terminal = owl::TerminalObject{object : o, datatype: d, meta: None };
        owl::OWL::TerminalObject(terminal) 
}

//pub fn get_object(owl : owl::OWL) -> owl::Object {
//    owl::Object{object : owl, datatype: String::from("asd"), meta: None }
//}

pub fn get_object(v : &Value) -> owl::Object {
    let o: owl::OWL = translate(&v);
    let d: String = String::from(util::translate_datatype(&v).as_str().unwrap());

    owl::Object{object : o, datatype: d, meta: None }
}

pub fn translate_some_values_from(v : &Value) -> owl::OWL {

    let property_o : owl::Object = get_object(&v[1]);
    let filler_o : owl::Object = get_object(&v[2]);
    let type_o : owl::Object = get_object(&json!("owl:Restriction"));

    //build struct
    let res : owl::SomeValuesFrom = owl::SomeValuesFrom{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_some_values_from  : vec![filler_o]}; 
    //return type
    owl::OWL::SomeValuesFrom(res) 
} 


pub fn translate_all_values_from(v : &Value) -> owl::OWL {

    let property_o : owl::Object = get_object(&v[1]);
    let filler_o : owl::Object = get_object(&v[2]);
    let type_o : owl::Object = get_object(&json!("owl:Restriction"));

    let res : owl::AllValuesFrom = owl::AllValuesFrom{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_all_values_from  : vec![filler_o]}; 
    owl::OWL::AllValuesFrom(res) 
} 

pub fn translate_has_value(v : &Value) -> owl::OWL {

    let property_o : owl::Object = get_object(&v[1]);
    let filler_o : owl::Object = get_object(&v[2]);
    let type_o : owl::Object = get_object(&json!("owl:Restriction"));

    let res : owl::HasValue = owl::HasValue{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_has_value  : vec![filler_o]}; 
    owl::OWL::HasValue(res) 
} 

pub fn translate_has_self(v : &Value) -> owl::OWL {

    let property_o : owl::Object = get_object(&v[1]);
    let has_self_o : owl::Object = get_object(&json!("true^^xsd:boolean"));
    let type_o : owl::Object = get_object(&json!("owl:Restriction"));

    let res : owl::HasSelf = owl::HasSelf{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_has_self  : vec![has_self_o]}; 
    owl::OWL::HasSelf(res) 
} 


pub fn translate_min_cardinality(v : &Value) -> owl::OWL {

    let property_o : owl::Object = get_object(&v[1]);
    let cardinality_o : owl::Object = get_object(&v[2]);
    let type_o : owl::Object = get_object(&json!("owl:Restriction"));

    let res : owl::MinCardinality = owl::MinCardinality{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_min_cardinality  : vec![cardinality_o]}; 
    owl::OWL::MinCardinality(res) 
} 

pub fn translate_min_qualified_cardinality(v : &Value) -> owl::OWL {

    let property_o : owl::Object = get_object(&v[1]);
    let cardinality_o : owl::Object = get_object(&v[2]);
    let filler_o : owl::Object = get_object(&v[3]);
    let type_o : owl::Object = get_object(&json!("owl:Restriction"));

    let res : owl::MinQualifiedCardinality = owl::MinQualifiedCardinality{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_min_qualified_cardinality : vec![cardinality_o],
                                                          owl_on_class : vec![filler_o]}; 
    owl::OWL::MinQualifiedCardinality(res) 
} 

pub fn translate_max_cardinality(v : &Value) -> owl::OWL {

    let property_o : owl::Object = get_object(&v[1]);
    let cardinality_o : owl::Object = get_object(&v[2]);
    let type_o : owl::Object = get_object(&json!("owl:Restriction"));

    let res : owl::MaxCardinality = owl::MaxCardinality{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_max_cardinality  : vec![cardinality_o]}; 
    owl::OWL::MaxCardinality(res) 
} 

pub fn translate_max_qualified_cardinality(v : &Value) -> owl::OWL {

    let property_o : owl::Object = get_object(&v[1]);
    let cardinality_o : owl::Object = get_object(&v[2]);
    let filler_o : owl::Object = get_object(&v[3]);
    let type_o : owl::Object = get_object(&json!("owl:Restriction"));

    let res : owl::MaxQualifiedCardinality = owl::MaxQualifiedCardinality{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_max_qualified_cardinality : vec![cardinality_o],
                                                          owl_on_class : vec![filler_o]}; 
    owl::OWL::MaxQualifiedCardinality(res) 
} 

pub fn translate_exact_cardinality(v : &Value) -> owl::OWL {

    let property_o : owl::Object = get_object(&v[1]);
    let cardinality_o : owl::Object = get_object(&v[2]);
    let type_o : owl::Object = get_object(&json!("owl:Restriction"));

    let res : owl::ExactCardinality = owl::ExactCardinality{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_cardinality  : vec![cardinality_o]}; 
    owl::OWL::ExactCardinality(res) 
} 

pub fn translate_exact_qualified_cardinality(v : &Value) -> owl::OWL {

    let property_o : owl::Object = get_object(&v[1]);
    let cardinality_o : owl::Object = get_object(&v[2]);
    let filler_o : owl::Object = get_object(&v[3]);
    let type_o : owl::Object = get_object(&json!("owl:Restriction"));

    let res : owl::ExactQualifiedCardinality = owl::ExactQualifiedCardinality{ rdf_type : Some(vec![type_o]),
                                                          owl_on_property : vec![property_o],
                                                          owl_qualified_cardinality : vec![cardinality_o],
                                                          owl_on_class : vec![filler_o]}; 
    owl::OWL::ExactQualifiedCardinality(res) 
} 

pub fn translate_list(v : &[Value]) -> owl::OWL {

    //TODO: refactor common parts
    if v.len() == 1 {

        let first_o : owl::Object = get_object(&v[0]);
        let rest_o : owl::Object = get_object(&json!("rdf:nil"));

        let res : owl::RDFList = owl::RDFList { rdf_first : vec![first_o],
                       rdf_rest : vec![rest_o]};
        owl::OWL::RDFList(res)

    } else { 

        //let first: owl::OWL = translate(&v[0]); 
        let rest: owl::OWL = translate_list(&v[1..]);//datatype is necessarily _JSON?

        let first_o : owl::Object = get_object(&v[0]);
        let rest_o : owl::Object = owl::Object{object : rest, datatype : String::from("_JSON"), meta: None };
        //let rest_o : owl::Object = get_object(rest);

        let res : owl::RDFList = owl::RDFList { rdf_first : vec![first_o],
                       rdf_rest : vec![rest_o]};
        owl::OWL::RDFList(res) 
    } 
}

pub fn translate_intersection_of(v : &Value) -> owl::OWL {

    let operands : owl::OWL = translate_list(&(v.as_array().unwrap())[1..]);

    let operands_o : owl::Object = owl::Object{object : operands, datatype : String::from("_JSON"), meta: None };

    let type_o : owl::Object = get_object(&json!("owl:Class"));

    let res : owl::IntersectionOf = owl::IntersectionOf{ rdf_type : Some(vec![type_o]),
                                                          owl_intersection_of : vec![operands_o]}; 
    owl::OWL::IntersectionOf(res) 
} 

pub fn translate_union_of(v : &Value) -> owl::OWL {

    let operands : owl::OWL = translate_list(&(v.as_array().unwrap())[1..]);

    //let operands_o : owl::Object = get_object(operands);
    let operands_o : owl::Object = owl::Object{object : operands, datatype : String::from("_JSON"), meta: None };
    let type_o : owl::Object = get_object(&json!("owl:Class"));

    let res : owl::UnionOf = owl::UnionOf{ rdf_type : Some(vec![type_o]),
                                                          owl_union_of : vec![operands_o]}; 
    owl::OWL::UnionOf(res) 
} 

pub fn translate_one_of(v : &Value) -> owl::OWL {

    let operands : owl::OWL = translate_list(&(v.as_array().unwrap())[1..]);

    //let operands_o : owl::Object = get_object(operands);
    let operands_o : owl::Object = owl::Object{object : operands, datatype : String::from("_JSON"), meta: None };
    let type_o : owl::Object = get_object(&json!("owl:Class"));

    let res : owl::OneOf = owl::OneOf{ rdf_type : Some(vec![type_o]),
                                                          owl_one_of : vec![operands_o]}; 
    owl::OWL::OneOf(res) 
} 

pub fn translate_complement_of(v : &Value) -> owl::OWL {

    let argument_o : owl::Object = get_object(&v[1]);
    let type_o : owl::Object = get_object(&json!("owl:Class"));

    let res : owl::ComplementOf = owl::ComplementOf{ rdf_type : Some(vec![type_o]),
                                                          owl_complement_of : vec![argument_o]}; 
    owl::OWL::ComplementOf(res) 
} 
