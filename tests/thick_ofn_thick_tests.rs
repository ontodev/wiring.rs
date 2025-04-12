use serde_json::Value;
use wiring_rs;
//use wiring_rs::ofn_2_thick;
//use wiring_rs::thick_2_ofn;
use wiring_rs::ofn_2_ldtab;
use wiring_rs::ldtab_2_ofn;

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

fn round_trip(input: &str) -> bool {
    //translate thick -> ofn -> thick
    let orig: Value = serde_json::from_str(input).unwrap();
    println!("orig: {:?}", orig);
    let ldtab_2_ofn = ldtab_2_ofn::translation::thick_triple_2_ofn(&orig);
    println!("ldtab_2_ofn: {:?}", ldtab_2_ofn);
    let ofn_2_ldtab = ofn_2_ldtab::translation::ofn_2_thick_triple(&ldtab_2_ofn);
    println!("ofn_2_ldtab: {:?}", ofn_2_ldtab);

    //test whether (generated) JSON values are the same
    orig == ofn_2_ldtab
}

#[test]
fn sub_class_of_axiom() {
    let subclass_of = r#" {"subject": "ex:subClass",
                           "predicate": "rdfs:subClassOf",
                           "object": "ex:superClass",
                           "datatype" : "_IRI",
                           "annotation" : {},
                           "graph" : "graph",
                           "assertion" : "1",
                           "retraction" : "0" }"#;
    assert!(round_trip(subclass_of));
}

#[test]
fn test_n_ary_equivalence_axiom() {
    let equivalence = r#" {"subject": "<ldtab:blanknode:e290d01aaeeb3aa3b9960e2b9299214d83ce6f224c3da50084915d473595dc75>",
                           "predicate": "owl:equivalentClass",
                           "object": {"owl:members" : [{"object" : [
                                                       {"object": "ex:equivalent1", "datatype" : "_IRI"},
                                                       {"object": "ex:equivalent2", "datatype" : "_IRI"},
                                                       {"object": "ex:equivalent3", "datatype" : "_IRI"}],
                                                      "datatype" : "_JSONLIST"}]},
                           "datatype" : "_JSONMAP",
                           "annotation" : {},
                           "graph" : "graph",
                           "assertion" : "1",
                           "retraction" : "0" }"#;

    assert!(round_trip(equivalence));
}

#[test]
fn binary_equivalence_axiom() {
    let equivalence =
        r#" {"subject": "ex:lhs",
             "predicate": "owl:equivalentClass",
             "object": "ex:rhs",
             "datatype" : "_IRI",
             "annotation" : {},
             "graph" : "graph",
             "assertion" : "1",
             "retraction" : "0" }"#;
    assert!(round_trip(equivalence));
}

#[test]
fn disjoint_union_axiom() {
    let disjoint_union = r#" {"subject": "ex:disjointUnion",
                              "predicate": "owl:disjointUnionOf",
                              "object": [{"object": "ex:u1", "datatype" : "_IRI"},
                                         {"object": "ex:u2", "datatype" : "_IRI"},
                                         {"object": "ex:u3", "datatype" : "_IRI"}],
                           "datatype" : "_JSONLIST",
                           "annotation" : {},
                           "graph" : "graph",
                           "assertion" : "1",
                           "retraction" : "0" }"#;
    assert!(round_trip(disjoint_union));
}

#[test]
fn complement_of_expression() {
    let complement_of = r#" {"subject": "ex:complementOf",
                             "predicate": "owl:equivalentClass",
                             "object": {"rdf:type": [{"object": "owl:Class", "datatype": "_IRI"}],
                                        "owl:complementOf": [{"object": "ex:complement", "datatype": "_IRI"}]}, 
                           "datatype" : "_JSONMAP",
                           "annotation" : {},
                           "graph" : "graph",
                           "assertion" : "1",
                           "retraction" : "0" }"#;
    assert!(round_trip(complement_of));
}

#[test]
fn some_values_from() {
    let existential_restriction = r#" {"subject": "ex:existential",
                                       "predicate": "rdfs:subClassOf",
                                       "object": {"rdf:type": [{"object": "owl:Restriction", "datatype": "_IRI"}],
                                                  "owl:onProperty": [{"object": "ex:pExistential", "datatype": "_IRI"}],
                                                  "owl:someValuesFrom": [{"object": "ex:existentialFiller", "datatype": "_IRI"}]},
                                        "datatype" : "_JSONMAP",
                                        "annotation" : {},
                                        "graph" : "graph",
                                        "assertion" : "1",
                                        "retraction" : "0" } "#;
    assert!(round_trip(existential_restriction));
}

#[test]
fn has_self() {
    let has_self = r#" {"subject": "ex:hasSelf",
                        "predicate": "rdfs:subClassOf",
                        "object": {"rdf:type": [{"object": "owl:Restriction", "datatype": "_IRI"}],
                                   "owl:onProperty": [{"object": "ex:pHasSelf", "datatype": "_IRI"}],
                                   "owl:hasSelf": [{"object": "true^^xsd:boolean", "datatype": "_IRI"}]},
                        "datatype" : "_JSONMAP",
                        "annotation" : {},
                        "graph" : "graph",
                        "assertion" : "1",
                        "retraction" : "0" } "#;
    assert!(round_trip(has_self));
}

#[test]
fn has_value() {
    let has_value = r#" {"subject": "ex:hasValue",
                         "predicate": "rdfs:subClassOf",
                         "object": {"rdf:type": [{"object": "owl:Restriction", "datatype": "_IRI"}],
                                    "owl:onProperty": [{"object": "ex:pHasValue", "datatype": "_IRI"}],
                                    "owl:hasValue": [{"object": "ex:a1", "datatype": "_IRI"}]},
                         "datatype" : "_JSONMAP",
                         "annotation" : {},
                         "graph" : "graph",
                         "assertion" : "1",
                         "retraction" : "0" } "#;
    assert!(round_trip(has_value));
}

#[test]
fn max_cardinality() {
    let max_cardinality = r#" {"subject": "ex:maxCardinality",
                               "predicate": "rdfs:subClassOf",
                               "object": {"rdf:type": [{"object": "owl:Restriction", "datatype": "_IRI"}],
                                          "owl:onProperty": [{"object": "ex:pMaxCardinality", "datatype": "_IRI"}],
                                          "owl:maxCardinality": [{"object": "1", "datatype": "xsd:nonNegativeInteger"}]},
                                "datatype" : "_JSONMAP",
                                "annotation" : {},
                                "graph" : "graph",
                                "assertion" : "1",
                                "retraction" : "0" } "#;
    assert!(round_trip(max_cardinality));
}

#[test]
fn max_qualified_cardinality() {
    let max_qualified_cardinality = r#" {"subject": "ex:maxQualifiedCardinality",
                                         "predicate": "rdfs:subClassOf",
                                         "object": {"rdf:type": [{"object": "owl:Restriction", "datatype": "_IRI"}],
                                                    "owl:onProperty": [{"object": "ex:pMaxQualifiedCardinality", "datatype": "_IRI"}],
                                                    "owl:maxQualifiedCardinality": [{"object": "1", "datatype": "xsd:nonNegativeInteger"}],
                                                    "owl:onClass": [{"object": "ex:maxCardinalityFiller", "datatype": "_IRI"}]},

                                        "datatype" : "_JSONMAP",
                                        "annotation" : {},
                                        "graph" : "graph",
                                        "assertion" : "1",
                                        "retraction" : "0" } "#;
    assert!(round_trip(max_qualified_cardinality));
}

#[test]
fn min_cardinality() {
    let min_cardinality = r#" {"subject": "ex:minCardinality",
                               "predicate": "owl:equivalentClass",
                               "object": {"rdf:type": [{"object": "owl:Restriction", "datatype": "_IRI"}],
                               "owl:onProperty": [{"object": "ex:pMinCardinality", "datatype": "_IRI"}],
                               "owl:minCardinality": [{"object": "1", "datatype" : "xsd:nonNegativeInteger"}]},
                               "datatype" : "_JSONMAP",
                               "annotation" : {},
                               "graph" : "graph",
                               "assertion" : "1",
                               "retraction" : "0" } "#;
    assert!(round_trip(min_cardinality));
}

#[test]
fn min_qualified_cardinality() {
    let min_qualified_cardinality = r#" {"subject": "ex:minQualifiedCardinality",
                                         "predicate": "rdfs:subClassOf",
                                         "object": {"rdf:type": [{"object": "owl:Restriction", "datatype": "_IRI"}],
                                                    "owl:onProperty": [{"object": "ex:pMinQualifiedCardinality", "datatype": "_IRI"}],
                                                    "owl:minQualifiedCardinality": [{"object": "1", "datatype" : "xsd:nonNegativeInteger"}],
                                                    "owl:onClass": [{"object": "ex:minCardinalityFiller", "datatype": "_IRI"}]},
                               "datatype" : "_JSONMAP",
                               "annotation" : {},
                               "graph" : "graph",
                               "assertion" : "1",
                               "retraction" : "0" } "#;

    assert!(round_trip(min_qualified_cardinality));
}

#[test]
fn exact_cardinality_expression() {
    let exact_cardinality = r#" {"subject": "ex:exactCardinality",
                                 "predicate": "rdfs:subClassOf",
                                 "object": {"rdf:type": [{"object": "owl:Restriction", "datatype": "_IRI"}],
                                            "owl:onProperty": [{"object": "ex:pExactCardinality", "datatype": "_IRI"}],
                                            "owl:cardinality": [{"object": "2", "datatype" : "xsd:nonNegativeInteger"}]},
                               "datatype" : "_JSONMAP",
                               "annotation" : {},
                               "graph" : "graph",
                               "assertion" : "1",
                               "retraction" : "0" } "#;

    assert!(round_trip(exact_cardinality));
}

#[test]
fn exact_qualified_cardinality_expression() {
    let exact_qualified_cardinality = r#" {"subject": "ex:exactQualifiedCardinality",
                                           "predicate": "rdfs:subClassOf",
                                           "object": {"rdf:type": [{"object": "owl:Restriction", "datatype": "_IRI"}],
                                                      "owl:onProperty": [{"object": "ex:pExactQualifiedCardinality", "datatype": "_IRI"}],
                                                      "owl:qualifiedCardinality": [{"object": "2", "datatype" : "xsd:nonNegativeInteger"}],
                                                      "owl:onClass": [{"object": "ex:exactQualifiedCardinalityFiller", "datatype" : "_IRI"}]},
                               "datatype" : "_JSONMAP",
                               "annotation" : {},
                               "graph" : "graph",
                               "assertion" : "1",
                               "retraction" : "0" } "#;

    assert!(round_trip(exact_qualified_cardinality));
}

#[test]
fn intersection_expression() {
    let intersection = r#" {"subject": "ex:intersection",
                            "predicate": "rdfs:subClassOf",
                            "object": {"owl:intersectionOf": [{"object" :  [{"object" : "ex:I1", "datatype": "_IRI"},
                                                                            {"object" : "ex:I2", "datatype": "_IRI"},
                                                                            {"object" : "ex:I3", "datatype": "_IRI"}],
                                                              "datatype": "_JSONLIST"}],
                                      "rdf:type": [{"object": "owl:Class", "datatype" : "_IRI"}]},
                           "datatype" : "_JSONMAP",
                           "annotation" : {},
                           "graph" : "graph",
                           "assertion" : "1",
                           "retraction" : "0" } "#;

    assert!(round_trip(intersection));
}

#[test]
fn one_of() {
    let one_of = r#" {"subject": "ex:oneOf",
                      "predicate": "rdfs:subClassOf",
                      "object": {"rdf:type": [{"object": "owl:Class", "datatype": "_IRI"}],
                                 "owl:oneOf": [{"object" : [{"object" : "ex:a1", "datatype": "_IRI"},
                                                            {"object" : "ex:a2", "datatype": "_IRI"},
                                                            {"object" : "ex:a3", "datatype": "_IRI"}],
                                               "datatype": "_JSONLIST"}]},
                           "datatype" : "_JSONMAP",
                           "annotation" : {},
                           "graph" : "graph",
                           "assertion" : "1",
                           "retraction" : "0" } "#;



    assert!(round_trip(one_of));
}

#[test]
fn union_of() {
    let union_of = r#" {"subject": "ex:union",
                        "predicate": "rdfs:subClassOf",
                        "object": {"rdf:type": [{"object": "owl:Class", "datatype": "_IRI"}],
                                   "owl:unionOf": [{"object" : [{"object" : "ex:u1", "datatype": "_IRI"},
                                                                {"object" : "ex:u2", "datatype": "_IRI"},
                                                                {"object" : "ex:u3", "datatype": "_IRI"}],
                                                    "datatype": "_JSONLIST"}]},
                        "datatype" : "_JSONMAP",
                        "annotation" : {},
                        "graph" : "graph",
                        "assertion" : "1",
                        "retraction" : "0" } "#;

    assert!(round_trip(union_of));
}
