pub mod class_translation; 
pub mod axiom_translation;
pub mod thick_triple_parser;

fn main() {

let some_values = r#" {"subject": "ex:existential", "predicate": "rdfs:subClassOf", "object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pExistential"}], "owl:someValuesFrom": [{"object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pExistential"}], "owl:someValuesFrom": [{"object": "ex:existentialFiller"}]} }]}}"#;

    let test_some_values = thick_triple_parser::parse_tiple(some_values); 
    println!("{}", test_some_values);

let intersection = r#" {"subject": "ex:intersection", "predicate": "rdfs:subClassOf", "object": {"owl:intersectionOf": [{"object": {"rdf:first": [{"object": "ex:I1"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:I2"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:I3"}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}}], "rdf:type": [{"object": "owl:Class"}]}} "#;

    let test_intersection = thick_triple_parser::parse_tiple(intersection); 
    println!("{}", test_intersection); 

    let obo = r#" {"subject": "obo:OBI:0001977", "predicate": "owl:equivalentClass", "object": {"rdf:type": [{"object": "owl:Class"}], "owl:intersectionOf": [{"object": {"rdf:first": [{"object": "obo:OBI:0000070"}], "rdf:rest": [{"object": {"rdf:first": [{"object": {"rdf:type": [{"object": "owl:Class"}], "owl:intersectionOf": [{"object": {"rdf:first": [{"object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "obo:BFO:0000055"}], "owl:someValuesFrom": [{"object": {"rdf:type": [{"object": "owl:Class"}], "owl:intersectionOf": [{"object": {"rdf:first": [{"object": "obo:OBI:0000067"}], "rdf:rest": [{"object": {"rdf:first": [{"object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "obo:RO:0000081"}], "owl:someValuesFrom": [{"object": "obo:CL:0000000"}]}}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}}]}}], "rdf:rest": [{"object": {"rdf:first": [{"object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "obo:OBI:0000293"}], "owl:someValuesFrom": [{"object": {"rdf:type": [{"object": "owl:Class"}], "owl:intersectionOf": [{"object": {"rdf:first": [{"object": "obo:CL:0000000"}], "rdf:rest": [{"object": {"rdf:first": [{"object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "obo:RO:0000087"}], "owl:someValuesFrom": [{"object": "obo:OBI:0000067"}]}}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}}]}}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}}"#;

    let test_obo = thick_triple_parser::parse_tiple(obo); 
    println!("{}", test_obo);

let disjoint_classes = r#"
{"subject": "_:genid27", "predicate": "owl:AllDisjointClasses", "object": {"owl:members": {"rdf:first": [{"object": "ex:disjointClass1"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:disjointClass2"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:disjointClass3"}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}}} "#;

    let test_disjoint_classes = thick_triple_parser::parse_tiple(disjoint_classes); 
    println!("{}", test_disjoint_classes);

}

