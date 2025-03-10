use serde_json::{Map, Value};
use std::collections::HashSet;

//extract language tags + datatypes
//command line interface

pub fn get_iris_from_object(ldtab_object: &Map<String, Value>, iris: &mut HashSet<String>) {
    for k in ldtab_object.keys() {
        if k != "object" && k != "datatype" {
            iris.insert(k.clone());
        }
    }

    if ldtab_object.contains_key("datatype") {
        match ldtab_object.get("datatype") {
            Some(x) => {
                //get datatype ...
                match x {
                    Value::String(y) => {
                        //... as a string ...
                        match y.as_str() {
                            //... to check its value
                            "_IRI" => {
                                let object = ldtab_object.get("object").unwrap();
                                iris.insert(String::from(object.as_str().unwrap()));
                            }
                            "_JSONMAP" => {
                                let object = ldtab_object.get("object").unwrap();
                                get_iris(object, iris);
                            }
                            "_JSONLIST" => {
                                let list = ldtab_object.get("object").unwrap();
                                for v in list.as_array().unwrap() {
                                    get_iris(v, iris);
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => panic!(), //TODO: error handling
        }
    } else {
        for v in ldtab_object.values() {
            get_iris(&v, iris);
        }
    }
}

pub fn get_iris_from_array(ldtab_array: &Vec<Value>, iris: &mut HashSet<String>) {
    for a in ldtab_array {
        get_iris(&a, iris);
    }
}

//extract IRIs from a thick triple
pub fn get_iris(ldtab_thick_triple_object: &Value, iris: &mut HashSet<String>) {
    match ldtab_thick_triple_object {
        Value::Array(a) => get_iris_from_array(&a, iris),
        Value::Object(o) => get_iris_from_object(&o, iris),
        _ => {}
    }
}

//extract prefixes occuring in an OFN S-expression
pub fn get_prefixes(v: &Value) -> Vec<String> {
    let mut prefixes = Vec::new();
    let sig = extract(v);
    for s in sig {
        let h = s.as_str().unwrap();
        let split: Vec<&str> = h.split(":").collect();
        let p = split[0];
        prefixes.push(String::from(p));
    }
    prefixes.sort_unstable();
    prefixes.dedup();
    prefixes
}

//NB: this returns both IRIs as well as literals
pub fn extract_identifiers(v: &Value) -> Vec<Value> {
    match v {
        Value::String(_x) => vec![v.clone()],
        Value::Array(array) => {
            let mut res = Vec::new();
            for a in &array[1..] {
                //slice drops operator
                //recurse
                let elements = extract_identifiers(a);
                res.extend(elements);
            }
            res
        }
        _ => panic!("Not a valid OFN S-expression"),
    }
}

//extract signature of an OFN S-expression
pub fn extract(v: &Value) -> Vec<Value> {
    let res = match v[0].as_str() {
        Some("SubClassOf") => translate_subclass_of(v),
        Some("DisjointClasses") => translate_disjoint_classes(v),
        Some("DisjointUnionOf") => translate_disjoint_union_of(v),
        Some("EquivalentClasses") => translate_equivalent_classes(v),
        Some("ObjectSomeValuesFrom") => translate_some_values_from(v),
        Some("DataSomeValuesFrom") => translate_some_values_from(v),
        Some("ObjectAllValuesFrom") => translate_all_values_from(v),
        Some("ObjectHasValue") => translate_has_value(v),
        Some("ObjectMinCardinality") => translate_min_cardinality(v),
        Some("ObjectMinQualifiedCardinality") => translate_min_qualified_cardinality(v),
        Some("ObjectMaxCardinality") => translate_max_cardinality(v),
        Some("ObjectMaxQualifiedCardinality") => translate_max_qualified_cardinality(v),
        Some("ObjectExactCardinality") => translate_exact_cardinality(v),
        Some("DataExactCardinality") => translate_exact_cardinality(v),
        Some("ObjectExactQualifiedCardinality") => translate_exact_qualified_cardinality(v),
        Some("ObjectHasSelf") => translate_has_self(v),
        Some("ObjectIntersectionOf") => translate_intersection_of(v),
        Some("ObjectUnionOf") => translate_union_of(v),
        Some("ObjectOneOf") => translate_one_of(v),
        Some("ObjectComplementOf") => translate_complement_of(v),
        Some("ObjectInverseOf") => translate_inverse_of(v),

        //ambiguous expressions
        Some("SomeValuesFrom") => translate_some_values_from(v),
        Some("AllValuesFrom") => translate_all_values_from(v),
        Some("HasValue") => translate_has_value(v),
        Some("MinCardinality") => translate_min_cardinality(v),
        Some("MaxCardinality") => translate_max_cardinality(v),
        Some("ExactCardinality") => translate_exact_cardinality(v),
        Some("IntersectionOf") => translate_intersection_of(v),
        Some("UnionOf") => translate_union_of(v),
        Some("OneOf") => translate_one_of(v),
        Some("ComplementOf") => translate_complement_of(v),

        Some("ThinTriple") => translate_thin_triple(v),
        Some(_) => panic!(),
        None => vec![Value::String(String::from(v.as_str().unwrap()))],
    };

    dedup_vector(&res)
}

//a vector of Values cannot be sorted.
//Since all Values are Strings,
//we convert the input Vec<Value> into a Vec<&str>
//sort and dedup this Vec<&str>
//and then convert the deduped Vec<&str> back to a Vec<Value>
pub fn dedup_vector(v: &Vec<Value>) -> Vec<Value> {
    let mut str_vec = Vec::new();
    for argument in v {
        let s = argument.as_str().unwrap();
        str_vec.push(s);
    }

    str_vec.sort_unstable();
    str_vec.dedup();

    //convert to vec of Value
    let mut res = Vec::new();
    for s in str_vec {
        res.push(Value::String(String::from(s)));
    }

    res
}

pub fn translate_thin_triple(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();

    let mut subject = extract(&v[1]);
    let mut predicate = extract(&v[2]);
    let mut object = extract(&v[3]);

    res.append(&mut subject);
    res.append(&mut predicate);
    res.append(&mut object);

    res
}

pub fn translate_subclass_of(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();

    let mut subclass = extract(&v[1]);
    let mut superclass = extract(&v[2]);

    res.append(&mut subclass);
    res.append(&mut superclass);

    res
}

pub fn translate_disjoint_classes(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();

    let arguments = &(v.as_array().unwrap())[1..];
    for argument in arguments {
        let mut s = extract(&argument);
        res.append(&mut s);
    }
    res
}

pub fn translate_disjoint_union_of(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();

    let arguments = &(v.as_array().unwrap())[1..];
    for argument in arguments {
        let mut s = extract(&argument);
        res.append(&mut s);
    }
    res
}

pub fn translate_equivalent_classes(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();

    let arguments = &(v.as_array().unwrap())[1..];
    for argument in arguments {
        let mut s = extract(&argument);
        res.append(&mut s);
    }
    res
}

pub fn translate_some_values_from(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();

    let mut property = extract(&v[1]);
    let mut filler = extract(&v[2]);

    res.append(&mut property);
    res.append(&mut filler);

    res
}

pub fn translate_all_values_from(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();

    let mut property = extract(&v[1]);
    let mut filler = extract(&v[2]);

    res.append(&mut property);
    res.append(&mut filler);

    res
}

pub fn translate_has_value(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();

    let mut property = extract(&v[1]);
    let mut filler = extract(&v[2]);

    res.append(&mut property);
    res.append(&mut filler);

    res
}

pub fn translate_min_cardinality(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();
    let mut property = extract(&v[1]);
    res.append(&mut property);

    res
}

pub fn translate_min_qualified_cardinality(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();
    let mut property = extract(&v[1]);
    let mut filler = extract(&v[3]);
    res.append(&mut property);
    res.append(&mut filler);

    res
}

pub fn translate_max_cardinality(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();
    let mut property = extract(&v[1]);
    res.append(&mut property);

    res
}

pub fn translate_max_qualified_cardinality(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();
    let mut property = extract(&v[1]);
    let mut filler = extract(&v[3]);
    res.append(&mut property);
    res.append(&mut filler);

    res
}

pub fn translate_exact_cardinality(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();
    let mut property = extract(&v[1]);
    res.append(&mut property);

    res
}

pub fn translate_exact_qualified_cardinality(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();
    let mut property = extract(&v[1]);
    let mut filler = extract(&v[3]);
    res.append(&mut property);
    res.append(&mut filler);

    res
}

pub fn translate_has_self(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();
    let mut property = extract(&v[1]);
    res.append(&mut property);
    res
}

pub fn translate_intersection_of(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();

    let arguments = &(v.as_array().unwrap())[1..];
    for argument in arguments {
        let mut s = extract(&argument);
        res.append(&mut s);
    }
    res
}

pub fn translate_union_of(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();

    let arguments = &(v.as_array().unwrap())[1..];
    for argument in arguments {
        let mut s = extract(&argument);
        res.append(&mut s);
    }
    res
}

pub fn translate_one_of(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();

    let arguments = &(v.as_array().unwrap())[1..];
    for argument in arguments {
        let mut s = extract(&argument);
        res.append(&mut s);
    }
    res
}

pub fn translate_complement_of(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();
    let mut property = extract(&v[1]);
    res.append(&mut property);

    res
}

pub fn translate_inverse_of(v: &Value) -> Vec<Value> {
    let mut res = Vec::new();
    let mut property = extract(&v[1]);
    res.append(&mut property);

    res
}
