use serde::{Deserialize, Serialize};

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//                      RDF
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//
#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct RDFList {
    //#[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    //pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#first>")]
    pub rdf_first: Vec<Object>,
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#rest>")]
    pub rdf_rest: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct Members {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]//TODO: the type is *not* optional for 'owl:members'?
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#members>")]
    pub members: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct DistinctMembers {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#distinctMembers>")]
    pub distinct_members: Vec<Object>, 
}

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//                      Restrictions
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct SomeValuesFrom {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onProperty>")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#someValuesFrom>")]
    pub owl_some_values_from: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct AllValuesFrom {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onProperty>")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#allValuesFrom>")]
    pub owl_all_values_from: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct HasValue {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onProperty>")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#hasValue>")]
    pub owl_has_value: Vec<Object>, 
} 

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct MinCardinality {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onProperty>")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#minCardinality>")]
    pub owl_min_cardinality: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct MinObjectQualifiedCardinality {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onProperty>")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#minQualifiedCardinality>")]
    pub owl_min_qualified_cardinality: Vec<Object>, 
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onClass>")]
    pub owl_on_class: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct MinDataQualifiedCardinality {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onProperty>")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#minQualifiedCardinality>")]
    pub owl_min_qualified_cardinality: Vec<Object>, 
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onDataRange>")]
    pub owl_on_datarange: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct MaxCardinality {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onProperty>")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#maxCardinality>")]
    pub owl_max_cardinality: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct MaxObjectQualifiedCardinality {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onProperty>")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#maxQualifiedCardinality>")]
    pub owl_max_qualified_cardinality: Vec<Object>, 
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onClass>")]
    pub owl_on_class: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct MaxDataQualifiedCardinality {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onProperty>")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#maxQualifiedCardinality>")]
    pub owl_max_qualified_cardinality: Vec<Object>, 
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onDataRange>")]
    pub owl_on_datarange: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct ExactCardinality {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onProperty>")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#cardinality>")]
    pub owl_cardinality: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct ExactObjectQualifiedCardinality {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onProperty>")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#qualifiedCardinality>")]
    pub owl_qualified_cardinality: Vec<Object>, 
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onClass>")]
    pub owl_on_class: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct ExactDataQualifiedCardinality {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onProperty>")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#qualifiedCardinality>")]
    pub owl_qualified_cardinality: Vec<Object>, 
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onDataRange>")]
    pub owl_on_datarange: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct HasSelf {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#onProperty>")]
    pub owl_on_property: Vec<Object>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#hasSelf>")]
    pub owl_has_self: Vec<Object>,
}

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//           OWL propositional connectives
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct IntersectionOf {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#intersectionOf>")]
    pub owl_intersection_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct UnionOf {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#unionOf>")]
    pub owl_union_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct OneOf {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#oneOf>")]
    pub owl_one_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct ComplementOf {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "<http://www.w3.org/2002/07/owl#complementOf>")]
    pub owl_complement_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct NegativeObjectPropertyAssertion {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:sourceIndividual")]
    pub source_individual: Vec<Object>,
    #[serde(rename = "owl:assertionProperty")]
    pub assertion_property: Vec<Object>,
    #[serde(rename = "owl:targetIndividual")]
    pub target_individual: Vec<Object>, 
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct NegativeDataPropertyAssertion {
    #[serde(rename = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")]
    pub rdf_type: Option<Vec<Object>>,
    #[serde(rename = "owl:sourceIndividual")]
    pub source_individual: Vec<Object>,
    #[serde(rename = "owl:assertionProperty")]
    pub assertion_property: Vec<Object>,
    #[serde(rename = "owl:targetValue")]
    pub target_value: Vec<Object>, 
}

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//           OWL Object Properties
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct InverseOf {
    #[serde(rename = "<http://www.w3.org/2002/07/owl#inverseOf>")]
    pub owl_inverse_of: Vec<Object>,
}

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
pub struct Object {
    pub object: OWL,
    pub datatype: String,
    pub meta: Option<String>,
    //pub annotation: String,
} 

#[derive(Debug,Serialize, Deserialize,Clone,Hash)]
#[serde(untagged)]
pub enum OWL {
    Named(String), 
    //Number(i64), //TODO type numbers for cardinality restrictions ?
    SomeValuesFrom(SomeValuesFrom),
    AllValuesFrom(AllValuesFrom),
    HasValue(HasValue),
    HasSelf(HasSelf),

    MinCardinality(MinCardinality),
    MaxCardinality(MaxCardinality),
    ExactCardinality(ExactCardinality),

    MinObjectQualifiedCardinality(MinObjectQualifiedCardinality),
    MaxObjectQualifiedCardinality(MaxObjectQualifiedCardinality),
    ExactObjectQualifiedCardinality(ExactObjectQualifiedCardinality),

    MinDataQualifiedCardinality(MinDataQualifiedCardinality),
    MaxDataQualifiedCardinality(MaxDataQualifiedCardinality),
    ExactDataQualifiedCardinality(ExactDataQualifiedCardinality),

    IntersectionOf(IntersectionOf),
    UnionOf(UnionOf),
    OneOf(OneOf),
    ComplementOf(ComplementOf),
    InverseOf(InverseOf),
    RDFList(RDFList),
    List(Vec<Object>),
    Members(Members),
    DistinctMembers(DistinctMembers),

    NegativeObjectPropertyAssertion(NegativeObjectPropertyAssertion),
    NegativeDataPropertyAssertion(NegativeDataPropertyAssertion), 
}
