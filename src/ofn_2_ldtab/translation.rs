use crate::ofn_2_ldtab::axiom_translation;
use crate::ofn_2_ldtab::util;
use serde_json::Value;

/// Given an OFN S-expression (encoded in JSON),
/// return its corresponding LDTab ThickTripe
///
/// Examples
///
/// let ofn_string = r#"["SubClassOf","obo:IAO_0000120",["ObjectSomeValuesFrom","obo:BFO_0000050","obo:OBI_0500000"]]"#;
/// let ofn = util::parser::parse(&ofn_string);
/// let thick_triple = ofn_2_ldtab::translation::ofn_2_thick_triple(&ofn);
/// println!("{}", thick_triple);
///
pub fn ofn_2_thick_triple(v: &Value) -> Value {
    let ldtab_triple = match v[0].as_str() {
        Some("Declaration") => axiom_translation::translate_declaration(v),
        Some("DatatypeDefinition") => axiom_translation::translate_datatype_definition(v),
        Some("SubClassOf") => axiom_translation::translate_subclass_of_axiom(v),
        Some("DisjointClasses") => axiom_translation::translate_disjoint_classes_axiom(v),
        Some("DisjointUnionOf") => axiom_translation::translate_disjoint_union_of_axiom(v),
        Some("EquivalentClasses") => axiom_translation::translate_equivalent_classes_axiom(v),
        Some("SubObjectPropertyOf") => axiom_translation::translate_sub_object_property(v),
        Some("SubDataPropertyOf") => axiom_translation::translate_sub_data_property(v),
        Some("EquivalentObjectProperties") => {
            axiom_translation::translate_equivalent_properties_axiom(v)
        }
        Some("EquivalentDataProperties") => {
            axiom_translation::translate_equivalent_properties_axiom(v)
        }
        Some("DisjointObjectProperties") => {
            axiom_translation::translate_disjoint_properties_axiom(v)
        }
        Some("DisjointDataProperties") => axiom_translation::translate_disjoint_properties_axiom(v),
        Some("InverseObjectProperties") => axiom_translation::translate_inverse_properties_axiom(v),
        Some("FunctionalObjectProperty") => {
            axiom_translation::translate_functional_property_axiom(v)
        }
        Some("FunctionalDataProperty") => axiom_translation::translate_functional_property_axiom(v),
        Some("InverseFunctionalObjectProperty") => {
            axiom_translation::translate_inverse_functional_object_property_axiom(v)
        }
        Some("ReflexiveObjectProperty") => {
            axiom_translation::translate_reflexive_object_property_axiom(v)
        }
        Some("IrreflexiveObjectProperty") => {
            axiom_translation::translate_irreflexive_object_property_axiom(v)
        }
        Some("SymmetricObjectProperty") => {
            axiom_translation::translate_symmetric_object_property_axiom(v)
        }
        Some("AsymmetricObjectProperty") => {
            axiom_translation::translate_asymmetric_object_property_axiom(v)
        }
        Some("TransitiveObjectProperty") => {
            axiom_translation::translate_transitive_object_property_axiom(v)
        }
        Some("ObjectPropertyDomain") => {
            axiom_translation::translate_object_property_domain_axiom(v)
        }
        Some("ObjectPropertyRange") => axiom_translation::translate_object_property_range_axiom(v),

        Some("DataPropertyDomain") => axiom_translation::translate_data_property_domain_axiom(v),
        Some("DataPropertyRange") => axiom_translation::translate_data_property_range_axiom(v),
        Some("SameIndividual") => axiom_translation::translate_same_individuals_axiom(v),
        Some("DifferentIndividuals") => axiom_translation::translate_different_individuals_axiom(v),
        Some("HasKey") => axiom_translation::translate_has_key_axiom(v),
        Some("ClassAssertion") => axiom_translation::translate_class_assertion_axiom(v),
        Some("ObjectPropertyAssertion") => {
            axiom_translation::translate_object_property_assertion_axiom(v)
        }
        Some("NegativeObjectPropertyAssertion") => {
            axiom_translation::translate_negative_object_property_assertion_axiom(v)
        }
        Some("DataPropertyAssertion") => {
            axiom_translation::translate_object_property_assertion_axiom(v)
        }
        Some("NegativeDataPropertyAssertion") => {
            axiom_translation::translate_negative_data_property_assertion_axiom(v)
        }
        Some("AnnotationAssertion") => axiom_translation::translate_annotation_assertion_axiom(v),
        Some("SubAnnotationPropertyOf") => {
            axiom_translation::translate_sub_annotation_property_of_axiom(v)
        }
        Some("AnnotationPropertyDomain") => {
            axiom_translation::translate_annotation_property_domain_axiom(v)
        }
        Some("AnnotationPropertyRange") => {
            axiom_translation::translate_annotation_property_range_axiom(v)
        }
        Some("Import") => axiom_translation::translate_ontology_import(v),

        //TODO: ontology annotations
        Some("OntologyAnnotation") => axiom_translation::translate_ontology_annotation(v),
        Some("DocIRI") => axiom_translation::translate_doc_iri(v), //this is horned-owl specific
        Some("Ontology") => axiom_translation::translate_ontology(v),

        Some("DLSafeRule") => axiom_translation::translate_rule(v),

        Some(_) => panic!(),
        None => panic!(),
    };

    //ensure that all triples for LDTab conform to the same order
    util::sort_value(&ldtab_triple)
}
