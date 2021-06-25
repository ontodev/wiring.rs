use wiring_rs;
use wiring_rs::thick2ofn as thick2ofn;
use wiring_rs::ofn2thick as ofn2thick;
use serde_json::{Value};

//subclass
//equivalence class (binary + nary)
//disjointclasses
//disjointunion
//
//class constructors
//
//property expression
//
//

fn round_trip(input : &str) -> bool {

    //translate thick -> ofn -> thick
    let thick2ofn = thick2ofn::thick_triple_parser::parse_tiple(input);
    let ofn2thick = ofn2thick::ofn_parser::parse_ofn(&thick2ofn); 

    //parse both original input and round-trip translation as JSON
    let orig: Value = serde_json::from_str(input).unwrap();
    let translated : Value = serde_json::from_str(&ofn2thick).unwrap();

    //test whether (generated) JSON values are the same 
    orig == translated 
}

#[test]
fn sub_class_of_axiom() { 
    let subclass_of = r#" {"subject": "ex:subClass", "predicate": "rdfs:subClassOf", "object": "ex:superClass"} "#; 
    assert!(round_trip(subclass_of)); 
}

//#[test]
//fn test_n_ary_equivalence_axiom() { 
//    let equivalence = r#" {"subject": "_:genid1", "predicate": "owl:equivalentClass", "object": {"rdf:rest": [{"object": {"rdf:rest": [{"object": {"rdf:rest": [{"object": "rdf:nil"}], "rdf:first": [{"object": "ex:equivalent2"}]}}], "rdf:first": [{"object": "ex:equivalent1"}]}}], "rdf:first": [{"object": "ex:equivalent3"}]}} "#;
//    assert!(round_trip(equivalence));
//}

#[test]
fn binary_equivalence_axiom() { 
    let equivalence = r#" {"subject": "ex:lhs", "predicate": "owl:equivalentClass", "object": "ex:rhs" } "#;
    assert!(round_trip(equivalence)); 
}

#[test]
fn disjoint_union_axiom() { 
    let disjoint_union = r#" {"subject": "ex:disjointUnion", "predicate": "owl:disjointUnionOf", "object": {"rdf:first": [{"object": "ex:u1"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:u2"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:u3"}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}} "#;
    assert!(round_trip(disjoint_union));
}

#[test]
fn complement_of_expression() { 
    let complement_of = r#" {"subject": "ex:complementOf", "predicate": "owl:equivalentClass", "object": {"rdf:type": [{"object": "owl:Class"}], "owl:complementOf": [{"object": "ex:complement"}]}} "#;
    assert!(round_trip(complement_of));
} 

#[test]
fn some_values_from() { 
    let existential_restriction = r#" {"subject": "ex:existential", "predicate": "rdfs:subClassOf", "object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pExistential"}], "owl:someValuesFrom": [{"object": "ex:existentialFiller"}]}} "#;
    assert!(round_trip(existential_restriction));
} 

#[test]
fn has_self() { 
    let has_self = r#" {"subject": "ex:hasSelf", "predicate": "rdfs:subClassOf", "object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pHasSelf"}], "owl:hasSelf": [{"object": "true^^xsd:boolean"}]}} "#;
    assert!(round_trip(has_self));
}

#[test]
fn has_value() { 
    let has_value = r#" {"subject": "ex:hasValue", "predicate": "rdfs:subClassOf", "object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pHasValue"}], "owl:hasValue": [{"object": "ex:a1"}]}} "#;
    assert!(round_trip(has_value));
}

#[test]
fn max_cardinality() { 
    let max_cardinality = r#" {"subject": "ex:maxCardinality", "predicate": "rdfs:subClassOf", "object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pMaxCardinality"}], "owl:maxCardinality": [{"object": "1^^xsd:nonNegativeInteger"}]}} "#;
    assert!(round_trip(max_cardinality));
}

#[test]
fn max_qualified_cardinality() { 
    let max_qualified_cardinality = r#" {"subject": "ex:maxQualifiedCardinality", "predicate": "rdfs:subClassOf", "object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pMaxQualifiedCardinality"}], "owl:maxQualifiedCardinality": [{"object": "1^^xsd:nonNegativeInteger"}], "owl:onClass": [{"object": "ex:maxCardinalityFiller"}]}} "#; 
    assert!(round_trip(max_qualified_cardinality));
}

#[test]
fn min_cardinality() { 
    let min_cardinality = r#" {"subject": "ex:minCardinality", "predicate": "owl:equivalentClass", "object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pMinCardinality"}], "owl:minCardinality": [{"object": "1^^xsd:nonNegativeInteger"}]}} "#;
    assert!(round_trip(min_cardinality));
}

#[test]
fn min_qualified_cardinality() { 
    let min_qualified_cardinality = r#" {"subject": "ex:minQualifiedCardinality", "predicate": "rdfs:subClassOf", "object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pMinQualifiedCardinality"}], "owl:minQualifiedCardinality": [{"object": "1^^xsd:nonNegativeInteger"}], "owl:onClass": [{"object": "ex:minCardinalityFiller"}]}} "#;
    assert!(round_trip(min_qualified_cardinality));
}

#[test]
fn exact_cardinality_expression() { 
    let exact_cardinality = r#" {"subject": "ex:exactCardinality", "predicate": "rdfs:subClassOf", "object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pExactCardinality"}], "owl:cardinality": [{"object": "2^^xsd:nonNegativeInteger"}]}} "#;

    assert!(round_trip(exact_cardinality));
}

#[test]
fn exact_qualified_cardinality_expression() { 
    let exact_qualified_cardinality = r#" {"subject": "ex:exactQualifiedCardinality", "predicate": "rdfs:subClassOf", "object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pExactQualifiedCardinality"}], "owl:qualifiedCardinality": [{"object": "2^^xsd:nonNegativeInteger"}], "owl:onClass": [{"object": "ex:exactQualifiedCardinalityFiller"}]}} "#; 
    assert!(round_trip(exact_qualified_cardinality));
}

#[test]
fn intersection_expression() {
    let intersection = r#" {"subject": "ex:intersection", "predicate": "rdfs:subClassOf", "object": {"owl:intersectionOf": [{"object": {"rdf:first": [{"object": "ex:I1"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:I2"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:I3"}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}}], "rdf:type": [{"object": "owl:Class"}]}} "#; 
    assert!(round_trip(intersection));
}

#[test]
fn one_of() {
    let one_of = r#" {"subject": "ex:oneOf", "predicate": "rdfs:subClassOf", "object": {"rdf:type": [{"object": "owl:Class"}], "owl:oneOf": [{"object": {"rdf:first": [{"object": "ex:a1"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:a2"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:a3"}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}}]}} "#;
    assert!(round_trip(one_of));
} 

#[test]
fn union_of() {
    let union_of = r#" {"subject": "ex:union", "predicate": "rdfs:subClassOf", "object": {"rdf:type": [{"object": "owl:Class"}], "owl:unionOf": [{"object": {"rdf:first": [{"object": "ex:u1"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:u2"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:u3"}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}}]}} "#;
    assert!(round_trip(union_of));
}
