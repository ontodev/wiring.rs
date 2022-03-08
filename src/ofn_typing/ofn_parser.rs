use serde_json::{Value};
use crate::ofn_typing::axiom_translation as axiom_translation; 
use std::collections::HashMap;
use std::collections::HashSet;
use rusqlite::{params, Connection, Result};
use crate::ofn_util::signature as signature; 

pub fn parse_ofn(v: &Value, m : &HashMap<String, HashSet<String>>) -> Value { 
    match v[0].as_str() {
        Some("SubClassOf") => axiom_translation::translate_subclass_of_axiom(v,m),
        Some("DisjointClasses") => axiom_translation::translate_disjoint_classes_axiom(v,m),
        Some("DisjointUnionOf") => axiom_translation::translate_disjoint_union_of_axiom(v,m),
        Some("EquivalentClasses") => axiom_translation::translate_equivalent_classes_axiom(v,m),
        Some("ThinTriple") => axiom_translation::translate_thin_triple(v),
        Some(_) => panic!(),
        None => Value::String(String::from(v.as_str().unwrap())),
    } 
} 

pub fn get_types(entity: &str, conn: &mut Connection) -> Result<HashSet<String>> {
    let query: String = format!("SELECT * FROM statement WHERE subject='{}' AND predicate='rdf:type'", entity); 
    let mut stmt = conn.prepare(query.as_str())?;
    let mut rows = stmt.query(params![])?;

    let mut types = HashSet::new();

    while let Some(row) = rows.next()? {
        types.insert(row.get(5)?); //5 is object column
    } 
    Ok(types)
}

pub fn parse_ofn_ldtab(v: &Value, m : &mut Connection) -> Value { 
    let signature = signature::extract(&v); 

    let mut entity_2_type = HashMap::new();
    
    for s in signature.iter() {
        let entity = s.as_str().unwrap();//signature elements are always strings
        let types = get_types(&entity, m).unwrap();//always returns a set (possibly empty)
        entity_2_type.insert(entity.to_string(), types); 
    }; 

    match v[0].as_str() {
        Some("SubClassOf") => axiom_translation::translate_subclass_of_axiom(v,&entity_2_type),
        Some("DisjointClasses") => axiom_translation::translate_disjoint_classes_axiom(v,&entity_2_type),
        Some("DisjointUnionOf") => axiom_translation::translate_disjoint_union_of_axiom(v,&entity_2_type),
        Some("EquivalentClasses") => axiom_translation::translate_equivalent_classes_axiom(v,&entity_2_type),
        Some("ThinTriple") => axiom_translation::translate_thin_triple(v),
        Some(_) => panic!(),
        None => Value::String(String::from(v.as_str().unwrap())),
    } 
} 
