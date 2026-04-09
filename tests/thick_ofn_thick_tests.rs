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
                           "predicate": "<http://www.w3.org/2000/01/rdf-schema#subClassOf>",
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
    let equivalence = r#" {"subject": "<ldtab:blanknode:f7107cdca26f46c56022cdecca9f2eb6f0538da8a81ae09f419801110a4bd694>",
                           "predicate": "<http://www.w3.org/2002/07/owl#equivalentClass>",
                           "object": {"<http://www.w3.org/2002/07/owl#members>" : [{"object" : [
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
             "predicate": "<http://www.w3.org/2002/07/owl#equivalentClass>",
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
                              "predicate": "<http://www.w3.org/2002/07/owl#disjointUnionOf>",
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
                             "predicate": "<http://www.w3.org/2002/07/owl#equivalentClass>",
                             "object": {"<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>": [{"object": "<http://www.w3.org/2002/07/owl#Class>", "datatype": "_IRI"}],
                                        "<http://www.w3.org/2002/07/owl#complementOf>": [{"object": "ex:complement", "datatype": "_IRI"}]}, 
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
                                       "predicate": "<http://www.w3.org/2000/01/rdf-schema#subClassOf>",
                                       "object": {"<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>": [{"object": "<http://www.w3.org/2002/07/owl#Restriction>", "datatype": "_IRI"}],
                                                  "<http://www.w3.org/2002/07/owl#onProperty>": [{"object": "ex:pExistential", "datatype": "_IRI"}],
                                                  "<http://www.w3.org/2002/07/owl#someValuesFrom>": [{"object": "ex:existentialFiller", "datatype": "_IRI"}]},
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
                        "predicate": "<http://www.w3.org/2000/01/rdf-schema#subClassOf>",
                        "object": {"<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>": [{"object": "<http://www.w3.org/2002/07/owl#Restriction>", "datatype": "_IRI"}],
                                   "<http://www.w3.org/2002/07/owl#onProperty>": [{"object": "ex:pHasSelf", "datatype": "_IRI"}],
                                   "<http://www.w3.org/2002/07/owl#hasSelf>": [{"object": "true", "datatype": "<http://www.w3.org/2001/XMLSchema#boolean>"}]},
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
                         "predicate": "<http://www.w3.org/2000/01/rdf-schema#subClassOf>",
                         "object": {"<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>": [{"object": "<http://www.w3.org/2002/07/owl#Restriction>", "datatype": "_IRI"}],
                                    "<http://www.w3.org/2002/07/owl#onProperty>": [{"object": "ex:pHasValue", "datatype": "_IRI"}],
                                    "<http://www.w3.org/2002/07/owl#hasValue>": [{"object": "ex:a1", "datatype": "_IRI"}]},
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
                               "predicate": "<http://www.w3.org/2000/01/rdf-schema#subClassOf>",
                               "object": {"<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>": [{"object": "<http://www.w3.org/2002/07/owl#Restriction>", "datatype": "_IRI"}],
                                          "<http://www.w3.org/2002/07/owl#onProperty>": [{"object": "ex:pMaxCardinality", "datatype": "_IRI"}],
                                          "<http://www.w3.org/2002/07/owl#maxCardinality>": [{"object": "1", "datatype": "<http://www.w3.org/2001/XMLSchema#nonNegativeInteger>"}]},
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
                                         "predicate": "<http://www.w3.org/2000/01/rdf-schema#subClassOf>",
                                         "object": {"<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>": [{"object": "<http://www.w3.org/2002/07/owl#Restriction>", "datatype": "_IRI"}],
                                                    "<http://www.w3.org/2002/07/owl#onProperty>": [{"object": "ex:pMaxQualifiedCardinality", "datatype": "_IRI"}],
                                                    "<http://www.w3.org/2002/07/owl#maxQualifiedCardinality>": [{"object": "1", "datatype": "<http://www.w3.org/2001/XMLSchema#nonNegativeInteger>"}],
                                                    "<http://www.w3.org/2002/07/owl#onClass>": [{"object": "ex:maxCardinalityFiller", "datatype": "_IRI"}]},

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
                               "predicate": "<http://www.w3.org/2002/07/owl#equivalentClass>",
                               "object": {"<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>": [{"object": "<http://www.w3.org/2002/07/owl#Restriction>", "datatype": "_IRI"}],
                               "<http://www.w3.org/2002/07/owl#onProperty>": [{"object": "ex:pMinCardinality", "datatype": "_IRI"}],
                               "<http://www.w3.org/2002/07/owl#minCardinality>": [{"object": "1", "datatype" : "<http://www.w3.org/2001/XMLSchema#nonNegativeInteger>"}]},
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
                                         "predicate": "<http://www.w3.org/2000/01/rdf-schema#subClassOf>",
                                         "object": {"<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>": [{"object": "<http://www.w3.org/2002/07/owl#Restriction>", "datatype": "_IRI"}],
                                                    "<http://www.w3.org/2002/07/owl#onProperty>": [{"object": "ex:pMinQualifiedCardinality", "datatype": "_IRI"}],
                                                    "<http://www.w3.org/2002/07/owl#minQualifiedCardinality>": [{"object": "1", "datatype" : "<http://www.w3.org/2001/XMLSchema#nonNegativeInteger>"}],
                                                    "<http://www.w3.org/2002/07/owl#onClass>": [{"object": "ex:minCardinalityFiller", "datatype": "_IRI"}]},
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
                                 "predicate": "<http://www.w3.org/2000/01/rdf-schema#subClassOf>",
                                 "object": {"<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>": [{"object": "<http://www.w3.org/2002/07/owl#Restriction>", "datatype": "_IRI"}],
                                            "<http://www.w3.org/2002/07/owl#onProperty>": [{"object": "ex:pExactCardinality", "datatype": "_IRI"}],
                                            "<http://www.w3.org/2002/07/owl#cardinality>": [{"object": "2", "datatype" : "<http://www.w3.org/2001/XMLSchema#nonNegativeInteger>"}]},
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
                                           "predicate": "<http://www.w3.org/2000/01/rdf-schema#subClassOf>",
                                           "object": {"<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>": [{"object": "<http://www.w3.org/2002/07/owl#Restriction>", "datatype": "_IRI"}],
                                                      "<http://www.w3.org/2002/07/owl#onProperty>": [{"object": "ex:pExactQualifiedCardinality", "datatype": "_IRI"}],
                                                      "<http://www.w3.org/2002/07/owl#qualifiedCardinality>": [{"object": "2", "datatype" : "<http://www.w3.org/2001/XMLSchema#nonNegativeInteger>"}],
                                                      "<http://www.w3.org/2002/07/owl#onClass>": [{"object": "ex:exactQualifiedCardinalityFiller", "datatype" : "_IRI"}]},
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
                            "predicate": "<http://www.w3.org/2000/01/rdf-schema#subClassOf>",
                            "object": {"<http://www.w3.org/2002/07/owl#intersectionOf>": [{"object" :  [{"object" : "ex:I1", "datatype": "_IRI"},
                                                                            {"object" : "ex:I2", "datatype": "_IRI"},
                                                                            {"object" : "ex:I3", "datatype": "_IRI"}],
                                                              "datatype": "_JSONLIST"}],
                                      "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>": [{"object": "<http://www.w3.org/2002/07/owl#Class>", "datatype" : "_IRI"}]},
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
                      "predicate": "<http://www.w3.org/2000/01/rdf-schema#subClassOf>",
                      "object": {"<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>": [{"object": "<http://www.w3.org/2002/07/owl#Class>", "datatype": "_IRI"}],
                                 "<http://www.w3.org/2002/07/owl#oneOf>": [{"object" : [{"object" : "ex:a1", "datatype": "_IRI"},
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
                        "predicate": "<http://www.w3.org/2000/01/rdf-schema#subClassOf>",
                        "object": {"<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>": [{"object": "<http://www.w3.org/2002/07/owl#Class>", "datatype": "_IRI"}],
                                   "<http://www.w3.org/2002/07/owl#unionOf>": [{"object" : [{"object" : "ex:u1", "datatype": "_IRI"},
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
