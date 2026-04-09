use regex::Regex;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::LazyLock;

use crate::constants::*;

static RE_SIMPLE_STRING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^\"(?s)(.*)\"$").unwrap());
static RE_LANG_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^\"(?s)(.*)\"@(.*)$").unwrap());
static RE_TYPED_LITERAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^\"(?s)(.*)\"\\^\\^(.*)$").unwrap());
static RE_LITERAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^\"(?s)(.+)\"(.*)$").unwrap());
static RE_URI: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^<(.+)>$").unwrap());
static RE_CURIE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^(.+):(.+)$").unwrap());

pub fn is_annotation(v: &Value) -> bool {
    matches!(
        v,
        Value::Array(xs)
            if xs.get(0)
                 .and_then(Value::as_str)
                 == Some("Annotation")
    )
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
        // Check if the string matches any of the literal forms
        return RE_SIMPLE_STRING.is_match(s)
            || RE_LANG_TAG.is_match(s)
            || RE_TYPED_LITERAL.is_match(s);
    }
    false
}

pub fn translate_literal(s: &str) -> Value {

    if s.starts_with('"') && !s.ends_with('"') {
        // typed literal
        if let Some((lexical_form, datatype)) = s.rsplit_once("\"^^") {
            let text = lexical_form[1..].to_string(); // drop the leading "
            return json!({
                "object":   text,
                "meta":     OWL_AXIOM,
                "datatype": datatype
            });
        }

        // literal with language tag
        if let Some((lexical_form, lang)) = s.rsplit_once("\"@") {
            let text = lexical_form[1..].to_string(); // drop the leading "
            return json!({
                "object":   text,
                "meta":     OWL_AXIOM,
                "datatype": format!("@{}", lang)
            });
        }
    }

    // plain literal
    if s.starts_with('"') && s.ends_with('"') {
        // remove quotes
        let text = &s[1..s.len()-1];
        return json!({
            "object":   text,
            "meta":     OWL_AXIOM,
            "datatype": XSD_STRING
        });
    }

    // fallback: treat the entire thing as xsd:string
    // TODO: this case should not occur
    json!({
        "object":   s,
        "meta":     OWL_AXIOM,
        "datatype": XSD_STRING
    })
}



pub fn translate_value(v: &Value) -> Value {
    let s = v.as_str().unwrap();

    if RE_LITERAL.is_match(s) {
        translate_literal(s)
    } else if RE_URI.is_match(s) {
        json!({"object" : s,
               "meta" : OWL_AXIOM,
               "datatype" : LDTAB_IRI})
    } else if RE_CURIE.is_match(s) {
        json!({"object" : s,
               "meta" : OWL_AXIOM,
               "datatype" : LDTAB_IRI})
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
