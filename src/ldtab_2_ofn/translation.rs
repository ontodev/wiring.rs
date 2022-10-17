use serde_json::{Value};
use crate::ldtab_2_ofn::axiom_translation as axiom_translation; 
use crate::ldtab_2_ofn::annotation_translation as annotation_translation; 
use crate::util::parser as parser;

/// Given an LDTab ThickTriple (encoded as a string),
/// return its corresponding OFN S-expression encoded as a serde_json::value::Value
/// 
/// #Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::translation as translation;
/// let thick_triple_string = r#"{"subject": "obo:IAO_0000120",
///                               "predicate": "rdfs:subClassOf",
///                               "object": {"owl:someValuesFrom": [{"object": "obo:OBI_0500000",
///                                                                  "datatype":"_iri",
///                                                                  "meta":null}],
///                                          "rdf:type": [{"object": "owl:Restriction",
///                                                        "datatype":"_iri",
///                                                        "meta":null}],
///                                          "owl:onProperty": [{"object": "obo:BFO_0000050",
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
pub fn thick_triple_2_ofn(thick_triple : &Value) -> Value {

    //translate subject, predicate, object into OFN S-expression
    let subject = thick_triple["subject"].to_string();
    let predicate = thick_triple["predicate"].to_string();
    let object = thick_triple["object"].to_string(); 
    let owl = translate_triple(&subject, &predicate, &object);

    //translate annotation
    let annotations = annotation_translation::translate(&thick_triple["annotation"]);

    //merge OFN S-expression with annotations
    let rest = &owl.as_array().unwrap()[1..];

    let mut res = vec![owl[0].clone()];
    for annotation in annotations {
        res.push(annotation.clone());
    } 

    for r in rest {
        res.push(r.clone());
    }

    Value::Array(res)
}

fn translate_triple(subject: &str, predicate: &str, object: &str) -> Value {

    let subject_json = parser::parse_thick_triple_object(subject); 
    let predicate_json = parser::parse_string(predicate); //Assumption: this is a string
    let object_json = parser::parse_thick_triple_object(object); 

    match predicate_json.as_str() {
        "rdfs:subClassOf"  => axiom_translation::translate_subclass_of_axiom(&subject_json, &object_json),
        "owl:equivalentClass" => axiom_translation::translate_equivalent_class(&subject_json, &object_json),
        "owl:AllDisjointClasses" => axiom_translation::translate_disjoint_classes(&object_json),
        "owl:disjointUnionOf" => axiom_translation::translate_disjoint_union(&subject_json,&object_json),
        "owl:disjointWith" => axiom_translation::translate_disjoint_with(&subject_json, &object_json), 
        "rdf:type" => axiom_translation::translate_rdf_type(&subject_json, &object_json),
        "rdfs:domain" => axiom_translation::translate_domain(&subject_json, &object_json),
        "rdfs:range" => axiom_translation::translate_range(&subject_json, &object_json),
        "owl:inverseOf" => axiom_translation:: translate_inverse_object_properties(&subject_json, &object_json),
        "owl:equivalentProperty"  => axiom_translation::translate_equivalent_properties(&subject_json, &object_json),
        "owl:propertyDisjointWith" => axiom_translation::translate_property_disjoint_with(&subject_json, &object_json),
        "owl:AllDisjointProperties" => axiom_translation::translate_all_disjoint_properties(&subject_json, &object_json),
        "rdfs:subPropertyOf" => axiom_translation::translate_sub_property_of(&subject_json, &object_json),
        "owl:AllDifferent" => axiom_translation::translate_all_different(&subject_json, &object_json),
        "owl:differentFrom" => axiom_translation::translate_different_from(&subject_json, &object_json),
        "owl:sameAs" => axiom_translation::translate_same_as(&subject_json, &object_json),
        "owl:AllSameAs" => axiom_translation::translate_all_same_as(&subject_json, &object_json),
        "owl:propertyChainAxiom" => axiom_translation::translate_property_chain(&subject_json, &object_json),
        "owl:NegativePropertyAssertion" => axiom_translation::translate_negative_property_assertion(&subject_json, &object_json),
        "owl:hasKey" => axiom_translation::translate_has_key(&subject_json, &object_json),
        "owl:imports" => axiom_translation::translate_import(&subject_json, &object_json),

        _ => axiom_translation::translate_thin_triple(subject, predicate, object),
    }
}
