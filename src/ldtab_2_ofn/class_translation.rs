use crate::owl::thick_triple as owl;
use crate::ldtab_2_ofn::property_translation as property_translation;
use serde_json::{Value};

/// Given an OWL expressions owl
/// return its corresponding OFN S-expression.
pub fn translate(owl: &owl::OWL) -> Value {
     match &*owl {
         owl::OWL::Named(x) => translate_named(x.to_string()),

        //restrictions
        owl::OWL::SomeValuesFrom(x) => translate_some_values_from(x),
        owl::OWL::AllValuesFrom(x) => translate_all_values_from(x),
        owl::OWL::HasValue(x) => translate_has_value(x),
        owl::OWL::HasSelf(x) => translate_has_self(x),
        owl::OWL::MinCardinality(x) => translate_min_cardinality(x),
        owl::OWL::ExactCardinality(x) => translate_exact_cardinality(x),
        owl::OWL::MaxCardinality(x) => translate_max_cardinality(x),

        owl::OWL::MinObjectQualifiedCardinality(x) => translate_object_min_qualified_cardinality(x),
        owl::OWL::MaxObjectQualifiedCardinality(x) => translate_object_max_qualified_cardinality(x),
        owl::OWL::ExactObjectQualifiedCardinality(x) => translate_object_exact_qualified_cardinality(x),

        owl::OWL::MinDataQualifiedCardinality(x) => translate_data_min_qualified_cardinality(x),
        owl::OWL::MaxDataQualifiedCardinality(x) => translate_data_max_qualified_cardinality(x),
        owl::OWL::ExactDataQualifiedCardinality(x) => translate_data_exact_qualified_cardinality(x),

        //propositional connectives
        owl::OWL::IntersectionOf(x) => translate_intersection_of(x),
        owl::OWL::UnionOf(x) => translate_union_of(x),
        owl::OWL::OneOf(x) => translate_one_of(x),
        owl::OWL::ComplementOf(x) => translate_complement_of(x),

        //RDF
        owl::OWL::RDFList(x) => translate_list(x),
        owl::OWL::Members(x) => translate_members(x),
        owl::OWL::DistinctMembers(x) => translate_distinct_members(x),

        //object properties
        owl::OWL::InverseOf(x) => property_translation::translate_inverse_of(x),

        owl::OWL::NegativeObjectPropertyAssertion(x) => translate_negative_object_property_assertion(x),
        owl::OWL::NegativeDataPropertyAssertion(x) => translate_negative_data_property_assertion(x) 
    }
}

/// Given an OFN S-expression operator (as a string),
/// return true if the operator encodes a class expression.
pub fn is_class_expression(s: &str) -> bool {
     match s {
         "SomeValuesFrom" => true,
         "ObjectSomeValuesFrom" => true,
         "AllValuesFrom" => true,
         "ObjectAllValuesFrom" => true,
         "ObjectHasValue" => true,
         "ObjectHasSelf" => true,
         "HasSelf" => true,
         "HasValue" => true,
         "ObjectMinCardinality" => true,
         "ObjectMinQualifiedCardinality" => true,
         "MinCardinality" => true,
         "MinQualifiedCardinality" => true,
         "ObjectMaxCardinality" => true,
         "ObjectMaxQualifiedCardinality" => true,
         "MaxCardinality" => true,
         "MaxQualifiedCardinality" => true,
         "ObjectExactCardinality" => true,
         "ObjectExactQualifiedCardinality" => true,
         "ExactCardinality" => true,
         "ExactQualifiedCardinality" => true,
         "ObjectOneOf" => true,
         "ObjectComplementOf" => true,
         "ObjectUnionOf" => true,
         "UnionOf" => true,
         "ObjectIntersectionOf" => true,
         "IntersectionOf" => true,
         "ObjectInverseOf" => true,
         "InverseOf" => true,
         _ => false,
     } 
}

/// Given a string, return a JSON string
pub fn translate_named(s: String) -> Value {
    Value::String(s)
}

/// Given an existential restriction,
/// return its corresponding OFN S-expression.
pub fn translate_some_values_from(exp: &owl::SomeValuesFrom) -> Value { 
    let property = translate(&exp.owl_on_property[0].object);
    let filler =  translate(&exp.owl_some_values_from[0].object);

    let operator = Value::String(String::from("SomeValuesFrom"));
    let v = vec![operator, property, filler];
    Value::Array(v) 
}

/// Given a universal restrictions,
/// return its corresponding OFN S-expression.
pub fn translate_all_values_from(exp: &owl::AllValuesFrom) -> Value { 
    let property = translate(&exp.owl_on_property[0].object);
    let filler =  translate(&exp.owl_all_values_from[0].object);

    let operator = Value::String(String::from("AllValuesFrom"));
    let v = vec![operator, property, filler];
    Value::Array(v)
}

/// Given a HasValue restriction,
/// return its corresponding OFN S-expression.
pub fn translate_has_value(exp: &owl::HasValue) -> Value { 
    let property = translate(&exp.owl_on_property[0].object);
    let filler =  translate(&exp.owl_has_value[0].object);

    let operator = Value::String(String::from("HasValue"));
    let v = vec![operator, property, filler];
    Value::Array(v)
}

/// Given a HasSelf restriction,
/// return its corresponding OFN S-expression.
pub fn translate_has_self(exp: &owl::HasSelf) -> Value { 
    let property = translate(&exp.owl_on_property[0].object); 
    let operator = Value::String(String::from("ObjectHasSelf"));
    let v = vec![operator, property];
    Value::Array(v)

    //ignoring "owl_has_self" because that only contains "true^^xsd:boolean"
    //expression
}

/// Given a MinCardinality restriction,
/// return its corresponding OFN S-expression.
pub fn translate_min_cardinality(exp: &owl::MinCardinality) -> Value { 
    let property = translate(&exp.owl_on_property[0].object);
    let cardinality =  translate(&exp.owl_min_cardinality[0].object);

    let operator = Value::String(String::from("MinCardinality"));
    let v = vec![operator, cardinality, property];
    Value::Array(v)
}

/// Given a qualified ObjectMinCardinality restriction,
/// return its corresponding OFN S-expression.
pub fn translate_object_min_qualified_cardinality(exp: &owl::MinObjectQualifiedCardinality) -> Value { 
    let property = translate(&exp.owl_on_property[0].object);
    let cardinality =  translate(&exp.owl_min_qualified_cardinality[0].object);
    let filler =  translate(&exp.owl_on_class[0].object);//this reveals the type

    let operator = Value::String(String::from("ObjectMinCardinality"));
    let v = vec![operator, cardinality, property, filler];
    Value::Array(v)
}

/// Given a DataMinCardinality restriction,
/// return its corresponding OFN S-expression.
pub fn translate_data_min_qualified_cardinality(exp: &owl::MinDataQualifiedCardinality) -> Value { 
    let property = translate(&exp.owl_on_property[0].object);
    let cardinality =  translate(&exp.owl_min_qualified_cardinality[0].object);
    let filler =  translate(&exp.owl_on_datarange[0].object);//this reveals the type

    let operator = Value::String(String::from("DataMinCardinality"));
    let v = vec![operator, cardinality, property, filler];
    Value::Array(v)
}

/// Given a MaxCardinality restriction,
/// return its corresponding OFN S-expression.
pub fn translate_max_cardinality(exp: &owl::MaxCardinality) -> Value { 
    let property = translate(&exp.owl_on_property[0].object);
    let cardinality =  translate(&exp.owl_max_cardinality[0].object);

    let operator = Value::String(String::from("MaxCardinality"));
    let v = vec![operator, cardinality, property];
    Value::Array(v)
}

/// Given a qualified ObjectMaxCardinality restriction,
/// return its corresponding OFN S-expression.
pub fn translate_object_max_qualified_cardinality(exp: &owl::MaxObjectQualifiedCardinality) -> Value { 
    let property = translate(&exp.owl_on_property[0].object);
    let cardinality =  translate(&exp.owl_max_qualified_cardinality[0].object);
    let filler =  translate(&exp.owl_on_class[0].object);//this reveals the type

    let operator = Value::String(String::from("ObjectMaxCardinality"));
    let v = vec![operator, cardinality, property, filler];
    Value::Array(v)
}

/// Given a qualified DataMaxCardinality restriction,
/// return its corresponding OFN S-expression.
pub fn translate_data_max_qualified_cardinality(exp: &owl::MaxDataQualifiedCardinality) -> Value { 
    let property = translate(&exp.owl_on_property[0].object);
    let cardinality =  translate(&exp.owl_max_qualified_cardinality[0].object);
    let filler =  translate(&exp.owl_on_datarange[0].object);//this reveals the type

    let operator = Value::String(String::from("DataMaxCardinality"));
    let v = vec![operator, cardinality, property, filler];
    Value::Array(v)
}

/// Given an ExactCardinality restriction,
/// return its corresponding OFN S-expression.
pub fn translate_exact_cardinality(exp: &owl::ExactCardinality) -> Value { 
    let property = translate(&exp.owl_on_property[0].object);
    let cardinality =  translate(&exp.owl_cardinality[0].object);

    let operator = Value::String(String::from("ExactCardinality"));
    let v = vec![operator, cardinality, property];
    Value::Array(v)
}

/// Given an qualified ObjectExactCardinality restriction,
/// return its corresponding OFN S-expression.
pub fn translate_object_exact_qualified_cardinality(exp: &owl::ExactObjectQualifiedCardinality) -> Value { 
    let property = translate(&exp.owl_on_property[0].object);
    let cardinality =  translate(&exp.owl_qualified_cardinality[0].object);
    let filler =  translate(&exp.owl_on_class[0].object);

    let operator = Value::String(String::from("ObjectExactCardinality"));
    let v = vec![operator, cardinality, property, filler];
    Value::Array(v)
} 

/// Given an qualified DataExactCardinality restriction,
/// return its corresponding OFN S-expression.
pub fn translate_data_exact_qualified_cardinality(exp: &owl::ExactDataQualifiedCardinality) -> Value { 
    let property = translate(&exp.owl_on_property[0].object);
    let cardinality =  translate(&exp.owl_qualified_cardinality[0].object);
    let filler =  translate(&exp.owl_on_datarange[0].object);

    let operator = Value::String(String::from("DataExactCardinality"));
    let v = vec![operator, cardinality, property, filler];
    Value::Array(v)
} 

/// Given a member list,
/// return its corresponding list of OFN S-expressions.
pub fn translate_members(exp: &owl::Members) -> Value { 
    translate(&exp.members[0].object) 
}


/// Given a list of distinct members,
/// return its corresponding OFN S-expressions.
pub fn translate_distinct_members(exp: &owl::DistinctMembers) -> Value { 
    //extract opertor
    let rdf_type = match &exp.rdf_type {
        Some(x) => match &x[0].object {
            owl::OWL::Named(t) => String::from(t),  //match on type
            _ => String::from("Error"), 
        }
        None => String::from("Error"),
    }; 

    let operator =  match rdf_type.as_str() {
        "owl:AllDifferent" => Value::String(String::from("DifferentIndividuals")),
        _ => Value::String(String::from("Error")),
    };

    let mut members = translate(&exp.distinct_members[0].object);

    let mut expression = vec![operator];
    let arguments = members.as_array_mut().unwrap();
    expression.append(arguments);
    Value::Array(expression.to_vec()) 

}

/// Given an RDF list, 
/// return its corresponding OFN S-expressions.
pub fn translate_list(exp: &owl::RDFList) -> Value { 
    //translate RDF list recursively
    let first = translate(&exp.rdf_first[0].object);
    let mut rest =  translate(&exp.rdf_rest[0].object);

    //base case for RDF lists
    if rest.is_string() && rest.as_str().unwrap() == "rdf:nil"  {
        let mut v = Vec::new();
        v.push(first); 
        Value::Array(v) 
    } else {
        //lists (serde arrays) are build up recursively.
        //So, if first = f and rest = [r1, r2], then
        //we want to return [f, r1, r2] instead of [f, [r1, r2]]
        let mut v = Vec::new();
        v.push(first); 
        let r = rest.as_array_mut().unwrap();
        v.append(r); 
        Value::Array(v) 
    }
}

/// Given an owl::Object,
/// return true, if its rdf:type is owl:Class
pub fn check_class_type(v : &Option<Vec<owl::Object>>) -> bool { 
    let mut res : bool = false;
     match v {
        Some(types) => {//check if there is type information
                            for t in types.iter() {//check all types
                                match &t.object {//look for an owl:Class
                                    owl::OWL::Named(s) => if s == "owl:Class" { res = true },
                                    _ => (),

                                }
                            }
                        },
        None => (),
    }
    return res;
}

/// Given an owl::Object,
/// return true, if its rdf:type is rdfs:Datatype
pub fn check_data_range_type(v : &Option<Vec<owl::Object>>) -> bool { 
    let mut res : bool = false;
     match v {
        Some(types) => {//check if there is type information
                            for t in types.iter() {//check all types
                                match &t.object {
                                    owl::OWL::Named(s) => if s == "rdfs:Datatype" { res = true },
                                    _ => (),

                                }
                            }
                        },
        None => (),
    }
    return res;
}


/// Given an owl:intersection,
/// return its corresponding OFN S-expression
pub fn translate_intersection_of(exp: &owl::IntersectionOf) -> Value { 
    let mut intersection_of = translate(&exp.owl_intersection_of[0].object);

    let is_class = check_class_type(&exp.rdf_type);
    let is_data_range = check_data_range_type(&exp.rdf_type);

    let operator =
    if is_class { 
        Value::String(String::from("ObjectIntersectionOf"))
    } else if is_data_range {
        Value::String(String::from("DataIntersectionOf"))
    } else { 
        Value::String(String::from("IntersectionOf"))
    };

    let mut intersection = vec![operator];
    let arguments = intersection_of.as_array_mut().unwrap();
    intersection.append(arguments);
    Value::Array(intersection.to_vec()) 
}

/// Given an owl:union,
/// return its corresponding OFN S-expression
pub fn translate_union_of(exp: &owl::UnionOf) -> Value { 
    let mut union_of = translate(&exp.owl_union_of[0].object);

    let is_class = check_class_type(&exp.rdf_type);
    let is_data_range = check_data_range_type(&exp.rdf_type);

    let operator =
    if is_class { 
        Value::String(String::from("ObjectUnionOf"))
    } else if is_data_range {
        Value::String(String::from("DataUnionOf"))
    }  else  {
        Value::String(String::from("UnionOf"))
    }; 

    let mut union = vec![operator];
    let arguments = union_of.as_array_mut().unwrap();
    union.append(arguments);
    Value::Array(union.to_vec())
}

/// Given an owl:oneOf,
/// return its corresponding OFN S-expression
pub fn translate_one_of(exp: &owl::OneOf) -> Value { 
    let mut one_of = translate(&exp.owl_one_of[0].object);

    let is_class = check_class_type(&exp.rdf_type);
    let is_data_range = check_data_range_type(&exp.rdf_type);

    let operator =
    if is_class { 
        Value::String(String::from("ObjectOneOf"))
    } else if is_data_range {
        Value::String(String::from("DataOneOf"))
    } else {
        Value::String(String::from("OneOf"))
    };

    let mut one = vec![operator];
    let arguments = one_of.as_array_mut().unwrap();
    one.append(arguments);
    Value::Array(one.to_vec()) 
}

/// Given an owl:complementOf,
/// return its corresponding OFN S-expression
pub fn translate_complement_of(exp: &owl::ComplementOf) -> Value { 
    let complement_of = translate(&exp.owl_complement_of[0].object);

    let is_class = check_class_type(&exp.rdf_type);
    let is_data_range = check_data_range_type(&exp.rdf_type);

    let operator = 
    if is_class { 
        Value::String(String::from("ObjectComplementOf"))
    } else if is_data_range {
        Value::String(String::from("DataComplementOf"))
    } else { 
        Value::String(String::from("ComplementOf"))
    };
    let v = vec![operator, complement_of];
    Value::Array(v)
} 

/// Given an owl:NegativePropertyAssertion,
/// return its corresponding OFN S-expression
pub fn translate_negative_object_property_assertion(exp: &owl::NegativeObjectPropertyAssertion) -> Value { 
    let source = translate(&exp.source_individual[0].object);
    let property =  translate(&exp.assertion_property[0].object);
    let target = translate(&exp.target_individual[0].object);

    let operator = Value::String(String::from("NegativeObjectPropertyAssertion"));
    let v = vec![operator, source, property, target];
    Value::Array(v) 
}

/// Given an owl:NegativePropertyAssertion,
/// return its corresponding OFN S-expression
pub fn translate_negative_data_property_assertion(exp: &owl::NegativeDataPropertyAssertion) -> Value { 
    let source = translate(&exp.source_individual[0].object);
    let property =  translate(&exp.assertion_property[0].object);
    let target = translate(&exp.target_value[0].object);

    let operator = Value::String(String::from("NegativeDataPropertyAssertion"));
    let v = vec![operator, source, property, target];
    Value::Array(v) 
}
