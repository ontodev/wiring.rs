use rusqlite::{params, Connection, Result};
use serde_json::{Value, Result as SResult};
use crate::owl::thick_triple as tt;
use crate::ldtab2ofn::axiom_translation as axiom_translation; 

#[derive(Debug)]
struct Statement {
    assertion: u32,
    retraction: u32,
    graph: String,
    subject: String,
    predicate: String,
    object: String,
    datatype: String,
    annotation: Option<String>,
}

pub fn get_entity(databse: &str, subject: &str) -> Vec<Value> {

    let mut conn = Connection::open(databse).unwrap();
    let statements = get_statements(&mut conn, subject).expect("Get statements");
    let mut axioms = Vec::new();

    for statement in statements.iter() { 

        let subject = statement.subject.as_str(); 
        let predicate = statement.predicate.as_str(); 
        let object = statement.object.as_str(); 

        let subject_json = parse_thick_triple(subject); 
        let object_json = parse_thick_triple(object);

        let ofn_expression = match predicate {
           "rdfs:subClassOf"  => axiom_translation::translate_subclass_of_axiom(&subject_json, &object_json),
           "owl:equivalentClass" => axiom_translation::translate_equivalent_class(&subject_json, &object_json),
           "owl:AllDisjointClasses" => axiom_translation::translate_disjoint_classes(&object_json),
           "owl:disjointUnionOf" => axiom_translation::translate_disjoint_union(&subject_json,&object_json),
           "owl:disjointWith" => axiom_translation::translate_disjoint_with(&subject_json, &object_json), 
            _ => axiom_translation::translate_thin_triple(subject, predicate, object),
        };

        axioms.push(ofn_expression);
    }
    axioms
}

fn parse_thick_triple(triple : &str) -> tt::OWL {
    let triple_json: SResult<tt::OWL> = serde_json::from_str(triple); 

    match triple_json {
        Err(_) => tt::OWL::Named(String::from(triple)),
        Ok(x) => x,
    } 
}

fn get_statements(conn: &mut Connection, subject: &str) -> Result<Vec<Statement>> {

    let query: String = format!("SELECT * FROM statement WHERE subject='{}'", subject);

    let mut stmt = conn.prepare(query.as_str())?;
    let mut rows = stmt.query(params![])?;
    let mut prefixes = Vec::new();
    while let Some(row) = rows.next()? {
        prefixes.push(Statement {
            assertion: row.get(0)?,
            retraction: row.get(1)?,
            graph: row.get(2)?,
            subject: row.get(3)?,
            predicate: row.get(4)?,
            object: row.get(5)?,
            datatype: row.get(6)?,
            annotation: row.get(7)?,
        });
    }
    Ok(prefixes)
}
