use rusqlite::{params, Connection, Result};
use serde_json::{Value};
use crate::owl::thick_triple as tt;
use crate::ldtab2ofn::class_translation as class_translation; 

#[derive(Debug)]
struct Statement {
    assertion: u32,
    retraction: u32,
    graph: String,
    subject: String,
    predicate: String,
    object: String,
    datatype: String,
    annotation: String,
}

pub fn translate() -> Result<()> {
    let mut conn = Connection::open("a.db")?;
    let statements = get_statements(&mut conn).expect("Get statements");

    for statement in statements.iter() {

        if !statement.predicate.eq("\"unknown\"") {

            //println!("Subject {:?}", statement); 


            //todo: skip if predicate is "unknown"?
            let subject = statement.subject.as_str();
            let subject_json: tt::OWL = serde_json::from_str(subject).unwrap(); 

            let predicate = statement.predicate.as_str();
            let predicate_json: tt::OWL = serde_json::from_str(predicate).unwrap(); 

            let object = statement.object.as_str();
            let object_json: tt::OWL = serde_json::from_str(object).unwrap(); 

            if predicate.eq("\"rdfs:subClassOf\""){

                let lhs : Value = class_translation::translate(&subject_json);
                let rhs: Value = class_translation::translate(&object_json); 
                let operator = Value::String(String::from("SubClassOf"));
                let v = vec![operator, lhs, rhs];
                let res = Value::Array(v);
                println!("{:?}", res);

            }


            //println!("Subject {:?}", subject_json); 
            //println!("Predicate {:?}", predicate_json); 
            //println!("Object {:?}", object_json); 
            //println!("\n");

        } else {
            println!("BLANK NODE WITHOUT TYPE {:?}", statement); 
            let bb = statement.object.as_str();
            //can only parse this as a serde value 
            let aa : Value = serde_json::from_str(bb).unwrap();
            println!("{:?}", aa);
        }
    } 

    //println!("In {:?}", statements[1].subject);
    //let sub = statements[1].subject.as_str();
    //let subclass: tt::OWL = serde_json::from_str(sub).unwrap(); 
    //println!("Out {:?}", subclass); 

    Ok(()) 
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
