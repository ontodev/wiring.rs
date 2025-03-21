use crate::owl::thick_triple as owl;
use crate::thick_2_ofn::property_translation;
use serde_json::Value;

pub fn translate(b: &owl::OWL) -> Value {
    match &*b {
        //TODO: don't quite understand why &* is necessary here
        owl::OWL::Named(x) => translate_named(x.to_string()),

        //restrictions
        owl::OWL::SomeValuesFrom(x) => translate_some_values_from(x),
        owl::OWL::AllValuesFrom(x) => translate_all_values_from(x),
        owl::OWL::HasValue(x) => translate_has_value(x),
        owl::OWL::HasSelf(x) => translate_has_self(x),
        owl::OWL::MinCardinality(x) => translate_min_cardinality(x),
        owl::OWL::MinObjectQualifiedCardinality(x) => translate_min_qualified_cardinality(x),
        owl::OWL::MaxCardinality(x) => translate_max_cardinality(x),
        owl::OWL::MaxObjectQualifiedCardinality(x) => translate_max_qualified_cardinality(x),
        owl::OWL::ExactCardinality(x) => translate_exact_cardinality(x),
        owl::OWL::ExactObjectQualifiedCardinality(x) => translate_exact_qualified_cardinality(x),

        //propositional connectives
        owl::OWL::IntersectionOf(x) => translate_intersection_of(x),
        owl::OWL::UnionOf(x) => translate_union_of(x),
        owl::OWL::OneOf(x) => translate_one_of(x),
        owl::OWL::ComplementOf(x) => translate_complement_of(x),
        owl::OWL::RDFList(x) => translate_rdf_list(x),

        owl::OWL::MaxDataQualifiedCardinality(x) => translate_max_data_qualified_cardinality(x),
        owl::OWL::MinDataQualifiedCardinality(x) => translate_min_data_qualified_cardinality(x),
        owl::OWL::ExactDataQualifiedCardinality(x) => translate_exact_data_qualified_cardinality(x),

        //object properties
        owl::OWL::InverseOf(x) => property_translation::translate_inverse_of(x),

        owl::OWL::List(x) => translate_list(x),
        owl::OWL::NegativeObjectPropertyAssertion(x) => translate_negative_object_property_assertion(x),
        owl::OWL::NegativeDataPropertyAssertion(x) => translate_negative_data_property_assertion(x),
        owl::OWL::Members(x) => translate_members(x),
        owl::OWL::DistinctMembers(x) => translate_distinct_members(x),
    }
}

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

pub fn translate_named(s: String) -> Value {
    Value::String(s)
}

//Note that a SomeValuesFrom expression
//can be either an ObjectSomeValuesFrom or a DataSomeValuesFrom
pub fn translate_some_values_from(s: &owl::SomeValuesFrom) -> Value {
    let property = translate(&s.owl_on_property[0].object);
    let filler = translate(&s.owl_some_values_from[0].object);

    let operator = Value::String(String::from("SomeValuesFrom"));
    let v = vec![operator, property, filler];
    Value::Array(v)
}

//Note that a AllValuesFrom expression
//can be either an ObjectAllValuesFrom or a DataAllValuesFrom
pub fn translate_all_values_from(s: &owl::AllValuesFrom) -> Value {
    let property = translate(&s.owl_on_property[0].object);
    let filler = translate(&s.owl_all_values_from[0].object);

    let operator = Value::String(String::from("AllValuesFrom"));
    let v = vec![operator, property, filler];
    Value::Array(v)
}

//Note that a HasValue expression
//can be either an ObjectHasValue or a DataHasValue
pub fn translate_has_value(s: &owl::HasValue) -> Value {
    let property = translate(&s.owl_on_property[0].object);
    let filler = translate(&s.owl_has_value[0].object);

    let operator = Value::String(String::from("HasValue"));
    let v = vec![operator, property, filler];
    Value::Array(v)
}

pub fn translate_has_self(s: &owl::HasSelf) -> Value {
    let property = translate(&s.owl_on_property[0].object);
    let operator = Value::String(String::from("ObjectHasSelf"));
    let v = vec![operator, property];
    Value::Array(v)

    //ignoring "owl_has_self" because that only contains "true^^xsd:boolean"
    //expression
}

//Note that a MinCardinality expression
//can be either an ObjectMinCardinality or a DataMinCardinality
pub fn translate_min_cardinality(s: &owl::MinCardinality) -> Value {
    let property = translate(&s.owl_on_property[0].object);
    let cardinality = translate(&s.owl_min_cardinality[0].object);

    let operator = Value::String(String::from("MinCardinality"));
    let v = vec![operator, property, cardinality];
    Value::Array(v)
}

pub fn translate_min_qualified_cardinality(s: &owl::MinObjectQualifiedCardinality) -> Value {
    let property = translate(&s.owl_on_property[0].object);
    let cardinality = translate(&s.owl_min_qualified_cardinality[0].object);
    let filler = translate(&s.owl_on_class[0].object); //this reveals the type

    let operator = Value::String(String::from("ObjectMinQualifiedCardinality"));
    let v = vec![operator, property, cardinality, filler];
    Value::Array(v)
}

pub fn translate_min_data_qualified_cardinality(s: &owl::MinDataQualifiedCardinality) -> Value {
    let property = translate(&s.owl_on_property[0].object);
    let cardinality = translate(&s.owl_min_qualified_cardinality[0].object);
    let filler = translate(&s.owl_on_datarange[0].object); //this reveals the type

    let operator = Value::String(String::from("DataMinQualifiedCardinality"));
    let v = vec![operator, property, cardinality, filler];
    Value::Array(v)
}

//Note that a MaxCardinality expression
//can be either an ObjectMaxCardinality or a DataMaxCardinality
pub fn translate_max_cardinality(s: &owl::MaxCardinality) -> Value {
    let property = translate(&s.owl_on_property[0].object);
    let cardinality = translate(&s.owl_max_cardinality[0].object);

    let operator = Value::String(String::from("ObjectMaxCardinality"));
    let v = vec![operator, property, cardinality];
    Value::Array(v)
}

pub fn translate_max_qualified_cardinality(s: &owl::MaxObjectQualifiedCardinality) -> Value {
    let property = translate(&s.owl_on_property[0].object);
    let cardinality = translate(&s.owl_max_qualified_cardinality[0].object);
    let filler = translate(&s.owl_on_class[0].object); //this reveals the type

    let operator = Value::String(String::from("ObjectMaxQualifiedCardinality"));
    let v = vec![operator, property, cardinality, filler];
    Value::Array(v)
}

pub fn translate_max_data_qualified_cardinality(s: &owl::MaxDataQualifiedCardinality) -> Value {
    let property = translate(&s.owl_on_property[0].object);
    let cardinality = translate(&s.owl_max_qualified_cardinality[0].object);
    let filler = translate(&s.owl_on_datarange[0].object); //this reveals the type

    let operator = Value::String(String::from("DataMaxQualifiedCardinality"));
    let v = vec![operator, property, cardinality, filler];
    Value::Array(v)
}

//Note that an ExactCardinality expression
//can be either an ObjectExactCardinality or a DataExactCardinality
pub fn translate_exact_cardinality(s: &owl::ExactCardinality) -> Value {
    let property = translate(&s.owl_on_property[0].object);
    let cardinality = translate(&s.owl_cardinality[0].object);

    let operator = Value::String(String::from("ExactCardinality"));
    let v = vec![operator, property, cardinality];
    Value::Array(v)
}

pub fn translate_exact_qualified_cardinality(s: &owl::ExactObjectQualifiedCardinality) -> Value {
    let property = translate(&s.owl_on_property[0].object);
    let cardinality = translate(&s.owl_qualified_cardinality[0].object);
    let filler = translate(&s.owl_on_class[0].object);

    let operator = Value::String(String::from("ObjectExactQualifiedCardinality"));
    let v = vec![operator, property, cardinality, filler];
    Value::Array(v)
}

pub fn translate_exact_data_qualified_cardinality(s: &owl::ExactDataQualifiedCardinality) -> Value {
    let property = translate(&s.owl_on_property[0].object);
    let cardinality = translate(&s.owl_qualified_cardinality[0].object);
    let filler = translate(&s.owl_on_datarange[0].object);

    let operator = Value::String(String::from("DataExactQualifiedCardinality"));
    let v = vec![operator, property, cardinality, filler];
    Value::Array(v)
}

pub fn translate_list(s: &Vec<owl::Object>) -> Value {
    let mut list = Vec::new();
    for o in s.iter() {
        list.push(translate(&o.object));
    }
    Value::Array(list)
}

pub fn translate_rdf_list(s: &owl::RDFList) -> Value {
    //translate RDF list recursively
    let first = translate(&s.rdf_first[0].object);
    let mut rest = translate(&s.rdf_rest[0].object);

    //base case for RDF lists
    if rest.is_string() && rest.as_str().unwrap() == "rdf:nil" {
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

pub fn check_class_type(v: &Option<Vec<owl::Object>>) -> bool {
    let mut res: bool = false;
    match v {
        Some(types) => {
            //check if there is type information
            for t in types.iter() {
                //check all types
                match &t.object {
                    //look for an owl:Class
                    owl::OWL::Named(s) => {
                        if s == "owl:Class" {
                            res = true
                        }
                    }
                    _ => (),
                }
            }
        }
        None => (),
    }
    return res;
}

pub fn translate_negative_object_property_assertion(s: &owl::NegativeObjectPropertyAssertion) -> Value {
    let property = translate(&s.assertion_property[0].object);
    let subject = translate(&s.source_individual[0].object);
    let object = translate(&s.target_individual[0].object);

    let operator = Value::String(String::from("NegativeObjectPropertyAssertion"));
    let v = vec![operator, property, subject, object];
    Value::Array(v)
}

pub fn translate_negative_data_property_assertion(s: &owl::NegativeDataPropertyAssertion) -> Value {
    let property = translate(&s.assertion_property[0].object);
    let subject = translate(&s.source_individual[0].object);
    let object = translate(&s.target_value[0].object);

    let operator = Value::String(String::from("NegativeDataPropertyAssertion"));
    let v = vec![operator, property, subject, object];
    Value::Array(v)
}

pub fn translate_members(s: &owl::Members) -> Value {
    let mut members = Vec::new();
    for o in s.members.iter() {
        members.push(translate(&o.object));
    }
    Value::Array(members)
}

pub fn translate_distinct_members(s: &owl::DistinctMembers) -> Value {
    let mut members = Vec::new();
    for o in s.distinct_members.iter() {
        members.push(translate(&o.object));
    }
    Value::Array(members)
}

//TODO: introduce case for data types
pub fn translate_intersection_of(s: &owl::IntersectionOf) -> Value {
    let mut intersection_of = translate(&s.owl_intersection_of[0].object);

    let is_class = check_class_type(&s.rdf_type);
    if is_class {
        let operator = Value::String(String::from("ObjectIntersectionOf"));
        let mut intersection = vec![operator];
        let arguments = intersection_of.as_array_mut().unwrap();
        intersection.append(arguments);
        Value::Array(intersection.to_vec())
    } else {
        let operator = Value::String(String::from("IntersectionOf"));
        let mut intersection = vec![operator];
        let arguments = intersection_of.as_array_mut().unwrap();
        intersection.append(arguments);
        Value::Array(intersection.to_vec())
    }
}

//TODO: test type information here?
pub fn translate_union_of(s: &owl::UnionOf) -> Value {
    let mut union_of = translate(&s.owl_union_of[0].object);

    let is_class = check_class_type(&s.rdf_type);
    if is_class {
        let operator = Value::String(String::from("ObjectUnionOf"));
        let mut union = vec![operator];
        let arguments = union_of.as_array_mut().unwrap();
        union.append(arguments);
        Value::Array(union.to_vec())
    } else {
        let operator = Value::String(String::from("UnionOf"));
        let mut union = vec![operator];
        let arguments = union_of.as_array_mut().unwrap();
        union.append(arguments);
        Value::Array(union.to_vec())
    }
}

pub fn translate_one_of(s: &owl::OneOf) -> Value {
    let mut one_of = translate(&s.owl_one_of[0].object);

    let is_class = check_class_type(&s.rdf_type);
    if is_class {
        let operator = Value::String(String::from("ObjectOneOf"));
        let mut one = vec![operator];
        let arguments = one_of.as_array_mut().unwrap();
        one.append(arguments);
        Value::Array(one.to_vec())
    } else {
        let operator = Value::String(String::from("OneOf"));
        let mut one = vec![operator];
        let arguments = one_of.as_array_mut().unwrap();
        one.append(arguments);
        Value::Array(one.to_vec())
    }
}

pub fn translate_complement_of(s: &owl::ComplementOf) -> Value {
    let complement_of = translate(&s.owl_complement_of[0].object);
    let is_class = check_class_type(&s.rdf_type);
    if is_class {
        let operator = Value::String(String::from("ObjectComplementOf"));
        let v = vec![operator, complement_of];
        Value::Array(v)
    } else {
        let operator = Value::String(String::from("ComplementOf"));
        let v = vec![operator, complement_of];
        Value::Array(v)
    }
}
