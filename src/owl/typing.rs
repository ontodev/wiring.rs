use serde::{Deserialize, Serialize};

//NO: axiom are encoded as (thick) triples - 
//and thick triples are not art of the OWL translation
//that will be a one step entry point for axioms
//
///#[derive(Debug,Serialize, Deserialize)]
///pub struct Triple {
///    subject: OWL,
///    predicate: OWL,//need a different type here
///    object: OWL,//would this work? 
///}

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//                      RDF
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//
#[derive(Debug,Serialize, Deserialize)]
pub struct RDFList {
    //#[serde(rename = "rdf:type")]
    //pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "rdf:first")]
    pub rdf_first: Vec<Object>,
    #[serde(rename = "rdf:rest")]
    pub rdf_rest: Vec<Object>, 
}

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//                      Restrictions
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

#[derive(Debug,Serialize, Deserialize)]
pub struct SomeValuesFrom {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "owl:someValuesFrom")]
    pub owl_some_values_from: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize)]
pub struct AllValuesFrom {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "owl:allValuesFrom")]
    pub owl_all_values_from: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize)]
pub struct HasValue {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "owl:hasValue")]
    pub owl_has_value: Vec<Object>, 
}


#[derive(Debug,Serialize, Deserialize)]
pub struct MinCardinality {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "owl:minCardinality")]
    pub owl_min_cardinality: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize)]
pub struct MinQualifiedCardinality {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "owl:minQualifiedCardinality")]
    pub owl_min_qualified_cardinality: Vec<Object>, 
    #[serde(rename = "owl:onClass")]
    pub owl_on_class: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize)]
pub struct MaxCardinality {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "owl:maxCardinality")]
    pub owl_max_cardinality: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize)]
pub struct MaxQualifiedCardinality {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "owl:maxQualifiedCardinality")]
    pub owl_max_qualified_cardinality: Vec<Object>, 
    #[serde(rename = "owl:onClass")]
    pub owl_on_class: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ExactCardinality {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "owl:cardinality")]
    pub owl_cardinality: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ExactQualifiedCardinality {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "owl:qualifiedCardinality")]
    pub owl_qualified_cardinality: Vec<Object>, 
    #[serde(rename = "owl:onClass")]
    pub owl_on_class: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize)]
pub struct HasSelf {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "owl:hasSelf")]
    pub owl_has_self: Vec<Object>,
}

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//           OWL propositional connectives
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

#[derive(Debug,Serialize, Deserialize)]
pub struct IntersectionOf {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:intersectionOf")]
    pub owl_intersection_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct UnionOf {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:unionOf")]
    pub owl_union_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct OneOf {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:oneOf")]
    pub owl_one_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ComplementOf {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:complementOf")]
    pub owl_complement_of: Vec<Object>,
}

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//           OWL Object Properties
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

//TODO: think about refactoring this into a separate module
//Pro: separation of concerns (class translation vs. property translation)
//Con: will make the code harder to read/understand 
//I am leaning towards keeping everything in here - that will blow this file up though
//Alternatively: we keep all typing in here and facotr the translation methods out
//so, we would have a 'central' translation method for everything

#[derive(Debug,Serialize, Deserialize)]
pub struct InverseOf {
    #[serde(rename = "owl:inverseOf")]
    pub owl_inverse_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Object {
    pub object: OWL
}

#[derive(Debug,Serialize, Deserialize)]
//#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum OWL {
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
    //InverseOf(property_translation::InverseOf),
    InverseOf(InverseOf),
    RDFList(RDFList),
}
