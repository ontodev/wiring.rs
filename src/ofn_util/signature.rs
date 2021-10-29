use serde_json::{Value};

//TODO: extract prefixes

//extract signature of an OFN S-expression
pub fn extract(v : &Value) -> Vec<Value> { 
    let owl_operator: String = v[0].to_string(); 

     let res = match owl_operator.as_str() {
         //TODO: ambiguous expressions?
         "\"SubClassOf\"" => translate_subclass_of(v), 
         "\"DisjointClasses\"" => translate_disjoint_classes(v), 
         "\"DisjointUnionOf\"" => translate_disjoint_union_of(v), 
         "\"EquivalentClasses\"" => translate_equivalent_classes(v), 
         "\"ObjectSomeValuesFrom\"" => translate_some_values_from(v), 
         "\"ObjectAllValuesFrom\"" => translate_all_values_from(v), 
         "\"ObjectHasValue\"" => translate_has_value(v), 
         "\"ObjectMinCardinality\"" => translate_min_cardinality(v), 
         "\"ObjectMinQualifiedCardinality\"" => translate_min_qualified_cardinality(v), 
         "\"ObjectMaxCardinality\"" => translate_max_cardinality(v), 
         "\"ObjectMaxQualifiedCardinality\"" => translate_max_qualified_cardinality(v), 
         "\"ObjectExactCardinality\"" => translate_exact_cardinality(v), 
         "\"ObjectExactQualifiedCardinality\"" => translate_exact_qualified_cardinality(v), 
         "\"ObjectHasSelf\"" => translate_has_self(v), 
         "\"ObjectIntersectionOf\"" => translate_intersection_of(v), 
         "\"ObjectUnionOf\"" => translate_union_of(v), 
         "\"ObjectOneOf\"" => translate_one_of(v), 
         "\"ObjectComplementOf\"" => translate_complement_of(v), 
         "\"ObjectInverseOf\"" => translate_inverse_of(v),
         _ => vec!(Value::String(String::from(v.as_str().unwrap()))),
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
