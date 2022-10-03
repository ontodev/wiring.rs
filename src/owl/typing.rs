use serde::{Deserialize, Serialize};

//TODO: 'typing.rs' is deprecated and should be replaced by 'thick_triple.rs'
//the difference is
//1. structs for 'Members' and 'DistinctMembers'
//2. encoding of 'Objects' (requiring 'datatype' key)

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//                      RDF
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//
#[derive(Debug,Serialize, Deserialize,Clone)]
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

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct SomeValuesFrom {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "owl:someValuesFrom")]
    pub owl_some_values_from: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct AllValuesFrom {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "owl:allValuesFrom")]
    pub owl_all_values_from: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct HasValue {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "owl:hasValue")]
    pub owl_has_value: Vec<Object>, 
}


#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct MinCardinality {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "owl:minCardinality")]
    pub owl_min_cardinality: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone)]
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

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct MaxCardinality {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "owl:maxCardinality")]
    pub owl_max_cardinality: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone)]
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

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct ExactCardinality {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:onProperty")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "owl:cardinality")]
    pub owl_cardinality: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone)]
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

#[derive(Debug,Serialize, Deserialize,Clone)]
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

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct IntersectionOf {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:intersectionOf")]
    pub owl_intersection_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct UnionOf {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:unionOf")]
    pub owl_union_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct OneOf {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:oneOf")]
    pub owl_one_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct ComplementOf {
    #[serde(rename = "rdf:type")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:complementOf")]
    pub owl_complement_of: Vec<Object>,
}

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//           OWL Object Properties
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;


#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct InverseOf {
    #[serde(rename = "owl:inverseOf")]
    pub owl_inverse_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct Object {
    pub object: OWL
}

#[derive(Debug,Serialize, Deserialize,Clone)]
#[serde(untagged)]
pub enum OWL {
    Named(String),
    //Number(i64), //TODO type numbers for cardinality restrictions ?
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
    InverseOf(InverseOf),
    RDFList(RDFList),
}
