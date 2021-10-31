use serde_json::{Value};
use std::collections::HashMap;

//Note that (thick) triples are not OWL
pub fn translate(v : &Value,
                 m : &HashMap<String, String>,
                 t: &dyn Fn(&Value, &HashMap<String, String>) -> Value)
    -> Value {

    let owl_operator: String = v[0].to_string();

     //TODO: ambiguous expressions?
     match owl_operator.as_str() {
         "\"SubClassOf\"" => translate_subclass_of(v,m,t), 
         "\"DisjointClasses\"" => translate_disjoint_classes(v,m,t), 
         "\"DisjointUnionOf\"" => translate_disjoint_union_of(v,m,t), 
         "\"EquivalentClasses\"" => translate_equivalent_classes(v,m,t), 
         "\"ObjectSomeValuesFrom\"" => translate_some_values_from(v,m,t), 
         "\"ObjectAllValuesFrom\"" => translate_all_values_from(v,m,t), 
         "\"ObjectHasValue\"" => translate_has_value(v,m,t), 
         "\"ObjectMinCardinality\"" => translate_min_cardinality(v,m,t), 
         "\"ObjectMinQualifiedCardinality\"" => translate_min_qualified_cardinality(v,m,t), 
         "\"ObjectMaxCardinality\"" => translate_max_cardinality(v,m,t), 
         "\"ObjectMaxQualifiedCardinality\"" => translate_max_qualified_cardinality(v,m,t), 
         "\"ObjectExactCardinality\"" => translate_exact_cardinality(v,m,t), 
         "\"ObjectExactQualifiedCardinality\"" => translate_exact_qualified_cardinality(v,m,t), 
         "\"ObjectHasSelf\"" => translate_has_self(v,m,t), 
         "\"ObjectIntersectionOf\"" => translate_intersection_of(v,m,t), 
         "\"ObjectUnionOf\"" => translate_union_of(v,m,t), 
         "\"ObjectOneOf\"" => translate_one_of(v,m,t), 
         "\"ObjectComplementOf\"" => translate_complement_of(v,m,t), 
         "\"ObjectInverseOf\"" => translate_inverse_of(v,m,t), 
         _ => t(v,m),//substitute labels for entities
     }
} 

pub fn translate_subclass_of(v : &Value,
                             m : &HashMap<String,String>,
                             t : &dyn Fn(&Value, &HashMap<String, String>) -> Value
                             ) -> Value {

    //translate OWL classes
    let subclass : Value = translate(&v[1], m, t);
    let superclass : Value = translate(&v[2], m, t); 

    let operator = Value::String(String::from("SubClassOf"));
    let v = vec![operator, subclass, superclass];
    Value::Array(v) 
}

pub fn translate_disjoint_classes(v : &Value,
                                  m : &HashMap<String,String>, 
                                  t : &dyn Fn(&Value, &HashMap<String, String>) -> Value
                                  ) -> Value {

    let mut operands : Value = translate_list(&(v.as_array().unwrap())[1..], m,t); 
    let operator = Value::String(String::from("DisjointClasses"));
    let mut disjoint = vec![operator];
    let arguments = operands.as_array_mut().unwrap();
    disjoint.append(arguments);
    Value::Array(disjoint.to_vec()) 
}

pub fn translate_disjoint_union_of(v : &Value,
                                   m : &HashMap<String,String>, 
                                   t : &dyn Fn(&Value, &HashMap<String, String>) -> Value
                                   ) -> Value {

    let lhs = translate(&v[1], m, t);
    let mut operands : Value = translate_list(&(v.as_array().unwrap())[2..], m, t); 
    let operator = Value::String(String::from("DisjointUnionOf"));

    let mut union = vec![operator, lhs];
    let arguments = operands.as_array_mut().unwrap();
    union.append(arguments);
    Value::Array(union.to_vec())
}


pub fn translate_equivalent_classes(v : &Value,
                                    m : &HashMap<String,String>,
                                    t : &dyn Fn(&Value, &HashMap<String, String>) -> Value
                                    ) -> Value {

    let mut operands : Value = translate_list(&(v.as_array().unwrap())[1..],m,t); 
    let operator = Value::String(String::from("EquivalentClasses"));
    let mut equivalent = vec![operator];
    let arguments = operands.as_array_mut().unwrap();
    equivalent.append(arguments);
    Value::Array(equivalent.to_vec()) 
}

pub fn translate_some_values_from(v : &Value,
                                  m : &HashMap<String, String>,
                                  t : &dyn Fn(&Value, &HashMap<String, String>) -> Value
                                  ) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m,t); 
    let filler: Value = translate(&v[2],m,t); 

    let expression = vec![operator, property, filler];
    Value::Array(expression)
} 

pub fn translate_all_values_from(v : &Value,
                                 m : &HashMap<String, String>,
                                 t : &dyn Fn(&Value, &HashMap<String, String>) -> Value
                                 ) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m,t); 
    let filler: Value = translate(&v[2],m,t); 

    let expression = vec![operator, property, filler];
    Value::Array(expression) 
} 

pub fn translate_has_value(v : &Value,
                           m : &HashMap<String, String>, 
                           t : &dyn Fn(&Value, &HashMap<String, String>) -> Value
                           ) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m,t); 
    let filler: Value = translate(&v[2],m,t); 
    let expression = vec![operator, property, filler];
    Value::Array(expression) 
} 

pub fn translate_has_self(v : &Value,
                          m : &HashMap<String, String>,
                          t : &dyn Fn(&Value, &HashMap<String, String>) -> Value 
                          ) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m,t); 
    let expression = vec![operator, property];
    Value::Array(expression) 
} 

pub fn translate_min_cardinality(v : &Value,
                                 m : &HashMap<String, String>,
                                 t : &dyn Fn(&Value, &HashMap<String, String>) -> Value 
                                 ) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m,t); 
    let cardinality: Value = translate(&v[2],m,t); 
    let expression = vec![operator, property, cardinality];
    Value::Array(expression) 
} 

pub fn translate_min_qualified_cardinality(v : &Value,
                                           m : &HashMap<String, String>,
                                           t : &dyn Fn(&Value, &HashMap<String, String>) -> Value 
                                           ) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m,t); 
    let cardinality: Value = translate(&v[2],m,t); 
    let filler: Value = translate(&v[3],m,t); 
    let expression = vec![operator, property, cardinality, filler];
    Value::Array(expression) 
} 

pub fn translate_max_cardinality(v : &Value,
                                 m : &HashMap<String, String>,
                                 t : &dyn Fn(&Value, &HashMap<String, String>) -> Value 
                                 ) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m,t); 
    let cardinality: Value = translate(&v[2],m,t); 

    let expression = vec![operator, property, cardinality];
    Value::Array(expression) 
} 

pub fn translate_max_qualified_cardinality(v : &Value,
                                           m : &HashMap<String, String>,
                                           t : &dyn Fn(&Value, &HashMap<String, String>) -> Value 
                                           ) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m,t); 
    let cardinality: Value = translate(&v[2],m,t); 
    let filler: Value = translate(&v[3],m,t); 
    let expression = vec![operator, property, cardinality, filler];
    Value::Array(expression) 
} 

pub fn translate_exact_cardinality(v : &Value,
                                   m : &HashMap<String, String>,
                                   t : &dyn Fn(&Value, &HashMap<String, String>) -> Value 
                                   ) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m,t); 
    let cardinality: Value = translate(&v[2],m,t); 

    let expression = vec![operator, property, cardinality];
    Value::Array(expression) 
} 

pub fn translate_exact_qualified_cardinality(v : &Value,
                                             m : &HashMap<String, String>,
                                             t: &dyn Fn(&Value, &HashMap<String, String>) -> Value ) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m,t); 
    let cardinality: Value = translate(&v[2],m,t); 
    let filler: Value = translate(&v[3],m,t); 

    let expression = vec![operator, property, cardinality, filler];
    Value::Array(expression) 
} 

pub fn translate_list(v : &[Value],
                      m : &HashMap<String, String>,
                      t : &dyn Fn(&Value, &HashMap<String, String>) -> Value
                      ) -> Value {

    let mut res = Vec::new();
    for argument in v {
        let t: Value = translate(&argument,m,t); 
        res.push(t) 
    }
    Value::Array(res) 
}

pub fn translate_intersection_of(v : &Value,
                                 m : &HashMap<String, String>,
                                 t : &dyn Fn(&Value, &HashMap<String, String>) -> Value 
                                 ) -> Value {

    let operator: Value = v[0].clone();
    let mut operands : Value = translate_list(&(v.as_array().unwrap())[1..],m,t);

    let mut res = Vec::new();
    let r = operands.as_array_mut().unwrap();
    res.push(operator);
    res.append(r); 
    Value::Array(res) 
} 

pub fn translate_union_of(v : &Value,
                          m : &HashMap<String, String>,
                          t : &dyn Fn(&Value, &HashMap<String, String>) -> Value 
                          ) -> Value {

    let operator: Value = v[0].clone();
    let mut operands : Value = translate_list(&(v.as_array().unwrap())[1..],m,t);

    let mut res = Vec::new();
    let r = operands.as_array_mut().unwrap();
    res.push(operator);
    res.append(r); 
    Value::Array(res) 
} 

pub fn translate_one_of(v : &Value,
                        m : &HashMap<String, String>,
                        t : &dyn Fn(&Value, &HashMap<String, String>) -> Value 
                        ) -> Value {

    let operator: Value = v[0].clone();
    let mut operands : Value = translate_list(&(v.as_array().unwrap())[1..],m,t);

    let mut res = Vec::new();
    let r = operands.as_array_mut().unwrap();
    res.push(operator);
    res.append(r); 
    Value::Array(res) 
} 

pub fn translate_complement_of(v : &Value,
                               m : &HashMap<String, String>, 
                               t : &dyn Fn(&Value, &HashMap<String, String>) -> Value 
                               ) -> Value {

    let operator: Value = v[0].clone(); 
    let argument: Value = translate(&v[1],m,t); 
    let v = vec![operator, argument];
    Value::Array(v) 
} 

pub fn translate_inverse_of(v : &Value,
                            m : &HashMap<String, String>,
                            t : &dyn Fn(&Value, &HashMap<String, String>) -> Value 
                            ) -> Value {

    let operator: Value = v[0].clone(); 
    let property = translate(&v[1],m,t); 
    let v = vec![operator, property]; 
    Value::Array(v) 
} 
