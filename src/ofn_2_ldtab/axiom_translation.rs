use crate::ofn_2_ldtab::annotation_translation;
use crate::ofn_2_ldtab::class_translation;
use crate::ofn_2_ldtab::property_translation;
use crate::ofn_2_ldtab::util;
use rand::Rng;
use serde_json::json;
use serde_json::Value;

//TODO
//
// == [1] ==
//top level translation is not always correct:
//- "subject":{"datatype":"_IRI","object":"http://purl.obolibrary.org/obo/OBI_0000301"}}
//should just be
//- "subject":"http://purl.obolibrary.org/obo/OBI_0000301"

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
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let ofn_annotations = annotation_translation::get_annotations(v);
    let annotation = annotation_translation::translate_annotations(&ofn_annotations);

    //unwrap declaration
    let unwrapped_declaration = owl[1].clone();

    //translate OWL classes
    let class = class_translation::translate(&unwrapped_declaration[1]);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":class,
    "predicate":"rdf:type",
    "object":"owl:Class",
    "datatype":"_IRI",
    "annotation":annotation
    });
    triple
}

pub fn translate_ontology_import(v: &Value) -> Value {
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let ofn_annotations = annotation_translation::get_annotations(v);
    let annotation = annotation_translation::translate_annotations(&ofn_annotations);

    let subject = class_translation::translate(&owl[1]);
    let object = class_translation::translate(&owl[2]);

    json!({"assertion":"1",
        "retraction":"0",
        "graph":"graph",
        "subject": subject,
        "predicate":"owl:imports",
        "object": object,
        "datatype":"_IRI",
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
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let ofn_annotations = annotation_translation::get_annotations(v);
    let annotation = annotation_translation::translate_annotations(&ofn_annotations);

    //translate OWL classes
    let class = class_translation::translate(&owl[1]);
    let individual = class_translation::translate(&owl[2]);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":individual,
    "predicate":"rdf:type",
    "object":class,
    "datatype":"_IRI",
    "annotation":annotation
    });
    triple
}

pub fn translate_object_property_assertion_axiom(v: &Value) -> Value {
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let ofn_annotations = annotation_translation::get_annotations(v);
    let annotation = annotation_translation::translate_annotations(&ofn_annotations);

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
    "datatype":"_IRI",
    "annotation":annotation
    });
    triple
}

pub fn translate_data_property_assertion_axiom(v: &Value) -> Value {
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let ofn_annotations = annotation_translation::get_annotations(v);
    let annotation = annotation_translation::translate_annotations(&ofn_annotations);

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
    "datatype":"_IRI",
    "annotation":annotation
    });
    triple
}

pub fn translate_negative_object_property_assertion_axiom(v: &Value) -> Value {
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let ofn_annotations = annotation_translation::get_annotations(v);
    let annotation = annotation_translation::translate_annotations(&ofn_annotations);

    //translate OWL classes
    let property = property_translation::translate(&owl[1]);
    let from = class_translation::translate(&owl[2]);
    let to = class_translation::translate(&owl[3]);

    let mut rng = rand::thread_rng();
    let blank_id: u8 = rng.gen();
    let blank_node = format!("_:gen{}", blank_id);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":blank_node,
    "predicate":"owl:NegativePropertyAssertion",
    "object":{
        "owl:sourceIndividual":[{"object":from}],
        "owl:assertionProperty":[{"object":property}],
        "owl:targetIndividual":[{"object":to}]
    },
    "datatype":"_IRI",
    "annotation":annotation
    });
    triple
}

pub fn translate_negative_data_property_assertion_axiom(v: &Value) -> Value {
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let ofn_annotations = annotation_translation::get_annotations(v);
    let annotation = annotation_translation::translate_annotations(&ofn_annotations);

    //translate OWL classes
    let property = property_translation::translate(&owl[1]);
    let from = class_translation::translate(&owl[2]);
    let to = class_translation::translate(&owl[3]);

    let mut rng = rand::thread_rng();
    let blank_id: u8 = rng.gen();
    let blank_node = format!("_:gen{}", blank_id);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":blank_node,
    "predicate":"owl:NegativePropertyAssertion",
    "object":{
        "owl:sourceIndividual":[{"object":from}],
        "owl:assertionProperty":[{"object":property}],
        "owl:targetIndividual":[{"object":to}]
    },
    "datatype":"_IRI",
    "annotation":annotation
    });
    triple
}

pub fn translate_same_individuals_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let number_of_operands = (owl.as_array().unwrap())[1..].len();

    if number_of_operands == 2 {
        //TODO check that class_translation supports individuals
        let lhs = class_translation::translate(&owl[1]);
        let rhs = class_translation::translate(&owl[2]);
        let annotation = annotation_translation::translate_annotations(&annotations);

        let triple = json!({
                        "assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":lhs,
                        "predicate":"owl:sameAs",
                        "object":rhs, 
                        "datatype":util::translate_datatype(&json!(rhs)), 
                        "annotation":annotation});
        triple
    } else {
        let mut rng = rand::thread_rng();
        let blank_id: u8 = rng.gen();
        let blank_node = format!("_:gen{}", blank_id);

        let operands: Value = class_translation::translate_list(&(owl.as_array().unwrap())[1..]);
        let annotation = annotation_translation::translate_annotations(&annotations);
        let triple = json!({"assertion":"1",
                            "retraction":"0",
                            "graph":"graph", //TODO
                            "subject":blank_node,
                            //"predicate":"owl:sameAs", 
                            "predicate":"owl:AllSameAs", //this is LDtab specific
                            "object": {"owl:members":[{"object":operands, "datatype":"_JSON"}]}, //TODO remove datatype
                            "datatype":"_JSON",
                            "annotation":annotation});
        triple
    }
}

pub fn translate_different_individuals_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let number_of_operands = (owl.as_array().unwrap())[1..].len();
    if number_of_operands == 2 {
        let lhs = class_translation::translate(&owl[1]);
        let rhs = class_translation::translate(&owl[2]);
        let annotation = annotation_translation::translate_annotations(&annotations);

        let triple = json!({
                        "assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":lhs,
                        "predicate":"owl:differentFrom",
                        "object":rhs, 
                        "datatype":util::translate_datatype(&json!(rhs)), 
                        "annotation":annotation});
        triple
    } else {
        let mut rng = rand::thread_rng();
        let blank_id: u8 = rng.gen();
        let blank_node = format!("_:gen{}", blank_id);

        let operands: Value = class_translation::translate_list(&(owl.as_array().unwrap())[1..]);
        let annotation = annotation_translation::translate_annotations(&annotations);
        let triple = json!({"assertion":"1",
                            "retraction":"0",
                            "graph":"graph", //TODO
                            "subject":blank_node,
                            "predicate":"owl:AllDifferent", 
                            "object": {"owl:distinctMembers":[{"object":operands, "datatype":"_JSON"}],"rdf:type":[{"datatype":"_IRI","object":"owl:AllDifferent"}]},
                            "datatype":"_JSON",
                            "annotation":annotation});
        triple
    }
}

pub fn translate_object_property_declaration(v: &Value) -> Value {
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);
    let annotation = annotation_translation::translate_annotations(&annotations);

    //unwrap declaration
    let unwrapped_declaration = owl[1].clone();

    let property = property_translation::translate(&unwrapped_declaration[1]);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":property,
    "predicate":"rdf:type",
    "object":"owl:ObjectProperty",
    "datatype":"_IRI",
    "annotation":annotation
    });
    triple
}

pub fn translate_data_property_declaration(v: &Value) -> Value {
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);
    let annotation = annotation_translation::translate_annotations(&annotations);

    //unwrap declaration
    let unwrapped_declaration = owl[1].clone();

    let property = property_translation::translate(&unwrapped_declaration[1]);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":property,
    "predicate":"rdf:type",
    "object":"owl:DatatypeProperty",
    "datatype":"_IRI",
    "annotation":annotation
    });
    triple
}

pub fn translate_annotation_property_declaration(v: &Value) -> Value {
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);
    let annotation = annotation_translation::translate_annotations(&annotations);

    //unwrap declaration
    let unwrapped_declaration = owl[1].clone();

    let property = property_translation::translate(&unwrapped_declaration[1]);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":property,
    "predicate":"rdf:type",
    "object":"owl:AnnotationProperty",
    "datatype":"_IRI",
    "annotation":annotation
    });
    triple
}

//TODO: test this
pub fn translate_datatype_definition(v: &Value) -> Value {
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);
    let annotation = annotation_translation::translate_annotations(&annotations);

    //TODO: check this (should just be a string)
    let lhs = owl[1].clone();
    let rhs = class_translation::translate(&owl[2].clone()); //this is a datarange

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":lhs,
    "predicate":"owl:equivalentClass",
    "object":rhs,
    "datatype":"_IRI",
    "annotation":annotation
    });
    triple
}

pub fn translate_datatype_declaration(v: &Value) -> Value {
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);
    let annotation = annotation_translation::translate_annotations(&annotations);

    //unwrap declaration
    let unwrapped_declaration = owl[1].clone();

    //TODO: check this (should just be a string)
    let datatype = unwrapped_declaration[1].clone();

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":datatype,
    "predicate":"rdf:type",
    "object":"rdfs:Datatype",
    "datatype":"_IRI",
    "annotation":annotation
    });
    triple
}

pub fn translate_individual_declaration(v: &Value) -> Value {
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);
    let annotation = annotation_translation::translate_annotations(&annotations);

    //unwrap declaration
    let unwrapped_declaration = owl[1].clone();

    //TODO: check this (should just be a string)
    let individual = unwrapped_declaration[1].clone();

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":individual,
    "predicate":"rdf:type",
    "object":"owl:NamedIndividual",
    "datatype":"_IRI",
    "annotation":annotation
    });
    triple
}

pub fn translate_sub_object_property(v: &Value) -> Value {
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    //SubObjectPropertyOf( ObjectPropertyChain( OPE1 ... OPEn ) OPE )
    //is translated as
    //T(OPE) owl:propertyChainAxiom T(SEQ OPE1 ... OPEn) .
    //so, we need to check here
    //whether the SubObjectPropertyOf has an ObjectPropertyChain argument
    if owl[1].is_array() && owl[1][0].as_str().unwrap().eq("ObjectPropertyChain") {
        let sub = property_translation::translate_list(&(owl[1].as_array().unwrap())[1..]);
        let sup = property_translation::translate(&owl[2]);
        let annotation = annotation_translation::translate_annotations(&annotations);

        json!({ "assertion":"1",
            "retraction":"0",
            "graph":"graph",
            "subject":sup,
            "predicate":"owl:propertyChainAxiom",
            "object":sub,
            "datatype":util::translate_datatype(&json!(sub)),
            "annotation":annotation
        })
    } else {
        let sub = property_translation::translate(&owl[1]);
        let sup = property_translation::translate(&owl[2]);
        let annotation = annotation_translation::translate_annotations(&annotations);

        json!({ "assertion":"1",
            "retraction":"0",
            "graph":"graph",
            "subject":sub,
            "predicate":"rdfs:subPropertyOf",
            "object":sup,
            "datatype":util::translate_datatype(&json!(sup)),
            "annotation":annotation
        })
    }
}

pub fn translate_sub_data_property(v: &Value) -> Value {
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let sub = property_translation::translate(&owl[1]);
    let sup = property_translation::translate(&owl[2]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    json!({ "assertion":"1",
        "retraction":"0",
        "graph":"graph",
        "subject":sub,
        "predicate":"rdfs:subPropertyOf",
        "object":sup,
        "datatype":util::translate_datatype(&json!(sup)),
        "annotation":annotation
    })
}

pub fn translate_subclass_of_axiom(v: &Value) -> Value {
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    //translate OWL classes
    let subclass = class_translation::translate(&owl[1]);
    let superclass = class_translation::translate(&owl[2]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":subclass,
    "predicate":"rdfs:subClassOf",
    "object":superclass,
    "datatype":util::translate_datatype(&json!(superclass)),
    "annotation":annotation
    });
    triple
}

pub fn translate_sub_annotation_property_of_axiom(v: &Value) -> Value {
    //split annotations from logical structure
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    //translate OWL classes
    let lhs = property_translation::translate(&owl[1]);
    let rhs = property_translation::translate(&owl[2]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":lhs,
    "predicate":"rdfs:subPropertyOf",
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
                            "predicate":"owl:disjointWith",
                            "object": rhs,
                            "datatype":util::translate_datatype(&json!(rhs)),
                            "annotation":annotation});
        triple
    } else {

        let mut rng = rand::thread_rng();
        let blank_id: u8 = rng.gen();
        let blank_node = format!("_:gen{}", blank_id);

        let operands: Value = class_translation::translate_list(&(owl.as_array().unwrap())[1..]);
        let annotation = annotation_translation::translate_annotations(&annotations);

        let triple = json!({"assertion":"1",
                            "retraction":"0",
                            "graph":"graph",
                            "subject":blank_node,
                            "predicate":"owl:AllDisjointClasses",
                            "object": {"owl:members":[{"object":operands, "datatype":"_JSON"}],"rdf:type":[{"datatype":"_IRI","object":"owl:AllDisjointClasses"}]},
                            "datatype": "_JSON", 
                            "annotation":annotation});
        triple
    }
}

pub fn translate_disjoint_union_of_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let lhs = class_translation::translate(&owl[1]);
    let operands: Value = class_translation::translate_list(&(owl.as_array().unwrap())[2..]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({
                        "assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":lhs,
                        "predicate":"owl:disjointUnionOf",
                        "object":operands,
                        "datatype": "_JSON", 
                        "annotation":annotation});
    triple
}

//TODO:: equivalent classe  (we have a custom encoding for this and need a case distinction
//between binary axioms and n-ary axioms)
pub fn translate_equivalent_classes_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let number_of_operands = (owl.as_array().unwrap())[1..].len();
    if number_of_operands == 2 {
        let lhs = class_translation::translate(&owl[1]);
        let rhs = class_translation::translate(&owl[2]);
        let annotation = annotation_translation::translate_annotations(&annotations);

        let triple = json!({
                        "assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":lhs,
                        "predicate":"owl:equivalentClass",
                        "object":rhs, 
                        "datatype":util::translate_datatype(&json!(rhs)), 
                        "annotation":annotation});
        triple
    } else {
        let mut rng = rand::thread_rng();
        let blank_id: u8 = rng.gen();
        let blank_node = format!("_:gen{}", blank_id);

        let operands: Value = class_translation::translate_list(&(owl.as_array().unwrap())[1..]);
        let annotation = annotation_translation::translate_annotations(&annotations);
        let triple = json!({"assertion":"1",
                            "retraction":"0",
                            "graph":"graph", //TODO
                            "subject":blank_node,
                            "predicate":"owl:equivalentClass",
                            "object": {"owl:members":[{"object":operands, "datatype":"_JSON"}]}, //TODO remove datatype 
                            "datatype":"_JSON",
                            "annotation":annotation});
        triple
    }
}

pub fn translate_object_property_domain_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let property = property_translation::translate(&owl[1]);
    let domain = class_translation::translate(&owl[2]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":property,
                        "predicate":"rdfs:domain",
                        "object":domain,
                        "datatype": util::translate_datatype(&json!(domain)),
                        "annotation":annotation});
    triple
}

pub fn translate_object_property_range_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let property = property_translation::translate(&owl[1]);
    let range = class_translation::translate(&owl[2]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":property,
                        "predicate":"rdfs:range",
                        "object":range,
                        "datatype": util::translate_datatype(&json!(range)),
                        "annotation":annotation});
    triple
}

pub fn translate_annotation_property_domain_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let property = property_translation::translate(&owl[1]);
    let domain = class_translation::translate(&owl[2]); //TODO IRI
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":property,
                        "predicate":"rdfs:domain",
                        "object":domain,
                        "datatype": util::translate_datatype(&json!(domain)),
                        "annotation":annotation});
    triple
}

pub fn translate_annotation_property_range_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let property = property_translation::translate(&owl[1]);
    let range = class_translation::translate(&owl[2]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":property,
                        "predicate":"rdfs:range",
                        "object":range,
                        "datatype": util::translate_datatype(&json!(range)),
                        "annotation":annotation});
    triple
}

//TODO test n-ary case
pub fn translate_equivalent_properties_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let number_of_operands = (owl.as_array().unwrap())[1..].len();
    if number_of_operands == 2 {
        let lhs = property_translation::translate(&owl[1]);
        let rhs = property_translation::translate(&owl[2]);
        let annotation = annotation_translation::translate_annotations(&annotations);

        let triple = json!({
                        "assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":lhs,
                        "predicate":"owl:equivalentProperty",
                        "object":rhs, 
                        "datatype":util::translate_datatype(&json!(rhs)), 
                        "annotation":annotation});
        triple
    } else {
        let mut rng = rand::thread_rng();
        let blank_id: u8 = rng.gen();
        let blank_node = format!("_:gen{}", blank_id);

        let operands: Value = property_translation::translate_list(&(owl.as_array().unwrap())[1..]);
        let annotation = annotation_translation::translate_annotations(&annotations);
        let triple = json!({"assertion":"1",
                            "retraction":"0",
                            "graph":"graph", //TODO
                            "subject":blank_node,
                            "predicate":"owl:equivalentProperty", //TODO AllEquivalentProperties?
                            "object": {"owl:members":[{"object":operands, "datatype":"_JSON"}]}, //TODO remove datatype
                            "datatype":"_JSON",
                            "annotation":annotation});
        triple
    }
}

pub fn translate_data_property_domain_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let property = property_translation::translate(&owl[1]);
    let domain = class_translation::translate(&owl[2]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":property,
                        "predicate":"rdfs:domain",
                        "object":domain,
                        "datatype": util::translate_datatype(&json!(domain)),
                        "annotation":annotation});
    triple
}

pub fn translate_inverse_properties_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let lhs = property_translation::translate(&owl[1]);
    let rhs = class_translation::translate(&owl[2]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":lhs,
                        "predicate":"owl:inverseOf",
                        "object":rhs,
                        "datatype": util::translate_datatype(&json!(rhs)),
                        "annotation":annotation});
    triple
}

pub fn translate_functional_property_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let argument = property_translation::translate(&owl[1]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":argument,
                        "predicate":"rdf:type",
                        "object":"owl:FunctionalProperty",
                        "datatype": util::translate_datatype(&json!(argument)),
                        "annotation":annotation});
    triple
}

pub fn translate_inverse_functional_object_property_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let argument = property_translation::translate(&owl[1]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":argument,
                        "predicate":"rdf:type",
                        "object":"owl:InverseFunctionalProperty",
                        "datatype": util::translate_datatype(&json!(argument)),
                        "annotation":annotation});
    triple
}

pub fn translate_reflexive_object_property_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let argument = property_translation::translate(&owl[1]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":argument,
                        "predicate":"rdf:type",
                        "object":"owl:ReflexiveProperty",
                        "datatype": util::translate_datatype(&json!(argument)),
                        "annotation":annotation});
    triple
}

pub fn translate_irreflexive_object_property_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let argument = property_translation::translate(&owl[1]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":argument,
                        "predicate":"rdf:type",
                        "object":"owl:IrreflexiveProperty",
                        "datatype": util::translate_datatype(&json!(argument)),
                        "annotation":annotation});
    triple
}

pub fn translate_symmetric_object_property_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let argument = property_translation::translate(&owl[1]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":argument,
                        "predicate":"rdf:type",
                        "object":"owl:SymmetricProperty",
                        "datatype": util::translate_datatype(&json!(argument)),
                        "annotation":annotation});
    triple
}

pub fn translate_asymmetric_object_property_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let argument = property_translation::translate(&owl[1]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":argument,
                        "predicate":"rdf:type",
                        "object":"owl:AsymmetricProperty",
                        "datatype": util::translate_datatype(&json!(argument)),
                        "annotation":annotation});
    triple
}

pub fn translate_transitive_object_property_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let argument = property_translation::translate(&owl[1]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":argument,
                        "predicate":"rdf:type",
                        "object":"owl:TransitiveProperty",
                        "datatype": util::translate_datatype(&json!(argument)),
                        "annotation":annotation});
    triple
}

pub fn translate_data_property_range_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let property = property_translation::translate(&owl[1]);
    let range = class_translation::translate(&owl[2]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":property,
                        "predicate":"rdfs:range",
                        "object":range,
                        "datatype": util::translate_datatype(&json!(range)),
                        "annotation":annotation});
    triple
}

pub fn translate_disjoint_properties_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let number_of_operands = (owl.as_array().unwrap())[1..].len();
    if number_of_operands == 2 {
        let lhs = property_translation::translate(&owl[1]);
        let rhs = property_translation::translate(&owl[2]);
        let annotation = annotation_translation::translate_annotations(&annotations);

        let triple = json!({
                        "assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":lhs,
                        "predicate":"owl:propertyDisjointWith",
                        "object":rhs, 
                        "datatype":util::translate_datatype(&json!(rhs)), 
                        "annotation":annotation});
        triple
    } else {
        let mut rng = rand::thread_rng();
        let blank_id: u8 = rng.gen();
        let blank_node = format!("_:gen{}", blank_id);

        let operands: Value = property_translation::translate_list(&(owl.as_array().unwrap())[1..]);
        let annotation = annotation_translation::translate_annotations(&annotations);
        let triple = json!({"assertion":"1",
                            "retraction":"0",
                            "graph":"graph", //TODO
                            "subject":blank_node,
                            "predicate":"owl:AllDisjointProperties", 
                            "object": {"owl:members":[{"object":operands, "datatype":"_JSON"}]}, //TODO remove datatype
                            "datatype":"_JSON",
                            "annotation":annotation});
        triple
    }
}

pub fn translate_has_key_axiom(v: &Value) -> Value {
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);

    let mut rng = rand::thread_rng();
    let blank_id: u8 = rng.gen();
    let blank_node = format!("_:gen{}", blank_id);

    let operands: Value = property_translation::translate_list(&(owl.as_array().unwrap())[1..]);
    let annotation = annotation_translation::translate_annotations(&annotations);

    let triple = json!({"assertion":"1",
                        "retraction":"0",
                        "graph":"graph", //TODO
                        "subject":blank_node,
                        "predicate":"owl:hasKey",
                        "object": {"owl:members":operands}, //TODO remove datatype
                        "datatype": "_JSON", 
                        "annotation":annotation});
    triple
}

pub fn translate_annotation_assertion_axiom(v: &Value) -> Value {
    //TODO: check order
    let owl = annotation_translation::get_owl(v);
    let annotations = annotation_translation::get_annotations(v);
    let annotation = annotation_translation::translate_annotations(&annotations);

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
        "datatype":"_IRI",
        "annotation":annotation
        });
        triple
    }
}

pub fn translate_ontology(v: &Value) -> Value {
    let iri = &v[1];
    let viri = &v[2];

    let triple = json!({
    "assertion":"1",
    "retraction":"0",
    "graph":"graph",
    "subject":iri,
    "predicate":"owl:versionIRI",
    "object":viri,
    "datatype":"_IRI",
    "annotation":""
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
    "predicate":"owl:versionIRI",
    "object":iri,
    "datatype":"_IRI",
    "annotation":""
    });
    triple
}
