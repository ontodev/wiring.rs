use serde_json::{Value};

//extract language tags + datatypes
//command line interface

//extract prefixes occuring in an OFN S-expression
pub fn get_prefixes(v : &Value) -> Vec<String> {
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

//extract signature of an OFN S-expression
pub fn extract(v : &Value) -> Vec<Value> { 

    let res = match v[0].as_str() {
         //TODO: ambiguous expressions?
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
         Some(_) => panic!(),
         None => vec!(Value::String(String::from(v.as_str().unwrap()))),
     };

     dedup_vector(&res)
}

//a vector of Values cannot be sorted.
//Since all Values are Strings,
//we convert the input Vec<Value> into a Vec<&str>
//sort and dedup this Vec<&str>
//and then convert the deduped Vec<&str> back to a Vec<Value>
pub fn dedup_vector(v : &Vec<Value>) -> Vec<Value> {
    let mut str_vec = Vec::new(); 
    for argument in v  {
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

pub fn translate_subclass_of(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 

    let mut subclass = extract(&v[1]); 
    let mut superclass = extract(&v[2]); 

    res.append(&mut subclass);
    res.append(&mut superclass);

    res 
} 

pub fn translate_disjoint_classes(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 

    let arguments  = &(v.as_array().unwrap())[1..]; 
    for argument in arguments  {
        let mut s = extract(&argument); 
        res.append(&mut s);
    }
    res
}

pub fn translate_disjoint_union_of(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 

    let arguments  = &(v.as_array().unwrap())[1..]; 
    for argument in arguments  {
        let mut s = extract(&argument); 
        res.append(&mut s);
    }
    res
}

pub fn translate_equivalent_classes(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 

    let arguments  = &(v.as_array().unwrap())[1..]; 
    for argument in arguments  {
        let mut s = extract(&argument); 
        res.append(&mut s);
    }
    res
}


pub fn translate_some_values_from(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 

    let mut property = extract(&v[1]); 
    let mut filler = extract(&v[2]); 

    res.append(&mut property);
    res.append(&mut filler);

    res 
} 

pub fn translate_all_values_from(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 

    let mut property = extract(&v[1]); 
    let mut filler = extract(&v[2]); 

    res.append(&mut property);
    res.append(&mut filler);

    res 
} 

pub fn translate_has_value(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 

    let mut property = extract(&v[1]); 
    let mut filler = extract(&v[2]); 

    res.append(&mut property);
    res.append(&mut filler);

    res 
} 

pub fn translate_min_cardinality(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 
    let mut property = extract(&v[1]); 
    res.append(&mut property);

    res 
} 


pub fn translate_min_qualified_cardinality(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 
    let mut property = extract(&v[1]); 
    let mut filler = extract(&v[3]); 
    res.append(&mut property);
    res.append(&mut filler);

    res 
} 

pub fn translate_max_cardinality(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 
    let mut property = extract(&v[1]); 
    res.append(&mut property);

    res 
} 


pub fn translate_max_qualified_cardinality(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 
    let mut property = extract(&v[1]); 
    let mut filler = extract(&v[3]); 
    res.append(&mut property);
    res.append(&mut filler);

    res 
} 

pub fn translate_exact_cardinality(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 
    let mut property = extract(&v[1]); 
    res.append(&mut property);

    res 
} 


pub fn translate_exact_qualified_cardinality(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 
    let mut property = extract(&v[1]); 
    let mut filler = extract(&v[3]); 
    res.append(&mut property);
    res.append(&mut filler);

    res 
} 

pub fn translate_has_self(v : &Value) -> Vec<Value> {
    let mut res = Vec::new(); 
    let mut property = extract(&v[1]); 
    res.append(&mut property);
    res 
}

pub fn translate_intersection_of(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 

    let arguments  = &(v.as_array().unwrap())[1..]; 
    for argument in arguments  {
        let mut s = extract(&argument); 
        res.append(&mut s);
    }
    res
}

pub fn translate_union_of(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 

    let arguments  = &(v.as_array().unwrap())[1..]; 
    for argument in arguments  {
        let mut s = extract(&argument); 
        res.append(&mut s);
    }
    res
}

pub fn translate_one_of(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 

    let arguments  = &(v.as_array().unwrap())[1..]; 
    for argument in arguments  {
        let mut s = extract(&argument); 
        res.append(&mut s);
    }
    res
}

pub fn translate_complement_of(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 
    let mut property = extract(&v[1]); 
    res.append(&mut property);

    res 
} 

pub fn translate_inverse_of(v : &Value) -> Vec<Value> {

    let mut res = Vec::new(); 
    let mut property = extract(&v[1]); 
    res.append(&mut property);

    res 
} 
