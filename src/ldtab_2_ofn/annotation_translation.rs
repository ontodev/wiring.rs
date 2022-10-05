use serde_json::{Value, Map};

pub fn translate_annotations(m : &Map<String, Value>) -> Vec<Value> {

    let mut annotations = Vec::new();

    for (k, v) in m {

        let property = Value::String(String::from(k)); 

        match v {
            //traverse list of objects
            Value::Array(array) =>  {
                for object in array { 

                    let value = encode_value(&object); 
                    //TODO:
                    //(horned OWL currently doesn't support recrusive annotations)
                    //1. check whether object has an annotation
                    //2. translate annotation recursively
                    //3. insert annotation at the second index
                    let annotation_operator = Value::String(String::from("Annotation")); 
                    let annotation = vec![annotation_operator, property.clone(), value];
                    annotations.push(Value::Array(annotation));
                } 
            } 
            _ => panic!("Not an array"), 
        }; 
    }
    annotations 
}

pub fn encode_value(object : &Value) -> Value {

    let value = object["object"].clone(); 
    let datatype = object["datatype"].clone();
    let datatype = datatype.as_str().unwrap();

    let value_str = value.as_str().unwrap(); 

    let value_res = 
    if datatype.eq("_IRI") { 
        format!("{}", value_str) 
    } else if datatype.eq("_plain") {
        format!("\"{}\"", value_str)
    } else if datatype[..1].eq("@") {
        format!("\"{}{}\"", value_str, datatype)
    } else {
        format!("\"{}^^{}\"", value_str, datatype)
    };

    Value::String(value_res) 
}

pub fn translate_annotation(m : &Map<String, Value>) -> Value {

    let operator = Value::String(String::from("Annotation")); 
    let mut annotation = vec![operator];

    for (k, v) in m {

        let property = Value::String(String::from(k)); 
        annotation.push(property);

        match v {
            //traverse list of objects
            Value::Array(array) =>  {
                for object in array {

                    let value = encode_value(&object);

                    //TODO:
                    //(horned OWL currently doesn't support recrusive annotations)
                    //1. check whether object has an annotation
                    //2. translate annotation recursively
                    //3. insert annotation at the second index
                    annotation.push(value);
                } 
            } 
            _ => panic!("Not an array"), 
        }; 
    } 

    Value::Array(annotation) 
} 

pub fn translate(annotation : &Value) -> Vec<Value> { 
     match annotation {
        Value::Object(x) => translate_annotations(&x),
        Value::Null => Vec::new(),
        _ => panic!("Not a valid annotation map"), 
    } 
}
