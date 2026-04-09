use crate::constants::*;
use crate::thick_2_ofn::axiom_translation;
use serde_json::Value;

//Note that (some) thick triples are ambiguous due to missing type information.
//In case of ambiguity, a thick triple is parsed into an *abstract* OFN-S expression.
//Consider the following trick triple as an example:
//
//{"subject": "ex:A",
// "predicate": "<http://www.w3.org/2000/01/rdf-schema#subClassOf>",
// "object": {"<http://www.w3.org/2002/07/owl#onProperty>":[{"object":"ex:prop"}],
//            "<http://www.w3.org/2002/07/owl#someValuesFrom>":[{"object":"ex:B"}],
//            "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>":[{"object":"<http://www.w3.org/2002/07/owl#Restriction>"}]}}
//
//Without type information about either the property or the filler of the existential restriction,
//we cannot decide whether it is
//an *ObjectSomeValuesFrom* expression or a *DataSomeValuesFrom* expression.
//So, we parse it into an abstract expression *SomeValuesFrom*
//and determine its actual type using the modules in crate::ofn_typing.

pub fn thick_2_ofn(thick_triple: &Value) -> Value {
    //we convert subcomponents of a thick_triple to strings,
    //so that we can serialise them into owl::OWL using serde
    let subj_helper: String = thick_triple["subject"].to_string();
    let subj: &str = subj_helper.as_str();

    let obj_helper: String = thick_triple["object"].to_string();
    let obj: &str = obj_helper.as_str();

    let predicate = thick_triple["predicate"].as_str();

    match predicate {
        Some(x) if x == RDFS_SUB_CLASS_OF => axiom_translation::translate_subclass_of_axiom(subj, obj),
        Some(x) if x == OWL_EQUIVALENT_CLASS => axiom_translation::translate_equivalent_class(subj, obj),
        Some(x) if x == OWL_ALL_DISJOINT_CLASSES => {
            let members_helper: String = thick_triple["object"][OWL_MEMBERS].to_string();
            let members: &str = members_helper.as_str();
            axiom_translation::translate_disjoint_classes(members)
        }
        Some(x) if x == OWL_DISJOINT_UNION_OF => axiom_translation::translate_disjoint_union(subj, obj),
        Some(x) if x == OWL_DISJOINT_WITH => axiom_translation::translate_disjoint_with(subj, obj),
        Some(_) => axiom_translation::translate_thin_triple(&thick_triple),
        None => Value::String(String::from("Fail")),
    }
}
