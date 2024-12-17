use regex::Regex;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

pub fn is_annotation(v: &Value) -> bool {
    match v.clone() {
        Value::Array(x) => {
            match x[0].as_str() {
                Some("Annotation") => true,
                //Some("AnnotationList") => true, //NB: this shouldn't occur
                Some(_) => false,
                None => false,
            }
        }
        _ => false,
    }
}

pub fn has_annotation(v: &Value) -> bool {
    match v.clone() {
        Value::Array(x) => is_annotation(&x[1]), //look into second argument
        _ => false,
    }
}

pub fn strip_annotations(v: &Value) -> Value {
    let mut res = Vec::new();
    let original = &v.as_array().unwrap()[0..];
    for element in original {
        if !is_annotation(element) {
            res.push(element.clone());
        }
    }
    Value::Array(res)
}

pub fn get_owl(v: &Value) -> Value {
    strip_annotations(v)
}

pub fn get_annotations(v: &Value) -> Vec<Value> {
    if has_annotation(&v) {
        let mut res = Vec::new();
        let candidates = &v.as_array().unwrap()[0..];
        for candidate in candidates {
            if is_annotation(candidate) {
                res.push(candidate.clone());
            }
        }
        res
    } else {
        Vec::new() //empty vector
    }
}

pub fn is_literal(value: &Value) -> bool {
    // Ensure the Value is a string
    if let Some(s) = value.as_str() {
        // Regex for a simple quoted string
        let simple_string_re = Regex::new("^\"(?s)(.*)\"$").unwrap();
        // Regex for a string with a language tag (e.g., "hello"@en)
        let lang_tag_re = Regex::new("^\"(?s)(.*)\"@(.*)$").unwrap();
        // Regex for a string with a datatype IRI or CURIE (e.g., "42"^^<http://example.com> or "42"^^prefix:suffix)
        let iri_or_curie_re = Regex::new("^\"(?s)(.*)\"\\^\\^(.*)$").unwrap();

        // Check if the string matches any of the forms
        return simple_string_re.is_match(s)
            || lang_tag_re.is_match(s)
            || iri_or_curie_re.is_match(s);
    }
    false
}

pub fn translate_literal(s: &str) -> Value {
    let language_tag = Regex::new("^\"(?s)(.*)\"@(.*)$").unwrap();
    let datatype = Regex::new("^\"(?s)(.*)\"\\^\\^(.*)$").unwrap();
    let plain = Regex::new("^\"(?s)(.*)\"$").unwrap();

    if language_tag.is_match(s) {
        match language_tag.captures(s) {
            Some(x) => {
                let lang = format!("@{}", &x[2]);
                json!({"object" : &x[1],
                       "meta" : "owl:Axiom",
                       "datatype" : lang})
            }
            None => json!("Error"),
        }
    } else if datatype.is_match(s) {
        match datatype.captures(s) {
            Some(x) => {
                let data = format!("{}", &x[2]);
                json!({"object" : &x[1],
                       "meta" : "owl:Axiom",
                        "datatype" : data})
            }
            None => json!("Error"),
        }
    } else if plain.is_match(s) {
        match plain.captures(s) {
            Some(x) => {
                json!({"object" : &x[1],
                       "meta" : "owl:Axiom",
                        "datatype" : "xsd:string"})
            }
            None => json!("Error"),
        }
    } else {
        json!("error")
        //json!({"object" : s, "datatype": "_plain"})
    }
}

pub fn translate_value(v: &Value) -> Value {
    let s = v.as_str().unwrap();

    let literal = Regex::new("^\"(?s)(.+)\"(.*)$").unwrap();
    let uri = Regex::new("^<(.+)>$").unwrap();
    let curie = Regex::new("^(.+):(.+)$").unwrap();

    if literal.is_match(s) {
        translate_literal(s)
    } else if uri.is_match(s) {
        json!({"object" : s,
               "meta" : "owl:Axiom",
               "datatype" : "_IRI"})
    } else if curie.is_match(s) {
        json!({"object" : s,
               "meta" : "owl:Axiom",
               "datatype" : "_IRI"})
    } else {
        json!("ERROR")
    }
}

pub fn translate_annotations(annotations: &Vec<Value>) -> Value {
    let mut property_2_value = HashMap::new();
    for annotation in annotations {
        let a = annotation.as_array().unwrap();
        let property = a[1].clone();
        let property_str = String::from(property.as_str().unwrap());
        let value = translate_value(&a[2].clone());

        //collect all annotations with the same property
        if !property_2_value.contains_key(&property_str) {
            let mut vec = Vec::new();
            vec.push(value);
            property_2_value.insert(property_str, vec);
        } else {
            property_2_value.get_mut(&property_str).unwrap().push(value);
        }
    }
    json!(property_2_value)
}
