use wiring_rs;
use wiring_rs::ofn_2_ldtab as ofn2ldtab;
use serde_json::{Value};

#[test]
fn test_key_order_is_the_same_after_parsing() {

  let s1 = r#"{"owl:onProperty":[{"datatype":"_IRI","object":"obo:RO_0000085"}],
             "owl:someValuesFrom":[{"datatype":"_IRI","object":"obo:OBI_0001043"}],
             "rdf:type":[{"datatype":"_IRI","object":"owl:Restriction"}]}"#;
  let s2 = r#"{"owl:someValuesFrom":[{"datatype":"_IRI","object":"obo:OBI_0001043"}],
             "owl:onProperty":[{"datatype":"_IRI","object":"obo:RO_0000085"}],
             "rdf:type":[{"datatype":"_IRI","object":"owl:Restriction"}]}"#;


  //NB: serde already sorts keys
  let json1 : Value = serde_json::from_str(s1).unwrap();
  let json2 : Value = serde_json::from_str(s2).unwrap();

  assert_eq!(json1, json2);
}

#[test]
fn test_key_order_is_the_same_after_sorting() {

  let s1 = r#"{"owl:onProperty":[{"datatype":"_IRI","object":"obo:RO_0000085"}],
             "owl:someValuesFrom":[{"datatype":"_IRI","object":"obo:OBI_0001043"}],
             "rdf:type":[{"datatype":"_IRI","object":"owl:Restriction"}]}"#;
  let s2 = r#"{"owl:someValuesFrom":[{"datatype":"_IRI","object":"obo:OBI_0001043"}],
             "owl:onProperty":[{"datatype":"_IRI","object":"obo:RO_0000085"}],
             "rdf:type":[{"datatype":"_IRI","object":"owl:Restriction"}]}"#;

  let json1 : Value = serde_json::from_str(s1).unwrap();
  let json2 : Value = serde_json::from_str(s2).unwrap();

  let ss1 = ofn2ldtab::util::sort_value(&json1);
  let ss2 = ofn2ldtab::util::sort_value(&json2);

  assert_eq!(ss1, ss2);
}



#[test]
fn array_order_is_different_after_parsing() {
  let s1 = r#"{"obo:IAO_0010000":[{"datatype":"_IRI","meta":"owl:Axiom","object":"obo:bfo/axiom/033-001"},
                    {"datatype":"_IRI","meta":"owl:Axiom","object":"obo:bfo/axiom/033-002"},
                    {"datatype":"_IRI","meta":"owl:Axiom","object":"obo:bfo/axiom/033-003"}]}"#;
  let s2 = r#"{"obo:IAO_0010000":[{"datatype":"_IRI","meta":"owl:Axiom","object":"obo:bfo/axiom/033-003"},
                    {"datatype":"_IRI","meta":"owl:Axiom","object":"obo:bfo/axiom/033-001"},
                    {"datatype":"_IRI","meta":"owl:Axiom","object":"obo:bfo/axiom/033-002"}]}"#;

  let json1 : Value = serde_json::from_str(s1).unwrap();
  let json2 : Value = serde_json::from_str(s2).unwrap();

  assert_ne!(json1, json2);
}

#[test]
fn array_order_is_the_same_after_sorting() {
  let s1 = r#"{"obo:IAO_0010000":[{"datatype":"_IRI","meta":"owl:Axiom","object":"obo:bfo/axiom/033-001"},
                    {"datatype":"_IRI","meta":"owl:Axiom","object":"obo:bfo/axiom/033-002"},
                    {"datatype":"_IRI","meta":"owl:Axiom","object":"obo:bfo/axiom/033-003"}]}"#;
  let s2 = r#"{"obo:IAO_0010000":[{"datatype":"_IRI","meta":"owl:Axiom","object":"obo:bfo/axiom/033-003"},
                    {"datatype":"_IRI","meta":"owl:Axiom","object":"obo:bfo/axiom/033-001"},
                    {"datatype":"_IRI","meta":"owl:Axiom","object":"obo:bfo/axiom/033-002"}]}"#;

  let json1 : Value = serde_json::from_str(s1).unwrap();
  let json2 : Value = serde_json::from_str(s2).unwrap();

  let ss1 = ofn2ldtab::util::sort_value(&json1);
  let ss2 = ofn2ldtab::util::sort_value(&json2);

  assert_eq!(ss1, ss2); 
}

#[test]
fn array_order_and_key_order_is_different_after_parsing() {

   let s1 = r#"{"owl:onProperty":[{"datatype":"_IRI","object":"obo:RO_0000085"}],
             "owl:someValuesFrom":[{"datatype":"_IRI","object":"obo:OBI_0001043"}],
             "rdf:type":[{"datatype":"_IRI","object":"owl:Restriction"}],
             "obo:IAO_0010000":[{"datatype":"_IRI","meta":"owl:Axiom","object":"obo:bfo/axiom/033-001"},
                                 {"datatype":"_IRI",
                                  "object":"obo:bfo/axiom/033-002",
                                  "meta":"owl:Axiom"},
                                 {"datatype":"_IRI",
                                  "meta":"owl:Axiom",
                                  "object":"obo:bfo/axiom/033-003"}]}"#;

   let s2 = r#"{"owl:someValuesFrom":[{"datatype":"_IRI","object":"obo:OBI_0001043"}],
             "owl:onProperty":[{"datatype":"_IRI","object":"obo:RO_0000085"}],
             "rdf:type":[{"datatype":"_IRI","object":"owl:Restriction"}], 
             "obo:IAO_0010000":[{"datatype":"_IRI","meta":"owl:Axiom","object":"obo:bfo/axiom/033-003"},
                                  {"datatype":"_IRI",
                                   "meta":"owl:Axiom",
                                   "object":"obo:bfo/axiom/033-001"},
                                  {"meta":"owl:Axiom",
                                   "datatype":"_IRI",
                                   "object":"obo:bfo/axiom/033-002"}]}"#;

  let json1 : Value = serde_json::from_str(s1).unwrap();
  let json2 : Value = serde_json::from_str(s2).unwrap();

  assert_ne!(json1, json2);
}

#[test]
fn array_order_and_key_order_are_the_same_after_sorting() {

   let s1 = r#"{"owl:onProperty":[{"datatype":"_IRI","object":"obo:RO_0000085"}],
             "owl:someValuesFrom":[{"datatype":"_IRI","object":"obo:OBI_0001043"}],
             "rdf:type":[{"datatype":"_IRI","object":"owl:Restriction"}],
             "obo:IAO_0010000":[{"datatype":"_IRI","meta":"owl:Axiom","object":"obo:bfo/axiom/033-001"},
                                 {"datatype":"_IRI",
                                  "object":"obo:bfo/axiom/033-002",
                                  "meta":"owl:Axiom"},
                                 {"datatype":"_IRI",
                                  "meta":"owl:Axiom",
                                  "object":"obo:bfo/axiom/033-003"}]}"#;

   let s2 = r#"{"owl:someValuesFrom":[{"datatype":"_IRI","object":"obo:OBI_0001043"}],
             "owl:onProperty":[{"datatype":"_IRI","object":"obo:RO_0000085"}],
             "rdf:type":[{"datatype":"_IRI","object":"owl:Restriction"}], 
             "obo:IAO_0010000":[{"datatype":"_IRI","meta":"owl:Axiom","object":"obo:bfo/axiom/033-003"},
                                  {"datatype":"_IRI",
                                   "meta":"owl:Axiom",
                                   "object":"obo:bfo/axiom/033-001"},
                                  {"meta":"owl:Axiom",
                                   "datatype":"_IRI",
                                   "object":"obo:bfo/axiom/033-002"}]}"#;

  let json1 : Value = serde_json::from_str(s1).unwrap();
  let json2 : Value = serde_json::from_str(s2).unwrap();

  let ss1 = ofn2ldtab::util::sort_value(&json1);
  let ss2 = ofn2ldtab::util::sort_value(&json2);

  assert_eq!(ss1, ss2); 
}


