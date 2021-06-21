pub mod class_translation; 
pub mod axiom_translation;
pub mod thick_triple_parser;

fn main() {

let some_values = r#" {"subject": "ex:existential", "predicate": "rdfs:subClassOf", "object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pExistential"}], "owl:someValuesFrom": [{"object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pExistential"}], "owl:someValuesFrom": [{"object": "ex:existentialFiller"}]} }]}}"#;

    let test = thick_triple_parser::parse_tiple(some_values); 
    println!("{}", test);
    //class_translation::obo_example().ok(); 
    //axiom_translation::obo_example2().ok(); 
}

