use serde::{Deserialize, Serialize};
//use std::fmt;
//use serde_json::json;

//TODO refactor this into a module for thick triples
#[derive(Debug,Serialize, Deserialize)]
pub struct ThickTriple {
    subject: OWL,
    predicate: OWL,
    object: OWL, 
}

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//                      RDF
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//
#[derive(Debug,Serialize, Deserialize)]
pub struct RDFlist {
    #[serde(rename = "rdf:type")]
    rdf_type: Option<Vec<Object>>,
    #[serde(rename = "rdf:first")]
    rdf_first: Vec<Object>,
    #[serde(rename = "rdf:rest")]
    rdf_rest: Vec<Object>, 
}

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//                      Restrictions
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

#[derive(Debug,Serialize, Deserialize)]
pub struct SomeValuesFrom {
    #[serde(rename = "rdf:type")]
    rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    owl_on_property: Vec<Object>,
    #[serde(rename = "owl:someValuesFrom")]
    owl_some_values_from: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize)]
pub struct AllValuesFrom {
    #[serde(rename = "rdf:type")]
    rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    owl_on_property: Vec<Object>,
    #[serde(rename = "owl:allValuesFrom")]
    owl_all_values_from: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize)]
pub struct HasValue {
    #[serde(rename = "rdf:type")]
    rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    owl_on_property: Vec<Object>,
    #[serde(rename = "owl:hasValue")]
    owl_has_value: Vec<Object>, 
}


#[derive(Debug,Serialize, Deserialize)]
pub struct MinCardinality {
    #[serde(rename = "rdf:type")]
    rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    owl_on_property: Vec<Object>,
    #[serde(rename = "owl:minCardinality")]
    owl_min_cardinality: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize)]
pub struct MinQualifiedCardinality {
    #[serde(rename = "rdf:type")]
    rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    owl_on_property: Vec<Object>,
    #[serde(rename = "owl:minQalifiedCardinality")]
    owl_min_qualified_cardinality: Vec<Object>, 
    #[serde(rename = "owl:onClass")]
    owl_on_class: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize)]
pub struct MaxCardinality {
    #[serde(rename = "rdf:type")]
    rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    owl_on_property: Vec<Object>,
    #[serde(rename = "owl:maxCardinality")]
    owl_max_cardinality: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize)]
pub struct MaxQualifiedCardinality {
    #[serde(rename = "rdf:type")]
    rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    owl_on_property: Vec<Object>,
    #[serde(rename = "owl:maxQalifiedCardinality")]
    owl_max_qualified_cardinality: Vec<Object>, 
    #[serde(rename = "owl:onClass")]
    owl_on_class: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ExactCardinality {
    #[serde(rename = "rdf:type")]
    rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    owl_on_property: Vec<Object>,
    #[serde(rename = "owl:cardinality")]
    owl_cardinality: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ExactQualifiedCardinality {
    #[serde(rename = "rdf:type")]
    rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    owl_on_property: Vec<Object>,
    #[serde(rename = "owl:qalifiedCardinality")]
    owl_qualified_cardinality: Vec<Object>, 
    #[serde(rename = "owl:onClass")]
    owl_on_class: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize)]
pub struct HasSelf {
    #[serde(rename = "rdf:type")]
    rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    owl_on_property: Vec<Object>,
    #[serde(rename = "owl:hasSelf")]
    owl_has_self: Vec<Object>,
}

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//           OWL propositional connectives
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

#[derive(Debug,Serialize, Deserialize)]
pub struct IntersectionOf {
    #[serde(rename = "rdf:type")]
    rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:intersectionOf")]
    owl_intersection_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct UnionOf {
    #[serde(rename = "rdf:type")]
    rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:unionOf")]
    owl_union_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct OneOf {
    #[serde(rename = "rdf:type")]
    rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:oneOf")]
    owl_one_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ComplementOf {
    #[serde(rename = "rdf:type")]
    rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:complementOf")]
    owl_complement_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Object {
    object: OWL
}

#[derive(Debug,Serialize, Deserialize)]
//#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum OWL {
    //String,
    Named(String),
    //Number(i64), //we could type numbers for cardinality restrictions - but I don't see the point
    SomeValuesFrom(SomeValuesFrom),
    AllValuesFrom(AllValuesFrom),
    HasValue(HasValue),
    HasSelf(HasSelf),
    MinCardinality(MinCardinality),
    MinQualifiedCardinality(MinQualifiedCardinality),
    MaxCardinality(MaxCardinality),
    MaxQualifiedCardinality(MaxQualifiedCardinality),
    ExactCardinality(ExactCardinality),
    ExactQualifiedCardinality(ExactQualifiedCardinality),
    IntersectionOf(IntersectionOf),
    UnionOf(UnionOf),
    OneOf(OneOf),
    ComplementOf(ComplementOf),
    RDFlist(RDFlist),
}

//pub fn thick2ofn(t: ThickTriple ) -> () {
//pub fn thick2ofn(t: &OWL ) -> () {
pub fn thick2ofn(t: &OWL ) -> String {

    //translate object of thick triples for test purposes
    //let owned_string : String = base2ofn(&t.object);
    //let owned_string : String = base2ofn(t);
    //println!("{}", owned_string); 
    base2ofn(t)
}

pub fn base2ofn(b: &OWL) -> String {
     match &*b {//TODO: don't quite understand why &* is necessary here
        //OWL::Named(x) => println!("Got 50 {:?}", x),
         OWL::Named(x) => translate_named(x.to_string()),
        //OWL::SomeValuesFrom(y) => println!("Matched, y = {:?}", y),

        //restrictions
        OWL::SomeValuesFrom(x) => translate_some_values_from(x),
        OWL::AllValuesFrom(x) => translate_all_values_from(x),
        OWL::HasValue(x) => translate_has_value(x),
        OWL::HasSelf(x) => translate_has_self(x),
        OWL::MinCardinality(x) => translate_min_cardinality(x),
        OWL::MinQualifiedCardinality(x) => translate_min_qualified_cardinality(x),
        OWL::MaxCardinality(x) => translate_max_cardinality(x),
        OWL::MaxQualifiedCardinality(x) => translate_max_qualified_cardinality(x),
        OWL::ExactCardinality(x) => translate_exact_cardinality(x),
        OWL::ExactQualifiedCardinality(x) => translate_exact_qualified_cardinality(x),

        //propositional connectives
        OWL::IntersectionOf(x) => translate_intersection_of(x),
        OWL::UnionOf(x) => translate_union_of(x),
        OWL::OneOf(x) => translate_one_of(x),
        OWL::ComplementOf(x) => translate_complement_of(x),
        OWL::RDFlist(x) => translate_list(x),
        //_ => println!("Default case, x = {:?}", x),
    }
}

pub fn translate_named(s: String) -> String {
    let expression = format!("\"{}\"", s);
        expression
}

pub fn translate_some_values_from(s: &SomeValuesFrom) -> String { 
    let property = base2ofn(&s.owl_on_property[0].object);
    let filler =  base2ofn(&s.owl_some_values_from[0].object);
    let expression = format!("[\"ObjectSomeValuesFrom\",{},{}]", property, filler);
    expression
}

pub fn translate_all_values_from(s: &AllValuesFrom) -> String { 
    let property = base2ofn(&s.owl_on_property[0].object);
    let filler =  base2ofn(&s.owl_all_values_from[0].object);
    let expression = format!("[\"ObjectAllValuesFrom\",{},{}]", property, filler);
    expression
}

pub fn translate_has_value(s: &HasValue) -> String { 
    let property = base2ofn(&s.owl_on_property[0].object);
    let filler =  base2ofn(&s.owl_has_value[0].object);
    let expression = format!("[\"ObjectHasValue\",{},{}]", property, filler);
    expression
}

pub fn translate_has_self(s: &HasSelf) -> String { 
    let property = base2ofn(&s.owl_on_property[0].object);
    let expression = format!("[\"ObjectHasSelf\",{}]", property);
    //ignoring "owl_has_self" because that only contains "true^^xsd:boolean"
    expression
}

pub fn translate_min_cardinality(s: &MinCardinality) -> String { 
    let property = base2ofn(&s.owl_on_property[0].object);
    let cardinality =  base2ofn(&s.owl_min_cardinality[0].object);
    let expression = format!("[\"ObjectMinCardinality\",{},{}]", property, cardinality);
    expression
}

pub fn translate_min_qualified_cardinality(s: &MinQualifiedCardinality) -> String { 
    let property = base2ofn(&s.owl_on_property[0].object);
    let cardinality =  base2ofn(&s.owl_min_qualified_cardinality[0].object);
    let filler =  base2ofn(&s.owl_on_class[0].object);
    let expression = format!("[\"ObjectMinCardinality\",{},{},{}]", property, cardinality, filler);
    expression
}

pub fn translate_max_cardinality(s: &MaxCardinality) -> String { 
    let property = base2ofn(&s.owl_on_property[0].object);
    let cardinality =  base2ofn(&s.owl_max_cardinality[0].object);
    let expression = format!("[\"ObjectMaxCardinality\",{},{}]", property, cardinality);
    expression
}

pub fn translate_max_qualified_cardinality(s: &MaxQualifiedCardinality) -> String { 
    let property = base2ofn(&s.owl_on_property[0].object);
    let cardinality =  base2ofn(&s.owl_max_qualified_cardinality[0].object);
    let filler =  base2ofn(&s.owl_on_class[0].object);
    let expression = format!("[\"ObjectMaxCardinality\",{},{},{}]", property, cardinality, filler);
    expression
}

pub fn translate_exact_cardinality(s: &ExactCardinality) -> String { 
    let property = base2ofn(&s.owl_on_property[0].object);
    let cardinality =  base2ofn(&s.owl_cardinality[0].object);
    let expression = format!("[\"ObjectExactCardinality\",{},{}]", property, cardinality);
    expression
}

pub fn translate_exact_qualified_cardinality(s: &ExactQualifiedCardinality) -> String { 
    let property = base2ofn(&s.owl_on_property[0].object);
    let cardinality =  base2ofn(&s.owl_qualified_cardinality[0].object);
    let filler =  base2ofn(&s.owl_on_class[0].object);
    let expression = format!("[\"ObjectExactCardinality\",{},{},{}]", property, cardinality, filler);
    expression
} 

pub fn translate_list(s: &RDFlist) -> String { 
    let first = base2ofn(&s.rdf_first[0].object);
    let rest =  base2ofn(&s.rdf_rest[0].object);

    //match &rest[..] {
    match &*rest {
        "\"rdf:nil\"" => format!("{}", first),
        _ => format!("{},{}", first, rest),
    }
}

pub fn translate_intersection_of(s: &IntersectionOf) -> String { 
    let intersection_of = base2ofn(&s.owl_intersection_of[0].object);
    let expression = format!("[\"ObjectIntersectionOf\",{}]", intersection_of);
    expression
}

pub fn translate_union_of(s: &UnionOf) -> String { 
    let union_of = base2ofn(&s.owl_union_of[0].object);
    let expression = format!("[\"ObjectUnionOf\",{}]", union_of);
    expression
}

pub fn translate_one_of(s: &OneOf) -> String { 
    let one_of = base2ofn(&s.owl_one_of[0].object);
    let expression = format!("[\"ObjectOneOf\",{}]", one_of);
    expression
}

pub fn translate_complement_of(s: &ComplementOf) -> String { 
    let complement_of = base2ofn(&s.owl_complement_of[0].object);
    let expression = format!("[\"ObjectComplementOf\",{}]", complement_of);
    expression
}
