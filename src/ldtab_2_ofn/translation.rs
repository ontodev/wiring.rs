use crate::ldtab_2_ofn::annotation_translation;
use crate::ldtab_2_ofn::axiom_translation;
use crate::util::parser;
use serde_json::Value;

/// Given an LDTab ThickTriple (encoded as a string),
/// return its corresponding OFN S-expression encoded as a serde_json::value::Value
///
/// #Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::translation as translation;
/// let thick_triple_string = r#"{"subject": "obo:IAO_0000120",
///                               "predicate": "<http://www.w3.org/2000/01/rdf-schema#subClassOf>",
///                               "object": {"<http://www.w3.org/2002/07/owl#someValuesFrom>": [{"object": "obo:OBI_0500000",
///                                                                  "datatype":"_iri",
///                                                                  "meta":null}],
///                                          "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>": [{"object": "<http://www.w3.org/2002/07/owl#Restriction>",
///                                                        "datatype":"_iri",
///                                                        "meta":null}],
///                                          "<http://www.w3.org/2002/07/owl#onProperty>": [{"object": "obo:BFO_0000050",
///                                                              "datatype":"_iri",
///                                                              "meta":null}]},
///                               "annotation": null,
///                               "assertion":"1",
///                               "graph":"graph",
///                               "retraction":"0",
///                               "datatype":"_iri"}"#;
///
/// let thick_triple = serde_json::from_str(thick_triple_string).unwrap();
///
/// let ofn = translation::thick_triple_2_ofn(&thick_triple);
///
/// let ofn_expected_string =r#"["SubClassOf","obo:IAO_0000120",["SomeValuesFrom","obo:BFO_0000050","obo:OBI_0500000"]]"#;
/// let ofn_expected : Value = serde_json::from_str(ofn_expected_string).unwrap();
///
/// assert_eq!(ofn, ofn_expected);
/// ```
pub fn thick_triple_2_ofn(thick_triple: &Value) -> Value {
    //translate subject, predicate, object into OFN S-expression
    let subject = thick_triple["subject"].to_string();
    let predicate = thick_triple["predicate"].to_string();
    let object = thick_triple["object"].to_string();
    let owl = translate_triple(&subject, &predicate, &object);

    //translate annotation
    let annotations = annotation_translation::translate(&thick_triple["annotation"]);

    match owl {
        Value::Array(mut arr) if !arr.is_empty() => {
            // Pull out the OWL operator, then merge annotations and the rest
            let head = arr.remove(0);
            let mut merged = Vec::with_capacity(1 + annotations.len() + arr.len());
            merged.push(head);
            merged.extend(annotations);
            merged.extend(arr);

            Value::Array(merged)
        }
        _ => {//TODO: error handling
            owl
        }
    }
}

fn translate_triple(subject: &str, predicate: &str, object: &str) -> Value {
    let subject_json = parser::parse_thick_triple_object(subject);
    let predicate_json = parser::parse_string(predicate); //Assumption: this is a string
    let object_json = parser::parse_thick_triple_object(object);

    match predicate_json.as_str() {
        "<http://www.w3.org/2000/01/rdf-schema#subClassOf>" => {
            axiom_translation::translate_subclass_of_axiom(&subject_json, &object_json)
        }
        "<http://www.w3.org/2002/07/owl#equivalentClass>" => {
            axiom_translation::translate_equivalent_class(&subject_json, &object_json)
        }
        "<http://www.w3.org/2002/07/owl#AllDisjointClasses>" => axiom_translation::translate_disjoint_classes(&object_json),
        "<http://www.w3.org/2002/07/owl#disjointUnionOf>" => {
            axiom_translation::translate_disjoint_union(&subject_json, &object_json)
        }
        "<http://www.w3.org/2002/07/owl#disjointWith>" => {
            axiom_translation::translate_disjoint_with(&subject_json, &object_json)
        }
        "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>" => axiom_translation::translate_rdf_type(&subject_json, &object_json),
        "<http://www.w3.org/2000/01/rdf-schema#domain>" => axiom_translation::translate_domain(&subject_json, &object_json),
        "<http://www.w3.org/2000/01/rdf-schema#range>" => axiom_translation::translate_range(&subject_json, &object_json),
        "<http://www.w3.org/2002/07/owl#inverseOf>" => {
            axiom_translation::translate_inverse_object_properties(&subject_json, &object_json)
        }
        "<http://www.w3.org/2002/07/owl#equivalentProperty>" => {
            axiom_translation::translate_equivalent_properties(&subject_json, &object_json)
        }
        "<http://www.w3.org/2002/07/owl#propertyDisjointWith>" => {
            axiom_translation::translate_property_disjoint_with(&subject_json, &object_json)
        }
        "<http://www.w3.org/2002/07/owl#AllDisjointProperties>" => {
            axiom_translation::translate_all_disjoint_properties(&subject_json, &object_json)
        }
        "<http://www.w3.org/2000/01/rdf-schema#subPropertyOf>" => {
            axiom_translation::translate_sub_property_of(&subject_json, &object_json)
        }
        "<http://www.w3.org/2002/07/owl#AllDifferent>" => {
            axiom_translation::translate_all_different(&subject_json, &object_json)
        }
        "<http://www.w3.org/2002/07/owl#differentFrom>" => {
            axiom_translation::translate_different_from(&subject_json, &object_json)
        }
        "<http://www.w3.org/2002/07/owl#sameAs>" => axiom_translation::translate_same_as(&subject_json, &object_json),
        "<http://www.w3.org/2002/07/owl#AllSameAs>" => axiom_translation::translate_all_same_as(&subject_json, &object_json),
        "<http://www.w3.org/2002/07/owl#propertyChainAxiom>" => {
            axiom_translation::translate_property_chain(&subject_json, &object_json)
        }
        "<http://www.w3.org/2002/07/owl#NegativePropertyAssertion>" => {
            axiom_translation::translate_negative_property_assertion(&subject_json, &object_json)
        }
        "<http://www.w3.org/2002/07/owl#hasKey>" => axiom_translation::translate_has_key(&subject_json, &object_json),
        "<http://www.w3.org/2002/07/owl#imports>" => axiom_translation::translate_import(&subject_json, &object_json),

        _ => axiom_translation::translate_thin_triple(subject, predicate, object),
    }
}
