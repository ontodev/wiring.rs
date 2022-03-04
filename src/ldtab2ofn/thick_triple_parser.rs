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

pub fn translate() -> Result<()> {

    let mut conn = Connection::open("a.db")?;
    //TODO: query for specific subjects instead of everything
    let statements = get_statements(&mut conn).expect("Get statements");

    for statement in statements.iter() { 

        let subject = statement.subject.as_str(); 
        let predicate = statement.predicate.as_str(); 
        let object = statement.object.as_str(); 

        let subject_json = parse_thick_triple(subject); 
        //let predicate_json = parse_thick_triple(predicate); 
        let object_json = parse_thick_triple(object);

        let ofn = match predicate {
           "rdfs:subClassOf"  => axiom_translation::translate_subclass_of_axiom(&subject_json, &object_json),
           "owl:equivalentClass" => axiom_translation::translate_equivalent_class(&subject_json, &object_json),
           "owl:AllDisjointClasses" => axiom_translation::translate_disjoint_classes(&object_json),
           "owl:disjointUnionOf" => axiom_translation::translate_disjoint_union(&subject_json,&object_json),
           "owl:disjointWith" => axiom_translation::translate_disjoint_with(&subject_json, &object_json), 
            _ => axiom_translation::translate_thin_triple(subject, predicate, object),
            //_ => Value::String(String::from("Fail")),
        };

        println!("OFN-2: {}", ofn); 
    } 

    Ok(()) 
}

fn parse_thick_triple(triple : &str) -> tt::OWL {
    let triple_json: SResult<tt::OWL> = serde_json::from_str(triple); 

    match triple_json {
        Err(_) => tt::OWL::Named(String::from(triple)),
        Ok(x) => x,
    } 
}

fn get_statements(conn: &mut Connection) -> Result<Vec<Statement>> {

    let mut stmt = conn.prepare("SELECT * FROM statement")?;
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
