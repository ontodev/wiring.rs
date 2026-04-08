use crate::ofn_2_ldtab::annotation_translation;
use crate::ofn_2_ldtab::class_translation;
use crate::constants::*;
use crate::ofn_2_ldtab::property_translation;
use crate::ofn_2_ldtab::rule_translation;
use crate::ofn_2_ldtab::util;
use serde_json::json;
use serde_json::Value;


/// Splits an annotated axiom into its logical part and translated annotations.
/// Returns `(owl, annotation)` where `owl` is the axiom with annotations stripped,
/// and `annotation` is the JSON representation of the annotations.
fn split_axiom(v: &Value) -> (Value, Value) {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);
    let annotation = annotation_translation::translate_annotations(&annotations);
    (owl, annotation)
}


pub fn translate_declaration(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let unwrapped_declaration = owl[1].clone();

    match unwrapped_declaration[0].as_str() {
        Some("Class") => translate_class_declaration(v),
        Some("ObjectProperty") => translate_object_property_declaration(v),
        Some("DataProperty") => translate_data_property_declaration(v),
        Some("AnnotationProperty") => translate_annotation_property_declaration(v),
        Some("NamedIndividual") => translate_individual_declaration(v),
        Some("Datatype") => translate_datatype_declaration(v),
        Some(_) => panic!("Not a valid declaration"),
        None => panic!("Not a valid declaration"),
    }
}

pub fn translate_class_declaration(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    //unwrap declaration
    let unwrapped_declaration = owl[1].clone();

    //translate OWL classes
    let class = class_translation::translate(&unwrapped_declaration[1]);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":class,
    "predicate":RDF_TYPE,
    "object":OWL_CLASS,
    "datatype":LDTAB_IRI,
    "annotation":annotation
    });
    triple
}

pub fn translate_ontology_import(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let subject = class_translation::translate(&owl[1]);
    let object = class_translation::translate(&owl[2]);

    json!({"assertion":"1",
        "retraction":"0",
        "graph":"graph",
        "subject": subject,
        "predicate":OWL_IMPORTS,
        "object": object,
        "datatype":LDTAB_IRI,
        "annotation":annotation
    })
}

pub fn translate_ontology_annotation(v: &Value) -> Value {
    let subject = class_translation::translate(&v[1]);
    let predicate = property_translation::translate(&v[2][1]);
    let annotation = annotation_translation::translate_value(&v[2][2]);
    let object = annotation.get("object").unwrap().clone();
    let datatype = annotation.get("datatype").unwrap().clone();

    json!({"assertion":"1",
        "retraction":"0",
        "graph":"graph",
        "subject": subject,
        "predicate":predicate,
        "object":object,
        "datatype":datatype,
        "annotation":""
    })
}

pub fn translate_class_assertion_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    //translate OWL classes
    let class = class_translation::translate(&owl[1]);
    let individual = class_translation::translate(&owl[2]);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":individual,
    "predicate":RDF_TYPE,
    "object":class,
    "datatype":LDTAB_IRI,
    "annotation":annotation
    });
    triple
}

pub fn translate_object_property_assertion_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    //translate OWL classes
    let property = property_translation::translate(&owl[1]);
    let from = class_translation::translate(&owl[2]);
    let to = class_translation::translate(&owl[3]);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":from,
    "predicate":property,
    "object":to,
    "datatype":LDTAB_IRI,
    "annotation":annotation
    });
    triple
}

pub fn translate_data_property_assertion_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    //translate OWL classes
    let property = property_translation::translate(&owl[1]);
    let from = class_translation::translate(&owl[2]);

    let to: Value = if let Some(s) = owl[3].as_str() {
    if let Some((literal, datatype)) = s.split_once("^^") {
        json!({ "object": unquote_once(literal), "datatype": datatype })
    } else if let Some((literal, language)) = s.split_once('@') {
        json!({ "object": unquote_once(literal), "datatype": format!("@{language}") })
    } else {
        json!({ "object": unquote_once(s), "datatype": XSD_STRING })
    }
} else {
        Value::Null
    };

    let literal = to.get("object").unwrap().as_str().unwrap();
    let datatype = to.get("datatype").unwrap().as_str().unwrap();


    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":from,
    "predicate":property,
    "object":literal,
    "datatype":datatype,
    "annotation":annotation
    });
    triple
}

pub fn translate_negative_object_property_assertion_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    //translate OWL classes
    let property = property_translation::translate(&owl[1]);
    let from = class_translation::translate(&owl[2]);
    let to = class_translation::translate(&owl[3]);

    //TODO reuse blank node object
    let blank_node = json!({RDF_TYPE:[{"object" : OWL_NEGATIVE_PROPERTY_ASSERTION, "datatype" : LDTAB_IRI}],
                            OWL_SOURCE_INDIVIDUAL:[{"object":from, "datatype":LDTAB_IRI}],
                            OWL_ASSERTION_PROPERTY:[{"object":property, "datatype":LDTAB_IRI}],
                            OWL_TARGET_INDIVIDUAL:[{"object":to, "datatype":LDTAB_IRI}]});

    let blank_node_id = util::generate_blank_node_id(&blank_node);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":blank_node_id,
    "predicate":OWL_NEGATIVE_PROPERTY_ASSERTION,
    "object": {RDF_TYPE:[{"object" : OWL_NEGATIVE_PROPERTY_ASSERTION, "datatype" : LDTAB_IRI}],
               OWL_SOURCE_INDIVIDUAL:[{"object":from, "datatype":LDTAB_IRI}],
               OWL_ASSERTION_PROPERTY:[{"object":property, "datatype":LDTAB_IRI}],
               OWL_TARGET_INDIVIDUAL:[{"object":to, "datatype":LDTAB_IRI}]},
    "datatype":LDTAB_JSON_MAP,
    "annotation":annotation
    });
    triple
}

fn unquote_once(s: &str) -> &str {
    s.strip_prefix('"')
        .and_then(|t| t.strip_suffix('"'))
        .unwrap_or(s)
}

pub fn translate_negative_data_property_assertion_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    //translate OWL classes
    let property = property_translation::translate(&owl[1]);
    let from = class_translation::translate(&owl[2]);

    //TODO: handle data values properly
    let to: Value = if let Some(s) = owl[3].as_str() {
    if let Some((literal, datatype)) = s.split_once("^^") {
        json!({ "object": unquote_once(literal), "datatype": datatype })
    } else if let Some((literal, language)) = s.split_once('@') {
        json!({ "object": unquote_once(literal), "datatype": format!("@{language}") })
    } else {
        json!({ "object": unquote_once(s), "datatype": XSD_STRING })
    }
} else {
        Value::Null
    };

    let literal = to.get("object").unwrap().as_str().unwrap();
    let datatype = to.get("datatype").unwrap().as_str().unwrap();

    let blank_node = json!({RDF_TYPE:[{"object" : OWL_NEGATIVE_PROPERTY_ASSERTION, "datatype" : LDTAB_IRI}],
                            OWL_SOURCE_INDIVIDUAL:[{"object":from, "datatype":LDTAB_IRI}],
                            OWL_ASSERTION_PROPERTY:[{"object":property, "datatype":LDTAB_IRI}],
                            OWL_TARGET_VALUE:[{"object":literal, "datatype":datatype }]});

    let blank_node_id = util::generate_blank_node_id(&blank_node);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":blank_node_id,
    "predicate":OWL_NEGATIVE_PROPERTY_ASSERTION,
    "object":{ RDF_TYPE:[{"object" : OWL_NEGATIVE_PROPERTY_ASSERTION, "datatype" : LDTAB_IRI}],
               OWL_SOURCE_INDIVIDUAL:[{"object":from, "datatype":LDTAB_IRI}],
               OWL_ASSERTION_PROPERTY:[{"object":property, "datatype":LDTAB_IRI}],
               OWL_TARGET_VALUE:[{"object":literal, "datatype":datatype }]},
    "datatype":LDTAB_JSON_MAP,
    "annotation":annotation
    });
    triple
}

pub fn translate_same_individuals_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let number_of_operands = (owl.as_array().unwrap())[1..].len();

    if number_of_operands == 2 {
        //TODO check that class_translation supports individuals
        let lhs = class_translation::translate(&owl[1]);
        let rhs = class_translation::translate(&owl[2]);

        let triple = json!({
                        "assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":lhs,
                        "predicate":OWL_SAME_AS,
                        "object":rhs, 
                        "datatype":util::translate_datatype(&json!(rhs)), 
                        "annotation":annotation});
        triple
    } else {
        let operands: Value = class_translation::translate_list(&(owl.as_array().unwrap())[1..]);

        //NB: IRIs are not expanded by wiring - this is LDTab's responsibility
        let blank_node = json!({"predicate":OWL_ALL_SAME_AS,
                                "object": {OWL_MEMBERS:[{"object":operands, "datatype":LDTAB_JSON_LIST}]},
                                "datatype":LDTAB_JSON_MAP});

        let blank_node_id = util::generate_blank_node_id(&blank_node);

        let triple = json!({"assertion":"1",
                            "retraction":"0",
                            "graph":"graph", //TODO
                            "subject":blank_node_id,
                            //"predicate":OWL_SAME_AS, 
                            "predicate":OWL_ALL_SAME_AS, //this is LDtab specific
                            "object": {OWL_MEMBERS:[{"object":operands, "datatype":LDTAB_JSON_LIST}]}, //TODO remove datatype
                            "datatype":LDTAB_JSON_MAP,
                            "annotation":annotation});
        triple
    }
}

pub fn translate_different_individuals_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let number_of_operands = (owl.as_array().unwrap())[1..].len();
    if number_of_operands == 2 {
        let lhs = class_translation::translate(&owl[1]);
        let rhs = class_translation::translate(&owl[2]);

        let triple = json!({
                        "assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":lhs,
                        "predicate":OWL_DIFFERENT_FROM,
                        "object":rhs, 
                        "datatype":util::translate_datatype(&json!(rhs)), 
                        "annotation":annotation});
        triple
    } else {
        let operands: Value = class_translation::translate_list(&(owl.as_array().unwrap())[1..]);

        //TODO: this object should be reused
        let blank_node = json!({OWL_DISTINCT_MEMBERS:[{"object":operands, "datatype":LDTAB_JSON_LIST}],
                                RDF_TYPE:[{"datatype":LDTAB_IRI,"object":OWL_ALL_DIFFERENT}]});

        let blank_node_id = util::generate_blank_node_id(&blank_node);

        let triple = json!({"assertion":"1",
                            "retraction":"0",
                            "graph":"graph", //TODO
                            "subject":blank_node_id,
                            "predicate":OWL_ALL_DIFFERENT, 
                            "object": {OWL_DISTINCT_MEMBERS:[{"object":operands, "datatype":LDTAB_JSON_LIST}],
                                       RDF_TYPE:[{"datatype":LDTAB_IRI,"object":OWL_ALL_DIFFERENT}]},
                            "datatype":LDTAB_JSON_MAP,
                            "annotation":annotation});
        triple
    }
}

pub fn translate_object_property_declaration(v: &Value) -> Value {
    //split annotations from logical structure
    let (owl, annotation) = split_axiom(v);

    //unwrap declaration
    let unwrapped_declaration = owl[1].clone();

    let property = property_translation::translate(&unwrapped_declaration[1]);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":property,
    "predicate":RDF_TYPE,
    "object":OWL_OBJECT_PROPERTY,
    "datatype":LDTAB_IRI,
    "annotation":annotation
    });
    triple
}

pub fn translate_data_property_declaration(v: &Value) -> Value {
    //split annotations from logical structure
    let (owl, annotation) = split_axiom(v);

    //unwrap declaration
    let unwrapped_declaration = owl[1].clone();

    let property = property_translation::translate(&unwrapped_declaration[1]);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":property,
    "predicate":RDF_TYPE,
    "object":OWL_DATATYPE_PROPERTY,
    "datatype":LDTAB_IRI,
    "annotation":annotation
    });
    triple
}

pub fn translate_annotation_property_declaration(v: &Value) -> Value {
    //split annotations from logical structure
    let (owl, annotation) = split_axiom(v);

    //unwrap declaration
    let unwrapped_declaration = owl[1].clone();

    let property = property_translation::translate(&unwrapped_declaration[1]);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":property,
    "predicate":RDF_TYPE,
    "object":OWL_ANNOTATION_PROPERTY,
    "datatype":LDTAB_IRI,
    "annotation":annotation
    });
    triple
}

//TODO: test this
pub fn translate_datatype_definition(v: &Value) -> Value {
    //split annotations from logical structure
    let (owl, annotation) = split_axiom(v);

    //TODO: check this (should just be a string)
    let lhs = owl[1].clone();
    let rhs = class_translation::translate(&owl[2].clone()); //this is a datarange

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":lhs,
    "predicate":OWL_EQUIVALENT_CLASS,
    "object":rhs,
    "datatype":LDTAB_IRI,
    "annotation":annotation
    });
    triple
}

pub fn translate_datatype_declaration(v: &Value) -> Value {
    //split annotations from logical structure
    let (owl, annotation) = split_axiom(v);

    //unwrap declaration
    let unwrapped_declaration = owl[1].clone();

    //TODO: check this (should just be a string)
    let datatype = unwrapped_declaration[1].clone();

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":datatype,
    "predicate":RDF_TYPE,
    "object":RDFS_DATATYPE,
    "datatype":LDTAB_IRI,
    "annotation":annotation
    });
    triple
}

pub fn translate_individual_declaration(v: &Value) -> Value {
    //split annotations from logical structure
    let (owl, annotation) = split_axiom(v);

    //unwrap declaration
    let unwrapped_declaration = owl[1].clone();

    //TODO: check this (should just be a string)
    let individual = unwrapped_declaration[1].clone();

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":individual,
    "predicate":RDF_TYPE,
    "object":OWL_NAMED_INDIVIDUAL,
    "datatype":LDTAB_IRI,
    "annotation":annotation
    });
    triple
}

pub fn translate_sub_object_property(v: &Value) -> Value {
    //split annotations from logical structure
    let (owl, annotation) = split_axiom(v);

    //SubObjectPropertyOf( ObjectPropertyChain( OPE1 ... OPEn ) OPE )
    //is translated as
    //T(OPE) owl:propertyChainAxiom T(SEQ OPE1 ... OPEn) .
    //so, we need to check here
    //whether the SubObjectPropertyOf has an ObjectPropertyChain argument
    if owl[1].is_array() && owl[1][0].as_str().unwrap().eq("ObjectPropertyChain") {
        let sub = property_translation::translate_list(&(owl[1].as_array().unwrap())[1..]);
        let sup = property_translation::translate(&owl[2]);

        json!({ "assertion":"1",
            "retraction":"0",
            "graph":"graph",
            "subject":sup,
            "predicate":OWL_PROPERTY_CHAIN_AXIOM,
            "object":sub,
            "datatype":LDTAB_JSON_LIST,
            "annotation":annotation
        })
    } else {
        let sub = property_translation::translate(&owl[1]);
        let sup = property_translation::translate(&owl[2]);

        json!({ "assertion":"1",
            "retraction":"0",
            "graph":"graph",
            "subject":sub,
            "predicate":RDFS_SUB_PROPERTY_OF,
            "object":sup,
            "datatype":util::translate_datatype(&json!(sup)),
            "annotation":annotation
        })
    }
}

pub fn translate_sub_data_property(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let sub = property_translation::translate(&owl[1]);
    let sup = property_translation::translate(&owl[2]);

    json!({ "assertion":"1",
        "retraction":"0",
        "graph":"graph",
        "subject":sub,
        "predicate":RDFS_SUB_PROPERTY_OF,
        "object":sup,
        "datatype":util::translate_datatype(&json!(sup)),
        "annotation":annotation
    })
}

pub fn translate_subclass_of_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let subclass = class_translation::translate(&owl[1]);
    let superclass = class_translation::translate(&owl[2]);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":subclass,
    "predicate":RDFS_SUB_CLASS_OF,
    "object":superclass,
    "datatype":util::translate_datatype(&json!(superclass)),
    "annotation":annotation
    });
    triple
}

pub fn translate_sub_annotation_property_of_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let lhs = property_translation::translate(&owl[1]);
    let rhs = property_translation::translate(&owl[2]);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":lhs,
    "predicate":RDFS_SUB_PROPERTY_OF,
    "object":rhs,
    "datatype":util::translate_datatype(&json!(rhs)),
    "annotation":annotation
    });
    triple
}

pub fn translate_disjoint_classes_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    if (owl.as_array().unwrap())[1..].len() == 2 {
        let lhs = class_translation::translate(&owl.as_array().unwrap()[1]);
        let rhs = class_translation::translate(&owl.as_array().unwrap()[2]);
        let annotation = annotation_translation::translate_annotations(&annotations);

        let triple = json!({"assertion":"1",
                            "retraction":"0",
                            "graph":"graph",
                            "subject":lhs,
                            "predicate":OWL_DISJOINT_WITH,
                            "object": rhs,
                            "datatype":util::translate_datatype(&json!(rhs)),
                            "annotation":annotation});
        triple
    } else {
        let operands: Value = class_translation::translate_list(&(owl.as_array().unwrap())[1..]);
        let annotation = annotation_translation::translate_annotations(&annotations);

        let blank_node = json!({OWL_MEMBERS:[{"object":operands, "datatype":LDTAB_JSON_LIST}],RDF_TYPE:[{"datatype":LDTAB_IRI,"object":OWL_ALL_DISJOINT_CLASSES}]});

        //"annotation":annotation});

        let blank_node_id = util::generate_blank_node_id(&blank_node);

        let triple = json!({"assertion":"1",
                            "retraction":"0",
                            "graph":"graph",
                            "subject":blank_node_id,
                            "predicate":OWL_ALL_DISJOINT_CLASSES,
                            "object": {OWL_MEMBERS:[{"object":operands, "datatype":LDTAB_JSON_LIST}],RDF_TYPE:[{"datatype":LDTAB_IRI,"object":OWL_ALL_DISJOINT_CLASSES}]},
                            "datatype": LDTAB_JSON_MAP, 
                            "annotation":annotation});
        triple
    }
}

pub fn translate_disjoint_union_of_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let lhs = class_translation::translate(&owl[1]);
    let operands: Value = class_translation::translate_list(&(owl.as_array().unwrap())[2..]);

    let triple = json!({
                        "assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":lhs,
                        "predicate":OWL_DISJOINT_UNION_OF,
                        "object":operands,
                        "datatype": LDTAB_JSON_LIST, 
                        "annotation":annotation});
    triple
}

//TODO:: equivalent classe  (we have a custom encoding for this and need a case distinction
//between binary axioms and n-ary axioms)
pub fn translate_equivalent_classes_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let number_of_operands = (owl.as_array().unwrap())[1..].len();
    if number_of_operands == 2 {
        let lhs = class_translation::translate(&owl[1]);
        let rhs = class_translation::translate(&owl[2]);

        let triple = json!({
                        "assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":lhs,
                        "predicate":OWL_EQUIVALENT_CLASS,
                        "object":rhs, 
                        "datatype":util::translate_datatype(&json!(rhs)), 
                        "annotation":annotation});
        triple
    } else {
        let operands: Value = class_translation::translate_list(&(owl.as_array().unwrap())[1..]);

        let blank_node = json!({"predicate":OWL_EQUIVALENT_CLASS,
                                "object": {OWL_MEMBERS:[{"object":operands, "datatype":LDTAB_JSON_LIST}]},
                                "datatype":LDTAB_JSON_MAP});

        let blank_node_id = util::generate_blank_node_id(&blank_node);

        let triple = json!({"assertion":"1",
                            "retraction":"0",
                            "graph":"graph", //TODO
                            "subject":blank_node_id,
                            "predicate":OWL_EQUIVALENT_CLASS,
                            "object": {OWL_MEMBERS:[{"object":operands, "datatype":LDTAB_JSON_LIST}]}, //TODO remove datatype 
                            "datatype":LDTAB_JSON_MAP,
                            "annotation":annotation});
        triple
    }
}

pub fn translate_object_property_domain_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let property = property_translation::translate(&owl[1]);
    let domain = class_translation::translate(&owl[2]);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":property,
                        "predicate":RDFS_DOMAIN,
                        "object":domain,
                        "datatype": util::translate_datatype(&json!(domain)),
                        "annotation":annotation});
    triple
}

pub fn translate_object_property_range_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let property = property_translation::translate(&owl[1]);
    let range = class_translation::translate(&owl[2]);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":property,
                        "predicate":RDFS_RANGE,
                        "object":range,
                        "datatype": util::translate_datatype(&json!(range)),
                        "annotation":annotation});
    triple
}

pub fn translate_annotation_property_domain_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let property = property_translation::translate(&owl[1]);
    let domain = class_translation::translate(&owl[2]); //TODO IRI

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":property,
                        "predicate":RDFS_DOMAIN,
                        "object":domain,
                        "datatype": util::translate_datatype(&json!(domain)),
                        "annotation":annotation});
    triple
}

pub fn translate_annotation_property_range_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let property = property_translation::translate(&owl[1]);
    let range = class_translation::translate(&owl[2]);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":property,
                        "predicate":RDFS_RANGE,
                        "object":range,
                        "datatype": util::translate_datatype(&json!(range)),
                        "annotation":annotation});
    triple
}

//TODO test n-ary case
pub fn translate_equivalent_properties_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let number_of_operands = (owl.as_array().unwrap())[1..].len();
    if number_of_operands == 2 {
        let lhs = property_translation::translate(&owl[1]);
        let rhs = property_translation::translate(&owl[2]);

        let triple = json!({
                        "assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":lhs,
                        "predicate":OWL_EQUIVALENT_PROPERTY,
                        "object":rhs, 
                        "datatype":util::translate_datatype(&json!(rhs)), 
                        "annotation":annotation});
        triple
    } else {
        let operands: Value = property_translation::translate_list(&(owl.as_array().unwrap())[1..]);

        let blank_node = json!({"predicate":OWL_EQUIVALENT_PROPERTY,
                                "object": {OWL_MEMBERS:[{"object":operands, "datatype":LDTAB_JSON_LIST}]},
                                "datatype":LDTAB_JSON_MAP});

        let blank_node_id = util::generate_blank_node_id(&blank_node);

        let triple = json!({"assertion":"1",
                            "retraction":"0",
                            "graph":"graph", //TODO
                            "subject":blank_node_id,
                            "predicate":OWL_EQUIVALENT_PROPERTY, //TODO AllEquivalentProperties?
                            "object": {OWL_MEMBERS:[{"object":operands, "datatype":LDTAB_JSON_LIST}]}, //TODO remove datatype
                            "datatype":LDTAB_JSON_MAP,
                            "annotation":annotation});
        triple
    }
}

pub fn translate_data_property_domain_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let property = property_translation::translate(&owl[1]);
    let domain = class_translation::translate(&owl[2]);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":property,
                        "predicate":RDFS_DOMAIN,
                        "object":domain,
                        "datatype": util::translate_datatype(&json!(domain)),
                        "annotation":annotation});
    triple
}

pub fn translate_inverse_properties_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let lhs = property_translation::translate(&owl[1]);
    let rhs = property_translation::translate(&owl[2]);
    let datatype = util::translate_datatype(&json!(rhs));

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":lhs,
                        "predicate":OWL_INVERSE_OF,
                        "object":rhs,
                        "datatype": datatype,
                        "annotation":annotation});
    triple
}

pub fn translate_functional_property_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let argument = property_translation::translate(&owl[1]);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":argument,
                        "predicate":RDF_TYPE,
                        "object":OWL_FUNCTIONAL_PROPERTY,
                        "datatype": util::translate_datatype(&json!(argument)),
                        "annotation":annotation});
    triple
}

pub fn translate_inverse_functional_object_property_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let argument = property_translation::translate(&owl[1]);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":argument,
                        "predicate":RDF_TYPE,
                        "object":OWL_INVERSE_FUNCTIONAL_PROPERTY,
                        "datatype": util::translate_datatype(&json!(argument)),
                        "annotation":annotation});
    triple
}

pub fn translate_reflexive_object_property_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let argument = property_translation::translate(&owl[1]);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":argument,
                        "predicate":RDF_TYPE,
                        "object":OWL_REFLECTIVE_PROPERTY,
                        "datatype": util::translate_datatype(&json!(argument)),
                        "annotation":annotation});
    triple
}

pub fn translate_irreflexive_object_property_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let argument = property_translation::translate(&owl[1]);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":argument,
                        "predicate":RDF_TYPE,
                        "object":OWL_IRREFLEXIVE_PROPERTY,
                        "datatype": util::translate_datatype(&json!(argument)),
                        "annotation":annotation});
    triple
}

pub fn translate_symmetric_object_property_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let argument = property_translation::translate(&owl[1]);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":argument,
                        "predicate":RDF_TYPE,
                        "object":OWL_SYMMETRIC_PROPERTY,
                        "datatype": util::translate_datatype(&json!(argument)),
                        "annotation":annotation});
    triple
}

pub fn translate_asymmetric_object_property_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let argument = property_translation::translate(&owl[1]);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":argument,
                        "predicate":RDF_TYPE,
                        "object":OWL_ASYMMETRIC_PROPERTY,
                        "datatype": util::translate_datatype(&json!(argument)),
                        "annotation":annotation});
    triple
}

pub fn translate_transitive_object_property_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let argument = property_translation::translate(&owl[1]);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":argument,
                        "predicate":RDF_TYPE,
                        "object":OWL_TRANSITIVE_PROPERTY,
                        "datatype": util::translate_datatype(&json!(argument)),
                        "annotation":annotation});
    triple
}

pub fn translate_data_property_range_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let property = property_translation::translate(&owl[1]);
    let range = class_translation::translate(&owl[2]);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":property,
                        "predicate":RDFS_RANGE,
                        "object":range,
                        "datatype": util::translate_datatype(&json!(range)),
                        "annotation":annotation});
    triple
}

pub fn translate_disjoint_properties_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let number_of_operands = (owl.as_array().unwrap())[1..].len();
    if number_of_operands == 2 {
        let lhs = property_translation::translate(&owl[1]);
        let rhs = property_translation::translate(&owl[2]);

        let triple = json!({
                        "assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":lhs,
                        "predicate":OWL_PROPERTY_DISJOINT_WITH,
                        "object":rhs, 
                        "datatype":util::translate_datatype(&json!(rhs)), 
                        "annotation":annotation});
        triple
    } else {
        let operands: Value = property_translation::translate_list(&(owl.as_array().unwrap())[1..]);

        let blank_node = json!({"predicate":OWL_ALL_DISJOINT_PROPERTIES,
                                "object": {OWL_MEMBERS:[{"object":operands, "datatype":LDTAB_JSON_LIST}]},
                                "datatype":LDTAB_JSON_MAP});

        let blank_node_id = util::generate_blank_node_id(&blank_node);


        let triple = json!({"assertion":"1",
                            "retraction":"0",
                            "graph":"graph", //TODO
                            "subject":blank_node_id,
                            "predicate":OWL_ALL_DISJOINT_PROPERTIES, 
                            "object": {OWL_MEMBERS:[{"object":operands, "datatype":LDTAB_JSON_LIST}]}, //TODO remove datatype
                            "datatype":LDTAB_JSON_MAP,
                            "annotation":annotation});
        triple
    }
}

pub fn translate_has_key_axiom(v: &Value) -> Value {
    let (owl, annotation) = split_axiom(v);

    let class = class_translation::translate(&owl[1]);
    let ops = property_translation::translate_list(&owl[2].as_array().unwrap());
    let dps = property_translation::translate_list(&owl[3].as_array().unwrap());

    let mut operands = ops;
    if !dps.is_null() {
        if operands.is_null() {
            operands = dps;
        } else if operands.is_array() && dps.is_array() {
            let mut ops_array = operands.as_array().unwrap().clone();
            let dps_array = dps.as_array().unwrap();
            ops_array.extend(dps_array.iter().cloned());
            operands = Value::Array(ops_array);
        }
    }

    //let operands: Value = property_translation::translate_list(&(owl.as_array().unwrap())[1..]);


    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":class,
                        "predicate":OWL_HAS_KEY,
                        "object": operands,
                        "datatype": LDTAB_JSON_LIST, 
                        "annotation":annotation});
    triple
}

pub fn translate_annotation_assertion_axiom(v: &Value) -> Value {
    //TODO: check order
    let (owl, annotation) = split_axiom(v);

    //if annotation_translation::has_annotation(v) {
    //    println!("Input: {:?}",v);
    //    println!("annotations: {:?}",annotations);
    //    println!("annotation: {:?}",annotation);
    //}

    //translate OWL classes - these are not necessarily classes though..
    let from = class_translation::translate(&owl[2]); //subject (class or individual)
    let property = property_translation::translate(&owl[1]);

    if annotation_translation::is_literal(&owl[3]) {
        let to = annotation_translation::translate_literal(&owl[3].as_str().unwrap());
        let object = to.get("object").unwrap();
        let datatype = to.get("datatype").unwrap();

        let triple = json!({
        "assertion":"1",
        "retraction":"0",
        "graph":"graph",
        "subject":from,
        "predicate":property,
        "object":object,
        "datatype":datatype,
        "annotation":annotation
        });
        triple
    } else {
        let to = class_translation::translate(&owl[3]);

        let triple = json!({
        "assertion":"1",
        "retraction":"0",
        "graph":"graph",
        "subject":from,
        "predicate":property,
        "object":to,
        "datatype":LDTAB_IRI,
        "annotation":annotation
        });
        triple
    }
}

fn merge_json(a: &mut Value, b: Value) {
    match (a, b) {
        (Value::Object(a_map), Value::Object(b_map)) => {
            for (k, v) in b_map {
                // Merge if key exists, otherwise insert
                merge_json(a_map.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            // If `b` isn't an object (or `a` isn't an object),
            // just overwrite `a` with `b`.
            *a = b;
        }
    }
}

fn remove_meta(value: &mut Value) {
    match value {
        // If it's an object, we look for (key="meta", value="owl:Axiom")
        Value::Object(map) => {
            // First, remove all "meta" keys that have the value "owl:Axiom"
            let mut keys_to_remove = Vec::new();
            for (k, v) in map.iter() {
                if k == "meta" {
                    keys_to_remove.push(k.clone());
                }
            }
            for k in keys_to_remove {
                map.remove(&k);
            }

            // Then, recurse into any sub-values
            for (_, v) in map.iter_mut() {
                remove_meta(v);
            }
        }
        // If it's an array, just recurse on each element
        Value::Array(arr) => {
            for item in arr {
                remove_meta(item);
            }
        }
        // Otherwise, it's a primitive (string, number, bool, null). No action needed.
        _ => {}
    }
}

pub fn translate_rule(v: &Value) -> Value {

    let owl = annotation_translation::get_owl(v);
    let ofn_annotations = annotation_translation::get_annotations(v);
    let mut annotation = annotation_translation::translate_annotations(&ofn_annotations);



    let mut triples = Vec::new();

    let body = rule_translation::translate(&owl[1]);
    let head = rule_translation::translate(&owl[2]);

    remove_meta(&mut annotation);
    let mut anno_blan = annotation.clone();

    if let Value::Object(ref mut map) = anno_blan {
        map.remove("meta");
    }

    let mut blank_node = json!({RDF_TYPE:[{"datatype":LDTAB_IRI, "object" :SWRL_IMP}],
                            SWRL_BODY:[{"datatype":LDTAB_JSON_LIST, "object" : body}],
                            SWRL_HEAD:[{"datatype":LDTAB_JSON_LIST, "object" :head}]});
    //merge_json(&mut blank_node, anno_blan.clone());

    let blank_node_id = util::generate_blank_node_id(&blank_node);

    let blank_node_type = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject": blank_node_id,
    "predicate":RDF_TYPE,
    "object":SWRL_IMP,
    "datatype":LDTAB_IRI,
    "annotation": Value::Null
    });

    let body_triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject": blank_node_id,
    "predicate":SWRL_BODY,
    "object":body,
    "datatype":LDTAB_JSON_LIST,
    "annotation": Value::Null
    });

    let head_triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":blank_node_id,
    "predicate":SWRL_HEAD,
    "object":head,
    "datatype":LDTAB_JSON_LIST,
    "annotation": Value::Null
    });

    triples.push(blank_node_type);
    triples.push(body_triple);
    triples.push(head_triple);

    if let Some(obj) = annotation.as_object() {
        for (key, value) in obj {
            if let Some(list) = value.as_array() {
                for e in list {
                    let annotation = json!({
                    "assertion":"1",
                    "retraction":"0",
                    "graph":"graph",
                    "subject": blank_node_id,
                    "predicate":key,
                    "object":e["object"],
                    "datatype":e["datatype"],
                    "annotation": Value::Null
                    });

                    triples.push(annotation);
                }
            }
        }
    }

    Value::Array(triples)
}

pub fn translate_ontology(v: &Value) -> Value {
    let iri = &v[1];
    let viri = &v[2];

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":iri,
    "predicate":OWL_VERSION_IRI,
    "object":viri,
    "datatype":LDTAB_IRI,
    "annotation": Value::Null
    });
    triple
}

pub fn translate_doc_iri(v: &Value) -> Value {
    let iri = &v[1];

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":"ontology",//TODO
    "predicate":OWL_VERSION_IRI,
    "object":iri,
    "datatype":LDTAB_IRI,
    "annotation":""
    });
    triple
}
