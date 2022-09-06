use serde_json::{Value};
use serde_json::json;
use crate::ofn_typing::property_translation as property_translation;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn translate(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

     match v[0].as_str() {
         Some("SomeValuesFrom") => translate_some_values_from(v,m), 
         Some("AllValuesFrom") =>  translate_all_values_from(v,m), 
         Some("HasValue") => translate_has_value(v,m), 
         Some("MinCardinality") => translate_min_cardinality(v,m), 
         Some("MaxCardinality") => translate_max_cardinality(v,m), 
         Some("ExactCardinality") => translate_exact_cardinality(v,m), 

         //TODO: (note DataIntersection is not a class expression)
         //NB: we are currently relying on an exclicit typing for the followiong:
         //Some("IntersectionOf") => id(v,m), 
         //Some("UnionOf") => id(v,m), 
         //Some("OneOf") => id(v,m), 
         //Some("ComplementOf") => id(v,m), 

         //declarations
         Some("Class") => id(v,m),
         Some("Datatype") => id(v,m),
         Some("ObjectProperty") => id(v,m),
         Some("DataProperty") => id(v,m),
         Some("AnnotationProperty") => id(v,m),
         Some("NamedIndividual") => id(v,m), 


         Some("ObjectSomeValuesFrom") => id(v,m),
         Some("ObjectAllValuesFrom") => id(v,m),
         Some("ObjectHasValue") =>  id(v,m),
         Some("ObjectMinCardinality") =>  id(v,m),
         Some("ObjectMinQualifiedCardinality") => id(v,m), 
         Some("ObjectMaxCardinality") =>  id(v,m),
         Some("ObjectMaxQualifiedCardinality") => id(v,m), 
         Some("ObjectExactCardinality") =>  id(v,m),
         Some("ObjectExactQualifiedCardinality") => id(v,m), 
         Some("ObjectHasSelf") => id(v,m), 
         Some("ObjectIntersectionOf") => id(v,m), 
         Some("ObjectUnionOf") => id(v,m), 
         Some("ObjectOneOf") => id(v,m), 
         Some("ObjectComplementOf") => id(v,m), 

         Some("ObjectInverseOf") => id(v,m),  //there is no data inverse
         Some(_) => panic!(),  //there is no data inverse
         None => Value::String(String::from(v.as_str().unwrap())),
     }
} 

pub fn is_named_individual(v :&Value, m : &HashMap<String, HashSet<String>>) -> bool {

    //TODO: test this
    let s = v.as_str().unwrap();

    match m.get(s) {
        Some(set) => set.contains("owl:NamedIndividual"),
        _ => false,
    }

}

pub fn is_class_expression(v : &Value, m : &HashMap<String, HashSet<String>>) -> bool {

     match v[0].as_str() {
         //abstract
         Some("SomeValuesFrom") => true,
         Some("AllValuesFrom") => true,
         Some("HasValue") => true,
         Some("MinCardinality") => true,
         Some("MaxCardinality") => true,
         Some("ExactCardinality") => true,

         //data - note that data restrictions are still class expressions
         Some("DataSomeValuesFrom") => true,
         Some("DataAllValuesFrom") => true,
         Some("DataHasValue") => true,
         Some("DataMinCardinality") => true,
         Some("DataMaxCardinality") => true,
         Some("DataExactCardinality") => true,

         Some("ObjectSomeValuesFrom") => true,
         Some("ObjectAllValuesFrom") => true,
         Some("ObjectHasValue") =>  true,
         Some("ObjectMinCardinality") =>  true,
         Some("ObjectMinQualifiedCardinality") => true,
         Some("ObjectMaxCardinality") =>  true,
         Some("ObjectMaxQualifiedCardinality") => true,
         Some("ObjectExactCardinality") =>  true,
         Some("ObjectExactQualifiedCardinality") => true, 
         Some("ObjectHasSelf") => true,

         //object (note that DataIntersections, etc. are NOT class expressions)
         Some("ObjectIntersectionOf") => true,
         Some("ObjectUnionOf") => true,
         Some("ObjectOneOf") => true,
         Some("ObjectComplementOf") => true,
         Some(_) => panic!(),  //there is no data inverse
         None => type_look_up(v.as_str().unwrap(), m),
     }
}

//TODO: refactor this into a data type translation 
//pub fn is_data_type_expression(v : &Value, m : &HashMap<String, HashSet<String>>) -> bool {
//
//}

pub fn type_look_up(s : &str, m: &HashMap<String, HashSet<String>>) -> bool { 

    match m.get(s) {
        Some(set) => set.contains("owl:Class"),
        _ => false,
    }
}

pub fn id(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value { 

    let mut res = Vec::new();
    let operator = Value::String(String::from(v[0].as_str().unwrap()));
    res.push(operator);

    let arguments  = &(v.as_array().unwrap())[1..]; 
    for argument in arguments  {
        let test : Value = translate(argument, m);
        res.push(test);
    } 
    Value::Array(res)
}

pub fn translate_rdf_type(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    match v[3].as_str() {
        //declarations
        Some("owl:Class") => translate_declaration(v,m),
        Some("rdfs:Datatype") => translate_declaration(v,m), 
        Some("owl:ObjectProperty") => translate_declaration(v,m),
        Some("owl:DatatypeProperty") => translate_declaration(v,m), 
        Some("owl:AnnotationProperty") => translate_declaration(v,m),
        Some("owl:NamedIndividual") => translate_declaration(v,m),

        //property axioms
        Some("owl:FunctionalProperty") => translate_functional_property(v,m),
        Some("owl:InverseFunctionalProperty") => translate_inverse_functional_property(v,m),
        Some("owl:ReflexiveProperty") => translate_reflexive_property(v,m),
        Some("owl:IrreflexiveProperty") => translate_irreflexive_property(v,m),
        Some("owl:SymmetricProperty") => translate_symmetric_property(v,m),
        Some("owl:AsymmetricProperty") => translate_asymmetric_property(v,m),
        Some("owl:TransitiveProperty") => translate_transitive_property(v,m),

        _ => translate_class_assertion(v,m),

        //_ => panic!("Unknown type in declaration") //Could be a class assertion
    } 
}

pub fn translate_class_assertion(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let i = v[1].as_str().unwrap();
    let c = v[2].as_str().unwrap();
    let individual = Value::String(String::from(i));
    let class = Value::String(String::from(c));

    if is_named_individual(&v[1], m) || is_class_expression(&v[2],m) { 
        let operator = Value::String(String::from("ClassAssertion"));
        let v = vec![operator, class, individual];
        Value::Array(v)
    } else {
        panic!("Not a class assertion")
    } 
}

pub fn translate_transitive_property(v : &Value, _m : &HashMap<String, HashSet<String>>) -> Value {
    //only object properties can be "transitive"

    let s = v[1].as_str().unwrap();
    let entity = Value::String(String::from(s));
    let operator = Value::String(String::from("TransitiveObjectProperty")); 

    let axiom = vec![operator,entity];
    Value::Array(axiom) 
}

pub fn translate_asymmetric_property(v : &Value, _m : &HashMap<String, HashSet<String>>) -> Value {
    //only object properties can be "asymmetric"

    let s = v[1].as_str().unwrap();
    let entity = Value::String(String::from(s));
    let operator = Value::String(String::from("AsymmetricObjectProperty")); 

    let axiom = vec![operator,entity];
    Value::Array(axiom) 
}

pub fn translate_symmetric_property(v : &Value, _m : &HashMap<String, HashSet<String>>) -> Value {
    //only object properties can be "symmetric"

    let s = v[1].as_str().unwrap();
    let entity = Value::String(String::from(s));
    let operator = Value::String(String::from("SymmetricObjectProperty")); 

    let axiom = vec![operator,entity];
    Value::Array(axiom) 
}

pub fn translate_irreflexive_property(v : &Value, _m : &HashMap<String, HashSet<String>>) -> Value {
    //only object properties can be "irreflexive"

    let s = v[1].as_str().unwrap();
    let entity = Value::String(String::from(s));
    let operator = Value::String(String::from("IrreflexiveObjectProperty")); 

    let axiom = vec![operator,entity];
    Value::Array(axiom) 
}

pub fn translate_reflexive_property(v : &Value, _m : &HashMap<String, HashSet<String>>) -> Value {
    //only object properties can be "reflexive"

    let s = v[1].as_str().unwrap();
    let entity = Value::String(String::from(s));
    let operator = Value::String(String::from("ReflexiveObjectProperty")); 

    let axiom = vec![operator,entity];
    Value::Array(axiom) 
}

pub fn translate_functional_property(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let s = v[1].as_str().unwrap();
    let entity = Value::String(String::from(s));

    let operator =
    if property_translation::is_object_property(&v[1],m) { 
        Value::String(String::from("FunctionalObjectProperty"))
    } else if property_translation::is_data_property(&v[1],m) {
        Value::String(String::from("FunctionalDataProperty"))
    } else { 
        panic!("Unknown functional property axiom")
        //Value::String(String::from("UnknwonFunctionalProperty"))
    };

    let axiom = vec![operator,entity];
    Value::Array(axiom)
}

pub fn translate_inverse_functional_property(v : &Value, _m : &HashMap<String, HashSet<String>>) -> Value {
    //only object properties can be "inverse functional"

    let s = v[1].as_str().unwrap();
    let entity = Value::String(String::from(s));
    let operator = Value::String(String::from("InverseFunctionalObjectProperty")); 

    let axiom = vec![operator,entity];
    Value::Array(axiom)
}

pub fn translate_declaration(v : &Value, _m : &HashMap<String, HashSet<String>>) -> Value {

    let s = v[1].as_str().unwrap();
    let entity = Value::String(String::from(s));
    let operator = 
    match v[3].as_str() {
        Some("owl:Class") => Value::String(String::from("Class")),
        Some("rdfs:Datatype") => Value::String(String::from("Datatype")), 
        Some("owl:ObjectProperty") => Value::String(String::from("ObjectProperty")),
        Some("owl:DatatypeProperty") => Value::String(String::from("DataProperty")), 
        Some("owl:AnnotationProperty") => Value::String(String::from("AnnotationProperty")),
        Some("owl:NamedIndividual") => Value::String(String::from("NamedIndividual")),
        _ => panic!("Unknown type in declaration")
    };

    let v = vec![operator, entity];
    let v = Value::Array(v);

    let declaration_operator = Value::String(String::from("Declaration"));

    let declaration = vec![declaration_operator,v];

    Value::Array(declaration)
}

pub fn translate_annotation_assertion(v : &Value, _m : &HashMap<String, HashSet<String>>) -> Value {
    let s = v[1].as_str().unwrap();
    let p = v[2].as_str().unwrap();
    let o = v[3].as_str().unwrap();

    let subject = Value::String(String::from(s));
    let predicate = Value::String(String::from(p));
    let object = Value::String(String::from(o));

    let operator = Value::String(String::from("AnnotationAssertion"));
    let v = vec![operator, predicate, subject, object];
    Value::Array(v) 

}

pub fn translate_some_values_from(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let property: Value = translate(&v[1],m); 
    let filler: Value = translate(&v[2],m); 

    //TODO: check data type
    if is_class_expression(&v[2], m) || property_translation::is_object_property(&v[1],m) { 
        let operator = Value::String(String::from("ObjectSomeValuesFrom"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    } else if property_translation::is_data_property(&v[1],m) {
        let operator = Value::String(String::from("DataSomeValuesFrom"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    } else { 
        let operator = Value::String(String::from("ErrorSomeValuesFrom"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    }
} 

pub fn translate_all_values_from(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let property: Value = translate(&v[1],m); 
    let filler: Value = translate(&v[2],m); 

    //TODO: check data type
    if is_class_expression(&v[2], m) || property_translation::is_object_property(&v[1],m) {
        let operator = Value::String(String::from("ObjectAllValuesFrom"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    } else if property_translation::is_data_property(&v[1],m) {
        let operator = Value::String(String::from("DataAllValuesFrom"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    } else { 
        let operator = Value::String(String::from("ErrorAllValuesFrom"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    }
} 

pub fn translate_has_value(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let property: Value = translate(&v[1],m); 
    let filler: Value = translate(&v[2],m); 

    //TODO: the filler would be an individual..
    if is_class_expression(&v[2], m) || property_translation::is_object_property(&v[1],m) {
        let operator = Value::String(String::from("ObjectHasValue"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    } else if property_translation::is_data_property(&v[1],m) {
        let operator = Value::String(String::from("DataHasValue"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    } else {
        let operator = Value::String(String::from("ErrorHasValue"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    }
} 

pub fn translate_min_cardinality(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let property: Value = translate(&v[1],m); 
    let cardinality: Value = translate(&v[2],m); 

    if property_translation::is_object_property(&v[1],m) { 
        let operator = Value::String(String::from("ObjectMinCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    } else if property_translation::is_data_property(&v[1],m) {
        let operator = Value::String(String::from("DataMinCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    } else { 
        let operator = Value::String(String::from("ErrorMinCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    } 
} 

pub fn translate_max_cardinality(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let property: Value = translate(&v[1],m); 
    let cardinality: Value = translate(&v[2],m); 

    if property_translation::is_object_property(&v[1],m) {
        let operator = Value::String(String::from("ObjectMaxCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    } else if property_translation::is_data_property(&v[1],m) {
        let operator = Value::String(String::from("DataMaxCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    } else {
        let operator = Value::String(String::from("ErrorMaxCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    } 
} 

pub fn translate_exact_cardinality(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let property: Value = translate(&v[1],m); 
    let cardinality: Value = translate(&v[2],m); 

    if property_translation::is_object_property(&v[1],m) {
        let operator = Value::String(String::from("ObjectExactCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    } else if property_translation::is_data_property(&v[1],m) {
        let operator = Value::String(String::from("DataExactCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    } else {
        let operator = Value::String(String::from("ErrorExactCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    }
}

pub fn translate_list(v : &[Value], m : &HashMap<String, HashSet<String>>) -> Value {

    let mut res = Vec::new();
    for argument in v {
        let t: Value = translate(&argument,m); 
        res.push(t) 
    }
    Value::Array(res) 
}

//TODO: check arguments for propert type
pub fn translate_intersection_of(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let mut res = Vec::new();
    let operator = Value::String(String::from("ObjectIntersectionOf"));
    let mut operands : Value = translate_list(&(v.as_array().unwrap())[1..],m);
    let r = operands.as_array_mut().unwrap();
    res.push(operator);
    res.append(r); 
    Value::Array(res) 

} 

pub fn translate_union_of(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let mut res = Vec::new();
    let operator = Value::String(String::from("ObjectUnionOf"));
    let mut operands : Value = translate_list(&(v.as_array().unwrap())[1..],m);
    let r = operands.as_array_mut().unwrap();
    res.push(operator);
    res.append(r); 
    Value::Array(res)
} 

pub fn translate_one_of(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let mut res = Vec::new();
    let operator = Value::String(String::from("ObjectOneOf"));
    let mut operands : Value = translate_list(&(v.as_array().unwrap())[1..],m);
    let r = operands.as_array_mut().unwrap();
    res.push(operator);
    res.append(r); 
    Value::Array(res) 
} 

pub fn translate_complement_of(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let argument: Value = translate(&v[1],m); 
    let operator = Value::String(String::from("ObjectComplementOf"));
    let v = vec![operator, argument];
    Value::Array(v) 
} 
