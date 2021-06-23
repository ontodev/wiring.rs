pub mod thick2ofn;
pub mod ofn2thick;
pub mod owl;

fn main() {

let some_values = r#" {"subject": "ex:existential", "predicate": "rdfs:subClassOf", "object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pExistential"}], "owl:someValuesFrom": [{"object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pExistential"}], "owl:someValuesFrom": [{"object": "ex:existentialFiller"}]} }]}}"#;

    let test_some_values = thick2ofn::thick_triple_parser::parse_tiple(some_values); 
    println!("{}", test_some_values);

let intersection = r#" {"subject": "ex:intersection", "predicate": "rdfs:subClassOf", "object": {"owl:intersectionOf": [{"object": {"rdf:first": [{"object": "ex:I1"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:I2"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:I3"}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}}], "rdf:type": [{"object": "owl:Class"}]}} "#;

    let test_intersection = thick2ofn::thick_triple_parser::parse_tiple(intersection); 
    println!("{}", test_intersection); 

    let obo = r#" {"subject": "obo:OBI:0001977", "predicate": "owl:equivalentClass", "object": {"rdf:type": [{"object": "owl:Class"}], "owl:intersectionOf": [{"object": {"rdf:first": [{"object": "obo:OBI:0000070"}], "rdf:rest": [{"object": {"rdf:first": [{"object": {"rdf:type": [{"object": "owl:Class"}], "owl:intersectionOf": [{"object": {"rdf:first": [{"object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "obo:BFO:0000055"}], "owl:someValuesFrom": [{"object": {"rdf:type": [{"object": "owl:Class"}], "owl:intersectionOf": [{"object": {"rdf:first": [{"object": "obo:OBI:0000067"}], "rdf:rest": [{"object": {"rdf:first": [{"object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "obo:RO:0000081"}], "owl:someValuesFrom": [{"object": "obo:CL:0000000"}]}}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}}]}}], "rdf:rest": [{"object": {"rdf:first": [{"object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "obo:OBI:0000293"}], "owl:someValuesFrom": [{"object": {"rdf:type": [{"object": "owl:Class"}], "owl:intersectionOf": [{"object": {"rdf:first": [{"object": "obo:CL:0000000"}], "rdf:rest": [{"object": {"rdf:first": [{"object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "obo:RO:0000087"}], "owl:someValuesFrom": [{"object": "obo:OBI:0000067"}]}}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}}]}}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}}"#;

    let test_obo = thick2ofn::thick_triple_parser::parse_tiple(obo); 
    println!("{}", test_obo);

let disjoint_classes = r#"
{"subject": "_:genid27", "predicate": "owl:AllDisjointClasses", "object": {"owl:members": {"rdf:first": [{"object": "ex:disjointClass1"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:disjointClass2"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:disjointClass3"}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}}} "#;

    let test_disjoint_classes = thick2ofn::thick_triple_parser::parse_tiple(disjoint_classes); 
    println!("{}", test_disjoint_classes);

    let disjoint_union_of = r#"
{"subject": "ex:disjointUnion", "predicate": "owl:disjointUnionOf", "object": {"rdf:first": [{"object": "ex:u1"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:u2"}], "rdf:rest": [{"object": {"rdf:first": [{"object": "ex:u3"}], "rdf:rest": [{"object": "rdf:nil"}]}}]}}]}} "#;

    let test_disjoint_union_of = thick2ofn::thick_triple_parser::parse_tiple(disjoint_union_of); 
    println!("{}", test_disjoint_union_of);

let n_ary_equivalent_classes = r#" {"subject": "_:genid1", "predicate": "owl:equivalentClass", "object": {"rdf:rest": [{"object": {"rdf:rest": [{"object": {"rdf:rest": [{"object": "rdf:nil"}], "rdf:first": [{"object": "ex:equivalent2"}]}}], "rdf:first": [{"object": "ex:equivalent1"}]}}], "rdf:first": [{"object": "ex:equivalent3"}]}} "#;

    let test_n_ary_equivalent_classes = thick2ofn::thick_triple_parser::parse_tiple(n_ary_equivalent_classes); 
    println!("{}", test_n_ary_equivalent_classes);

    let inverse = r#" {"subject": "ex:A", "predicate": "rdfs:subClassOf", "object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": {"owl:inverseOf": [{"object": "ex:prop"}]}}], "owl:someValuesFrom": [{"object": "ex:B"}]}} "#;
    let test_inverse = thick2ofn::thick_triple_parser::parse_tiple(inverse); 
    println!("{}", test_inverse);

    let min = r#" {"subject": "ex:minCardinality", "predicate": "owl:equivalentClass", "object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pMinCardinality"}], "owl:minCardinality": [{"object": "1^^xsd:nonNegativeInteger"}]}} "#;
    let test_min = thick2ofn::thick_triple_parser::parse_tiple(min); 
    println!("{}", test_min);

    let min_qualified = r#" {"subject": "ex:minQualifiedCardinality", "predicate": "rdfs:subClassOf", "object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pMinQualifiedCardinality"}], "owl:minQualifiedCardinality": [{"object": "1^^xsd:nonNegativeInteger"}], "owl:onClass": [{"object": "ex:minCardinalityFiller"}]}} "#;
    let test_min_qualified = thick2ofn::thick_triple_parser::parse_tiple(min_qualified); 
    println!("{}", test_min_qualified);

    let has_self = r#" {"subject": "ex:hasSelf", "predicate": "rdfs:subClassOf", "object": {"rdf:type": [{"object": "owl:Restriction"}], "owl:onProperty": [{"object": "ex:pHasSelf"}], "owl:hasSelf": [{"object": "true^^xsd:boolean"}]}} "#;
    let test_has_self = thick2ofn::thick_triple_parser::parse_tiple(has_self); 
    println!("{}", test_has_self);




    //
    // OFN back to thick
    //
    let ofn = r#" ["SubClassOf","ex:A",["ObjectSomeValuesFrom","ex:prop","ex:B"]] "#;
    let test_ofn = ofn2thick::ofn_parser::parse_ofn(ofn);
    println!("{}", test_ofn);

    //let ofn_min = r#" ["SubClassOf","ex:A",["ObjectMinCardinality","ex:prop","1"]] "#;
    let ofn_min = r#" ["SubClassOf","ex:minCardinality",["ObjectMinCardinality","ex:pMinCardinality","1^^xsd:nonNegativeInteger"]] "#;
    let test_min = ofn2thick::ofn_parser::parse_ofn(ofn_min);
    println!("{}", test_min);

    let ofn_min_qualified = r#" ["SubClassOf","ex:minQualifiedCardinality",["ObjectMinCardinality","ex:pMinQualifiedCardinality","1^^xsd:nonNegativeInteger","ex:minCardinalityFiller"]] "#;
    let test_min_qualified = ofn2thick::ofn_parser::parse_ofn(ofn_min_qualified);
    println!("{}", test_min_qualified);

    let ofn_intersection = r#" ["SubClassOf","ex:intersection",["ObjectIntersectionOf","ex:I1","ex:I2","ex:I3"]] "#;
    let test_intersection = ofn2thick::ofn_parser::parse_ofn(ofn_intersection);
    println!("{}", test_intersection);


}

