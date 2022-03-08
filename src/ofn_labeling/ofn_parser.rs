use serde_json::{Value};
use crate::ofn_labeling::axiom_translation as axiom_translation; 
use std::collections::HashMap;
use std::collections::HashSet;
use rusqlite::{params, Connection, Result}; 
use crate::ofn_util::signature as signature; 

//TODO: there is no 'real' parsing going on
//TODO: could refactor the whole labelling translation into a single file
//since OFN-S expressions are handled in a uniform manner 
pub fn parse_ofn(v: &Value, m : &HashMap<String,String>) -> Value { 

    match v[0].as_str() {
        Some("SubClassOf") => axiom_translation::translate_subclass_of_axiom(v,m),
        Some("DisjointClasses") => axiom_translation::translate_disjoint_classes_axiom(v,m),
        Some("DisjointUnionOf") => axiom_translation::translate_disjoint_union_of_axiom(v,m),
        Some("EquivalentClasses") => axiom_translation::translate_equivalent_classes_axiom(v,m),
        Some("ThinTriple") => axiom_translation::translate_thin_triple(v,m),
        Some(_) => panic!(),
        None => Value::String(String::from(v.as_str().unwrap())),
    }
}

pub fn get_labels(entity: &str, conn: &mut Connection) -> Result<String> {
    let query: String = format!("SELECT * FROM statement WHERE subject='{}' AND predicate='rdfs:label'", entity); 
    let mut stmt = conn.prepare(query.as_str())?;
    let mut rows = stmt.query(params![])?;

    //return first found label (assuming labels are unique) 
    let row = rows.next()?;
    let label : String = row.unwrap().get(5)?;
    Ok(label) 
}


pub fn parse_ofn_ldtab(v: &Value, m : &mut Connection) -> Value { 
    let signature = signature::extract(&v); 

    let mut entity_2_label = HashMap::new();
    
    for s in signature.iter() {
        let entity = s.as_str().unwrap();//signature elements are always strings
        let label = get_labels(&entity, m).unwrap();//always returns a set (possibly empty)
        entity_2_label.insert(entity.to_string(), label); 
    }; 

    match v[0].as_str() {
        Some("SubClassOf") => axiom_translation::translate_subclass_of_axiom(v,&entity_2_label),
        Some("DisjointClasses") => axiom_translation::translate_disjoint_classes_axiom(v,&entity_2_label),
        Some("DisjointUnionOf") => axiom_translation::translate_disjoint_union_of_axiom(v,&entity_2_label),
        Some("EquivalentClasses") => axiom_translation::translate_equivalent_classes_axiom(v,&entity_2_label),
        Some("ThinTriple") => axiom_translation::translate_thin_triple(v,&entity_2_label),
        Some(_) => panic!(),
        None => Value::String(String::from(v.as_str().unwrap())),
    } 
} 
