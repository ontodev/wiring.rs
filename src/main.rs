#![warn(clippy::all, clippy::pedantic)]

pub mod thick_2_ofn;
pub mod ofn_2_thick;
pub mod ofn_2_man;
pub mod owl;
pub mod ofn_labeling;
pub mod ofn_typing;
pub mod triples;
pub mod ofn_util;
pub mod ofn_2_rdfa;
pub mod ldtab_2_ofn;
pub mod ofn_2_ldtab;
use std::collections::HashMap;
use serde_json::json; 


fn main(){
    let path = String::from("resources/thickTest.txt"); 

    //let entity_2_label = ofn_labeling::labeling::extract_labeling(&path);
    let entity_2_type = ofn_typing::typing::extract_typing(&path); 
    let triples = triples::parser::get_thick_triples(&path);

    for triple in triples.iter() { 
         println!("Triple: {}", triple );//thick triples are given

         let ofn = ldtab_2_ofn::translation::thick_triple_2_ofn(triple.to_string().as_str());
         println!("OFN S: {}", ofn );//transformation to OFN S-expression 

         let ofn_typed  = ofn_typing::ofn_parser::parse_ofn(&ofn, &entity_2_type);
         println!("Typed: {}", ofn_typed);//OFN S-expression after typing

         println!("");
    }

}

fn _playground() {

    //example file with thick triples
    let path = String::from("resources/thickOBI.txt"); 
    //let path = String::from("resources/testOBI.txt"); 

    //get thick triples that encode class expression axioms
    let axioms = triples::parser::extract_class_expression_axioms(&path);
    //let thin_triples = triples::parser::extract_thin_triples(&path);

    //get OWL type information and entity labels
    let entity_2_label = ofn_labeling::labeling::extract_labeling(&path);
    let entity_2_type = ofn_typing::typing::extract_typing(&path); 

    //println!("Label: {:?}", entity_2_label);
    //let target = String::from("\"obo:OBI_2100162\"");
    //println!("Label: {:?}", entity_2_label.get(&target));
    //let t : &str = entity_2_label.get(&target).unwrap().as_str();
    //let size = t.len();
    //let tt = &t[1..size-1];
    //println!("Label: {:?}", tt);

    let mut prefix_2_expansion = HashMap::new();
    prefix_2_expansion.insert("obo".to_string(),"OBO#".to_string());
    //prefix_2_expansion.insert("obo".to_string(),"<http://purl.obolibrary.org/obo/>".to_string());

    let mut iri_2_prefix = HashMap::new();
    iri_2_prefix.insert("<OBO#>".to_string(),"obo:".to_string());
    //iri_2_prefix.insert("<http://purl.obolibrary.org/obo/>".to_string(),"obo:".to_string());

    //for triple in thin_triples.iter(){
    //     println!("Triple: {}", triple );//thick triples are given
    //     let ofn = thick_2_ofn::parser::parse_triple(triple.to_string().as_str());
    //     println!("OFN S: {}", ofn );//transformation to OFN S-expression
    //     let ofn_typed  = ofn_typing::ofn_parser::parse_ofn(&ofn, &entity_2_type);
    //     println!("Typed: {}", ofn_typed);//OFN S-expression after typing
    //     let ofn_labelled = ofn_labeling::ofn_parser::parse_ofn(&ofn_typed, &entity_2_label);
    //     println!("Labelled: {}", ofn_labelled);//OFN S-expression after labelling 
    //     println!(""); 
    //}

    let ex = ofn_2_ldtab::util::translate_datatype(&json!("\"asd\""));
    println!("AA {}", ex.as_str().unwrap());


    for triple in axioms.iter() { 
         println!("Triple: {}", triple );//thick triples are given

         let triple_sorted = ofn_2_ldtab::util::sort_value(&triple); 
         println!("Triple Sorted: {}", triple_sorted);//transformation to OFN S-expression 

         let ofn = thick_2_ofn::thick_triple_parser::parse_triple(triple.to_string().as_str());
         println!("OFN S: {}", ofn );//transformation to OFN S-expression 

         let sig = ofn_util::signature::extract_identifiers(&ofn);
         println!("Signature: {:?}", sig );//transformation to OFN S-expression 


         let ofn_sorted = ofn_2_ldtab::util::sort_value(&ofn); 
         println!("OFN Sorted: {}", ofn_sorted );//transformation to OFN S-expression 


         let tt = ofn_2_ldtab::translation::ofn_2_thick_triple(&ofn);
         println!("ThickTriple: {}", tt );//transformation to OFN S-expression 

         //let t = ofn2RDFa::ofn_parser::translate(&ofn);
         //println!("Type: {}", t);//transformation to OFN S-expression
         let ofn_typed  = ofn_typing::ofn_parser::parse_ofn(&ofn, &entity_2_type);
         println!("Typed: {}", ofn_typed);//OFN S-expression after typing


         let ofn_labelled = ofn_labeling::ofn_parser::parse_ofn(&ofn_typed, &entity_2_label);
         let ofn_labelled_untyped = ofn_labeling::ofn_parser::parse_ofn(&ofn, &entity_2_label);

         println!("Labelled: {}", ofn_labelled);//OFN S-expression after labelling
         println!("Untyped Labelled: {}", ofn_labelled_untyped);//OFN S-expression after labelling

         let man_str = ofn_2_man::translation::ofn_2_man(&ofn_typed);
         let man_str_labelled = ofn_2_man::translation::ofn_2_man(&ofn_labelled);

         let man_str_untyped = ofn_2_man::translation::ofn_2_man(&ofn);

         println!("Manchester (untyped): {}", man_str_untyped); 
         println!("Manchester: {}", man_str); 
         println!("Lanchester: {}", man_str_labelled); 

         let thick_triple = ofn_2_thick::ofn_parser::translate_triple(&ofn_typed);
         println!("Thick: {}", thick_triple); 
         let signature = ofn_util::signature::extract(&ofn_typed);
         println!("Signature: {:?}", signature); 
         let prefixes = ofn_util::signature::get_prefixes(&ofn_typed);
         println!("Prefixes: {:?}", prefixes); 
         let iri = ofn_util::curie_to_iri::translate(&ofn_typed,&prefix_2_expansion);
         println!("Test Curie2IRI: {}", iri); 
         let curie = ofn_util::iri_2_curie::translate(&iri,&iri_2_prefix);
         println!("Test IRI2Curie: {}", curie); 
         //let iri = ofn_util::structural_identity::translate(&ofn_typed,&prefix_2_expansion);
         let ofn_rdfa = ofn_2_rdfa::translation::ofn_2_rdfa(&ofn_typed, &entity_2_label);
         println!("Test RDFa: {}", ofn_rdfa); 
         //println!("Object: {}", triple["object"]);
         //let object = triple["object"].to_string();
         //println!("Object: {}", object.as_str());//TODO this stuff doesn't have datatypes
         //let typed_object =  ldtab_2_ofn::parser::parse_thick_triple_object(object.as_str());
         //let object_ofn =  ldtab_2_ofn::class_translation::translate(&typed_object);
         //println!("Object OFN: {}", object_ofn);
         println!("");
    }
}
