use crate::ofn_2_ldtab::constants::*;
use crate::ofn_2_ldtab::property_translation;
use crate::ofn_2_ldtab::util;
use serde_json::json;
use serde_json::Value;

pub fn translate(v: &Value) -> Value {

    match v[0].as_str() {
        Some("ObjectSomeValuesFrom") => translate_some_values_from(v),
        Some("ObjectAllValuesFrom") => translate_all_values_from(v),
        Some("ObjectHasValue") => translate_has_value(v),

        Some("ObjectMinCardinality") => translate_object_min_cardinality(v),
        Some("ObjectMaxCardinality") => translate_object_max_cardinality(v),
        Some("ObjectExactCardinality") => translate_object_exact_cardinality(v),

        Some("DataMinCardinality") => translate_data_min_cardinality(v),
        Some("DataMaxCardinality") => translate_data_max_cardinality(v),
        Some("DataExactCardinality") => translate_data_exact_cardinality(v),

        Some("MinCardinality") => translate_min_cardinality(v),
        Some("MaxCardinality") => translate_max_cardinality(v),
        Some("ExactCardinality") => translate_exact_cardinality(v),

        //TODO: deprecate these
        Some("ObjectMinQualifiedCardinality") => translate_min_qualified_cardinality(v),
        Some("ObjectMaxQualifiedCardinality") => translate_max_qualified_cardinality(v),
        Some("ObjectExactQualifiedCardinality") => translate_exact_qualified_cardinality(v),
        Some("DataMinQualifiedCardinality") => translate_min_qualified_cardinality(v),
        Some("DataMaxQualifiedCardinality") => translate_max_qualified_cardinality(v),
        Some("DataExactQualifiedCardinality") => translate_exact_qualified_cardinality(v),

        Some("ObjectHasSelf") => translate_has_self(v),
        Some("ObjectIntersectionOf") => translate_intersection_of(v),
        Some("ObjectUnionOf") => translate_union_of(v),
        Some("ObjectOneOf") => translate_one_of(v),
        Some("ObjectComplementOf") => translate_complement_of(v),
        Some("ObjectInverseOf") => property_translation::translate_inverse_of(v),

        Some("DataSomeValuesFrom") => translate_some_values_from(v),
        Some("DataAllValuesFrom") => translate_all_values_from(v),
        Some("DataHasValue") => translate_has_value(v),

        Some("DataHasSelf") => translate_has_self(v),
        Some("DataIntersectionOf") => translate_intersection_of(v),
        Some("DataUnionOf") => translate_union_of(v),
        Some("DataOneOf") => translate_one_of(v),
        Some("DataComplementOf") => translate_complement_of(v),

        //type ambiguity
        Some("SomeValuesFrom") => translate_some_values_from(v),
        Some("AllValuesFrom") => translate_all_values_from(v),
        Some("HasValue") => translate_has_value(v),

        //TODO: this cannot be translated without type information
        Some("MinQualifiedCardinality") => translate_min_qualified_cardinality(v),
        Some("MaxQualifiedCardinality") => translate_max_qualified_cardinality(v),
        Some("ExactQualifiedCardinality") => translate_exact_qualified_cardinality(v),

        Some("HasSelf") => translate_has_self(v),
        Some("IntersectionOf") => translate_intersection_of(v),
        Some("UnionOf") => translate_union_of(v),
        Some("OneOf") => translate_one_of(v),
        Some("ComplementOf") => translate_complement_of(v),
        Some("InverseOf") => property_translation::translate_inverse_of(v),

        Some(_) => panic!(),
        //None => owl::OWL::Named(String::from(v.as_str().unwrap())),
        None => translate_named_entity(&v),
    }
}

pub fn translate_named_entity(v: &Value) -> Value {
    let o: String = String::from(v.as_str().unwrap());
    json!(o)
}


pub fn strip_rdf_literal(v: &Value) -> Value {
    match v.as_str() {
        Some(s) if s.starts_with('"') => {
            let bytes = s.as_bytes();
            let mut i = 1usize;
            let mut escaped = false;

            while i < bytes.len() {
                match bytes[i] {
                    b'\\' if !escaped => { escaped = true; }
                    b'"' if !escaped => {
                        // Found closing quote
                        return json!(&s[1..i]);
                    }
                    _ => { escaped = false; }
                }
                i += 1;
            }
            // No closing quote found → return unchanged
            json!(s)
        }
        Some(s) => json!(s), // not a quoted literal
        None => v.clone(),   // not a string
    }
}


pub fn get_object(v: &Value) -> Value {
    let mut o: Value = translate(v);
    let d: String = String::from(util::translate_datatype(&o).as_str().unwrap());
    if d == XSD_STRING ||
       d == XSD_BOOLEAN ||
       d == XSD_INTEGER ||
       d == XSD_DECIMAL ||
       d == XSD_NON_NEGATIVE_INTEGER ||
       d == XSD_NON_POSITIVE_INTEGER ||
       d == XSD_POSITIVE_INTEGER ||
       d == XSD_NEGATIVE_INTEGER ||
       d == XSD_LONG ||
       d == XSD_INT ||
       d == XSD_SHORT ||
       d == XSD_BYTE ||
       d == XSD_UNSIGNED_LONG ||
       d == XSD_UNSIGNED_INT ||
       d == XSD_UNSIGNED_SHORT ||
       d == XSD_UNSIGNED_BYTE ||
       d == OWL_REAL ||
       d == OWL_RATIONAL {
        o = strip_rdf_literal(&o);
    };

    json!({"object" : o,
           "datatype" : d})
}

pub fn get_cardinality_object(v: &Value) -> Value {
    if let Some(s) = v.as_str() {
        if let Some((literal, datatype)) = s.split_once("^^") {
            let trimmed = literal.trim_matches('"');
            json!({
                "object": trimmed,
                "datatype": datatype
            })
        } else if s.chars().all(|c| c.is_ascii_digit()) {
            json!({
                "object": s,
                "datatype": XSD_NON_NEGATIVE_INTEGER
            })
        } else {
            json!({ "error": "Invalid format" })
        }
    } else {
        json!({ "error": "Expected a string" })
    }

}

pub fn translate_some_values_from(v: &Value) -> Value {
    let property_o: Value = get_object(&v[1]);
    let filler_o: Value = get_object(&v[2]);
    let type_o: Value = get_object(&json!(OWL_RESTRICTION));

    //build struct
    let res: Value = json!({RDF_TYPE : vec![type_o],
                             OWL_ON_PROPERTY : vec![property_o],
                             OWL_SOME_VALUES_FROM : vec![filler_o]});

    //return type
    res
}

pub fn translate_all_values_from(v: &Value) -> Value {
    let property_o: Value = get_object(&v[1]);
    let filler_o: Value = get_object(&v[2]);
    let type_o: Value = get_object(&json!(OWL_RESTRICTION));

    json!({RDF_TYPE : vec![type_o],
           OWL_ON_PROPERTY : vec![property_o],
           OWL_ALL_VALUES_FROM: vec![filler_o]})
}

pub fn translate_has_value(v: &Value) -> Value {
    let property_o: Value = get_object(&v[1]);
    let filler_o: Value = get_object(&v[2]);
    let type_o: Value = get_object(&json!(OWL_RESTRICTION));

    json!({RDF_TYPE : vec![type_o],
           OWL_ON_PROPERTY : vec![property_o],
           OWL_HAS_VALUE : vec![filler_o]})
}

pub fn translate_has_self(v: &Value) -> Value {
    let property_o: Value = get_object(&v[1]);
    let has_self_o: Value = get_object(&json!(format!("\"true\"^^{}", XSD_BOOLEAN)));
    let type_o: Value = get_object(&json!(OWL_RESTRICTION));

    json!({RDF_TYPE : vec![type_o],
           OWL_ON_PROPERTY : vec![property_o],
           OWL_HAS_SELF : vec![has_self_o]})
}

pub fn translate_object_min_cardinality(v: &Value) -> Value {
    let cardinality_o: Value = get_cardinality_object(&v[1]);
    let property_o: Value = get_object(&v[2]);
    let type_o: Value = get_object(&json!(OWL_RESTRICTION));

    let ofn = v.as_array().unwrap();
    let is_qualified = ofn.len() == 4;

    if is_qualified {
        let filler_o: Value = get_object(&v[3]);

        json!({RDF_TYPE : vec![type_o],
            OWL_ON_PROPERTY : vec![property_o],
            OWL_MIN_QUALIFIED_CARDINALITY : vec![cardinality_o],
            OWL_ON_CLASS : vec![filler_o]})
    } else {
        json!({RDF_TYPE : vec![type_o],
            OWL_ON_PROPERTY : vec![property_o],
            OWL_MIN_CARDINALITY : vec![cardinality_o]})
    }
}

pub fn translate_min_cardinality(v: &Value) -> Value {
    //TODO: check that this is not a *qualified* cardinality restriction

    let cardinality_o: Value = get_cardinality_object(&v[1]);
    let property_o: Value = get_object(&v[2]);
    let type_o: Value = get_object(&json!(OWL_RESTRICTION));

    json!({RDF_TYPE : vec![type_o],
        OWL_ON_PROPERTY : vec![property_o],
        OWL_MIN_CARDINALITY : vec![cardinality_o]})
}

pub fn translate_data_min_cardinality(v: &Value) -> Value {
    let cardinality_o: Value = get_cardinality_object(&v[1]);
    let property_o: Value = get_object(&v[2]);
    let type_o: Value = get_object(&json!(OWL_RESTRICTION));

    let ofn = v.as_array().unwrap();
    let is_qualified = ofn.len() == 4;

    if is_qualified {
        let filler_o: Value = get_object(&v[3]);

        json!({RDF_TYPE : vec![type_o],
            OWL_ON_PROPERTY : vec![property_o],
            OWL_MIN_QUALIFIED_CARDINALITY : vec![cardinality_o],
            OWL_ON_DATA_RANGE : vec![filler_o]})
    } else {
        json!({RDF_TYPE : vec![type_o],
            OWL_ON_PROPERTY : vec![property_o],
            OWL_MIN_CARDINALITY : vec![cardinality_o]})
    }
}

pub fn translate_min_qualified_cardinality(v: &Value) -> Value {
    let property_o: Value = get_object(&v[1]);
    let cardinality_o: Value = get_cardinality_object(&v[2]);
    let filler_o: Value = get_object(&v[3]);
    let type_o: Value = get_object(&json!(OWL_RESTRICTION));

    json!({RDF_TYPE : vec![type_o],
           OWL_ON_PROPERTY : vec![property_o],
           OWL_MIN_QUALIFIED_CARDINALITY : vec![cardinality_o],
           OWL_ON_CLASS : vec![filler_o] })
}

pub fn translate_max_cardinality(v: &Value) -> Value {
    let cardinality_o: Value = get_cardinality_object(&v[1]);
    let property_o: Value = get_object(&v[2]);
    let type_o: Value = get_object(&json!(OWL_RESTRICTION));

    json!({RDF_TYPE : vec![type_o],
           OWL_ON_PROPERTY : vec![property_o],
           OWL_MAX_CARDINALITY : vec![cardinality_o]})
}

pub fn translate_object_max_cardinality(v: &Value) -> Value {
    let cardinality_o: Value = get_cardinality_object(&v[1]);
    let property_o: Value = get_object(&v[2]);
    let type_o: Value = get_object(&json!(OWL_RESTRICTION));

    let ofn = v.as_array().unwrap();
    let is_qualified = ofn.len() == 4;

    if is_qualified {
        let filler_o: Value = get_object(&v[3]);

        json!({RDF_TYPE : vec![type_o],
            OWL_ON_PROPERTY : vec![property_o],
            OWL_MAX_QUALIFIED_CARDINALITY : vec![cardinality_o],
            OWL_ON_CLASS : vec![filler_o]})
    } else {
        json!({RDF_TYPE : vec![type_o],
            OWL_ON_PROPERTY : vec![property_o],
            OWL_MAX_CARDINALITY : vec![cardinality_o]})
    }
}

pub fn translate_data_max_cardinality(v: &Value) -> Value {
    let cardinality_o: Value = get_cardinality_object(&v[1]);
    let property_o: Value = get_object(&v[2]);
    let type_o: Value = get_object(&json!(OWL_RESTRICTION));

    let ofn = v.as_array().unwrap();
    let is_qualified = ofn.len() == 4;

    if is_qualified {
        let filler_o: Value = get_object(&v[3]);

        json!({RDF_TYPE : vec![type_o],
            OWL_ON_PROPERTY : vec![property_o],
            OWL_MAX_QUALIFIED_CARDINALITY : vec![cardinality_o],
            OWL_ON_DATA_RANGE : vec![filler_o]})
    } else {
        json!({RDF_TYPE : vec![type_o],
            OWL_ON_PROPERTY : vec![property_o],
            OWL_MAX_CARDINALITY : vec![cardinality_o]})
    }
}

pub fn translate_max_qualified_cardinality(v: &Value) -> Value {
    let property_o: Value = get_object(&v[1]);
    let cardinality_o: Value = get_cardinality_object(&v[2]);
    let filler_o: Value = get_object(&v[3]);
    let type_o: Value = get_object(&json!(OWL_RESTRICTION));

    json!({RDF_TYPE : vec![type_o],
           OWL_ON_PROPERTY : vec![property_o],
           OWL_MAX_QUALIFIED_CARDINALITY : vec![cardinality_o],
           OWL_ON_CLASS : vec![filler_o] })
}

pub fn translate_exact_cardinality(v: &Value) -> Value {
    let cardinality_o: Value = get_cardinality_object(&v[1]);
    let property_o: Value = get_object(&v[2]);
    let type_o: Value = get_object(&json!(OWL_RESTRICTION));

    json!({RDF_TYPE : vec![type_o],
           OWL_ON_PROPERTY : vec![property_o],
           OWL_CARDINALITY : vec![cardinality_o]})
}

pub fn translate_object_exact_cardinality(v: &Value) -> Value {
    let cardinality_o: Value = get_cardinality_object(&v[1]);
    let property_o: Value = get_object(&v[2]);
    let type_o: Value = get_object(&json!(OWL_RESTRICTION));

    let ofn = v.as_array().unwrap();
    let is_qualified = ofn.len() == 4;

    if is_qualified {
        let filler_o: Value = get_object(&v[3]);

        json!({RDF_TYPE : vec![type_o],
            OWL_ON_PROPERTY : vec![property_o],
            OWL_QUALIFIED_CARDINALITY : vec![cardinality_o],
            OWL_ON_CLASS : vec![filler_o]})
    } else {
        json!({RDF_TYPE : vec![type_o],
            OWL_ON_PROPERTY : vec![property_o],
            OWL_CARDINALITY : vec![cardinality_o]})
    }
}

pub fn translate_data_exact_cardinality(v: &Value) -> Value {
    let cardinality_o: Value = get_cardinality_object(&v[1]);
    let property_o: Value = get_object(&v[2]);
    let type_o: Value = get_object(&json!(OWL_RESTRICTION));

    let ofn = v.as_array().unwrap();
    let is_qualified = ofn.len() == 4;

    if is_qualified {
        let filler_o: Value = get_object(&v[3]);

        json!({RDF_TYPE : vec![type_o],
            OWL_ON_PROPERTY : vec![property_o],
            OWL_QUALIFIED_CARDINALITY : vec![cardinality_o],
            OWL_ON_DATA_RANGE : vec![filler_o]})
    } else {
        json!({RDF_TYPE : vec![type_o],
            OWL_ON_PROPERTY : vec![property_o],
            OWL_CARDINALITY : vec![cardinality_o]})
    }
}

//TODO: qualifiedCardinality requires ^^xsd:nonNegativeInteger as a datatype
//for cardinalities
pub fn translate_exact_qualified_cardinality(v: &Value) -> Value {
    let property_o: Value = get_object(&v[1]);
    let cardinality_o: Value = get_cardinality_object(&v[2]);
    let filler_o: Value = get_object(&v[3]);
    let type_o: Value = get_object(&json!(OWL_RESTRICTION));

    json!({RDF_TYPE : vec![type_o],
           OWL_ON_PROPERTY : vec![property_o],
           OWL_QUALIFIED_CARDINALITY : vec![cardinality_o],
           OWL_ON_CLASS : vec![filler_o] })
}

pub fn translate_list(v: &[Value]) -> Value {
    let mut list = Vec::new();

    for e in v.iter() {
        let e_object: Value = get_object(&e);
        list.push(e_object);
    }

    Value::Array(list)
}

pub fn translate_intersection_of(v: &Value) -> Value {
    let operands: Value = translate_list(&(v.as_array().unwrap())[1..]);

    let operands_o: Value = json!({"object" : operands,
                                   "datatype" : "_JSONLIST"});

    let type_o: Value = get_object(&json!(OWL_CLASS));

    json!({RDF_TYPE : vec![type_o],
           OWL_INTERSECTION_OF : vec![operands_o]})
}

pub fn translate_union_of(v: &Value) -> Value {
    let operands: Value = translate_list(&(v.as_array().unwrap())[1..]);

    //let operands_o : Value = get_object(operands);
    let operands_o: Value = json!({"object" : operands,
                                    "datatype" : "_JSONLIST"});
    let type_o: Value = get_object(&json!(OWL_CLASS));

    json!({RDF_TYPE : vec![type_o],
           OWL_UNION_OF : vec![operands_o]})
}

pub fn translate_one_of(v: &Value) -> Value {
    let operands: Value = translate_list(&(v.as_array().unwrap())[1..]);

    //let operands_o : Value = get_object(operands);
    let operands_o: Value = json!({"object" : operands,
                                    "datatype" : "_JSONLIST"});
    let type_o: Value = get_object(&json!(OWL_CLASS));

    json!({RDF_TYPE : vec![type_o],
           OWL_ONE_OF : vec![operands_o]})
}

pub fn translate_complement_of(v: &Value) -> Value {
    let argument_o: Value = get_object(&v[1]);
    let type_o: Value = get_object(&json!(OWL_CLASS));

    json!({RDF_TYPE : vec![type_o],
           OWL_COMPLEMENT_OF : vec![argument_o]})
}
