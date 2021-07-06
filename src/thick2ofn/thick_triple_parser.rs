use serde_json::{Value};

use crate::thick2ofn::axiom_translation as axiom_translation; 

//Note that (some) thick triples are ambiguous due to missing type information.
//In case of ambiguity, a thick triple is parsed into an *abstract* OFN-S expression.
//Consider the following trick triple as an example: 
//
//{"subject": "ex:A",
// "predicate": "rdfs:subClassOf",
// "object": {"owl:onProperty":[{"object":"ex:prop"}],
//            "owl:someValuesFrom":[{"object":"ex:B"}],
//            "rdf:type":[{"object":"owl:Restriction"}]}}
//
//Without type information about either the property or the filler of the existential restriction,
//we cannot decide whether it is
//an *ObjectSomeValuesFrom* expression or a *DataSomeValuesFrom* expression.
//So, we parse it into an abstract expression *SomeValuesFrom*
//and determine its actual type using the modules in crate::ofn_typing.

pub fn parse_triple(t: &str) -> String {

    let thick_triple: Value = serde_json::from_str(t).unwrap();

    //TODO: I cannot chain to_string() and as_str() - why?
    let subj_helper : String  = thick_triple["subject"].to_string();
    let subj : &str = subj_helper.as_str();

    let predicate : String = thick_triple["predicate"].to_string();

    let obj_helper : String  = thick_triple["object"].to_string();
    let obj : &str = obj_helper.as_str();

    match predicate.as_str() {
        "\"rdfs:subClassOf\"" => axiom_translation::translate_subclass_of_axiom(subj, obj),
        "\"owl:equivalentClass\"" => axiom_translation::translate_equivalent_class(subj, obj),
        "\"owl:AllDisjointClasses\"" => {
                let members_helper : String = thick_triple["object"]["owl:members"].to_string();
                let members : &str = members_helper.as_str(); 
                axiom_translation::translate_disjoint_classes(members) 
            }, 
        "\"owl:disjointUnionOf\"" => axiom_translation::translate_disjoint_union(subj, obj),
        _ => String::from("Fail"),
    } 
} 
