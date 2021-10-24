pub mod thick2ofn;
pub mod ofn2thick;
pub mod ofn2man;
pub mod owl;
pub mod ofn_labeling;
pub mod ofn_typing;
pub mod triples;

use serde_json::{Value};

fn main() {

    //example file with thick triples
    let path = String::from("resources/thickOBI.txt"); 

    //get thick triples that encode class expression axioms
    let axioms = triples::parser::extract_class_expression_axioms(&path);

    //get OWL type information and entity labels
    let entity_2_label = ofn_labeling::labeling::extract_labeling(&path);
    let entity_2_type = ofn_typing::typing::extract_typing(&path); 

    for triple in axioms.iter() { 
         println!("Triple: {}", triple );
         let ofn = thick2ofn::thick_triple_parser::parse_triple(triple.to_string().as_str());
         println!("OFN-S: {}", ofn );
         let ofn_typed  = ofn_typing::ofn_parser::parse_ofn(&ofn, &entity_2_type);
         println!("Typed: {}", ofn_typed);
         let ofn_labelled = ofn_labeling::ofn_parser::parse_ofn(&ofn_typed, &entity_2_label);
         println!("Labelled: {}", ofn_labelled);
         let man_str = ofn2man::parser::translate_triple(&ofn_typed);
         let man_str_labelled = ofn2man::parser::translate_triple(&ofn_labelled);
         println!("Manchester: {}", man_str); 
         println!("Lanchester: {}", man_str_labelled); 
         println!("");
    }
}

