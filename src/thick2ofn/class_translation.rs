use crate::owl::typing as owl;
use crate::thick2ofn::property_translation as property_translation;

pub fn translate(b: &owl::OWL) -> String {
     match &*b {//TODO: don't quite understand why &* is necessary here
         owl::OWL::Named(x) => translate_named(x.to_string()),

        //restrictions
        owl::OWL::SomeValuesFrom(x) => translate_some_values_from(x),
        owl::OWL::AllValuesFrom(x) => translate_all_values_from(x),
        owl::OWL::HasValue(x) => translate_has_value(x),
        owl::OWL::HasSelf(x) => translate_has_self(x),
        owl::OWL::MinCardinality(x) => translate_min_cardinality(x),
        owl::OWL::MinQualifiedCardinality(x) => translate_min_qualified_cardinality(x),
        owl::OWL::MaxCardinality(x) => translate_max_cardinality(x),
        owl::OWL::MaxQualifiedCardinality(x) => translate_max_qualified_cardinality(x),
        owl::OWL::ExactCardinality(x) => translate_exact_cardinality(x),
        owl::OWL::ExactQualifiedCardinality(x) => translate_exact_qualified_cardinality(x),

        //propositional connectives
        owl::OWL::IntersectionOf(x) => translate_intersection_of(x),
        owl::OWL::UnionOf(x) => translate_union_of(x),
        owl::OWL::OneOf(x) => translate_one_of(x),
        owl::OWL::ComplementOf(x) => translate_complement_of(x),
        owl::OWL::RDFList(x) => translate_list(x),

        //object properties
        owl::OWL::InverseOf(x) => property_translation::translate_inverse_of(x),
    }
}

pub fn translate_named(s: String) -> String {
    let expression = format!("\"{}\"", s);
        expression
}

//Note that a SomeValuesFrom expression
//can be either an ObjectSomeValuesFrom or a DataSomeValuesFrom
pub fn translate_some_values_from(s: &owl::SomeValuesFrom) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let filler =  translate(&s.owl_some_values_from[0].object);
    //let expression = format!("[\"ObjectSomeValuesFrom\",{},{}]", property, filler);
    let expression = format!("[\"SomeValuesFrom\",{},{}]", property, filler);
    expression
}

//Note that a AllValuesFrom expression
//can be either an ObjectAllValuesFrom or a DataAllValuesFrom
pub fn translate_all_values_from(s: &owl::AllValuesFrom) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let filler =  translate(&s.owl_all_values_from[0].object);
    //let expression = format!("[\"ObjectAllValuesFrom\",{},{}]", property, filler);
    let expression = format!("[\"AllValuesFrom\",{},{}]", property, filler);
    expression
}

//Note that a HasValue expression
//can be either an ObjectHasValue or a DataHasValue
pub fn translate_has_value(s: &owl::HasValue) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let filler =  translate(&s.owl_has_value[0].object);
    //let expression = format!("[\"ObjectHasValue\",{},{}]", property, filler);
    let expression = format!("[\"HasValue\",{},{}]", property, filler);
    expression
}

pub fn translate_has_self(s: &owl::HasSelf) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let expression = format!("[\"ObjectHasSelf\",{}]", property);
    //ignoring "owl_has_self" because that only contains "true^^xsd:boolean"
    expression
}

//Note that a MinCardinality expression
//can be either an ObjectMinCardinality or a DataMinCardinality
pub fn translate_min_cardinality(s: &owl::MinCardinality) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let cardinality =  translate(&s.owl_min_cardinality[0].object);
    //let expression = format!("[\"ObjectMinCardinality\",{},{}]", property, cardinality);
    let expression = format!("[\"MinCardinality\",{},{}]", property, cardinality);
    expression
}

pub fn translate_min_qualified_cardinality(s: &owl::MinQualifiedCardinality) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let cardinality =  translate(&s.owl_min_qualified_cardinality[0].object);
    let filler =  translate(&s.owl_on_class[0].object);//this reveals the type
    let expression = format!("[\"ObjectMinQualifiedCardinality\",{},{},{}]", property, cardinality, filler);
    expression
}

//Note that a MaxCardinality expression
//can be either an ObjectMaxCardinality or a DataMaxCardinality
pub fn translate_max_cardinality(s: &owl::MaxCardinality) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let cardinality =  translate(&s.owl_max_cardinality[0].object);
    //let expression = format!("[\"ObjectMaxCardinality\",{},{}]", property, cardinality);
    let expression = format!("[\"MaxCardinality\",{},{}]", property, cardinality);
    expression
}

pub fn translate_max_qualified_cardinality(s: &owl::MaxQualifiedCardinality) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let cardinality =  translate(&s.owl_max_qualified_cardinality[0].object);
    let filler =  translate(&s.owl_on_class[0].object);//this reveals the type
    let expression = format!("[\"ObjectMaxQualifiedCardinality\",{},{},{}]", property, cardinality, filler);
    expression
}

//Note that an ExactCardinality expression
//can be either an ObjectExactCardinality or a DataExactCardinality
pub fn translate_exact_cardinality(s: &owl::ExactCardinality) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let cardinality =  translate(&s.owl_cardinality[0].object);
    //let expression = format!("[\"ObjectExactCardinality\",{},{}]", property, cardinality);
    let expression = format!("[\"ExactCardinality\",{},{}]", property, cardinality);
    expression
}

pub fn translate_exact_qualified_cardinality(s: &owl::ExactQualifiedCardinality) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let cardinality =  translate(&s.owl_qualified_cardinality[0].object);
    let filler =  translate(&s.owl_on_class[0].object);
    let expression = format!("[\"ObjectExactQualifiedCardinality\",{},{},{}]", property, cardinality, filler);
    expression
} 

pub fn translate_list(s: &owl::RDFList) -> String { 
    let first = translate(&s.rdf_first[0].object);
    let rest =  translate(&s.rdf_rest[0].object);

    //match &rest[..] {
    match &*rest {
        "\"rdf:nil\"" => format!("{}", first),
        _ => format!("{},{}", first, rest),
    }
}

//TODO: test type information here? yes
pub fn translate_intersection_of(s: &owl::IntersectionOf) -> String { 
    let intersection_of = translate(&s.owl_intersection_of[0].object);
    let expression = format!("[\"ObjectIntersectionOf\",{}]", intersection_of);
    expression
}

//TODO: test type information here?
pub fn translate_union_of(s: &owl::UnionOf) -> String { 
    let union_of = translate(&s.owl_union_of[0].object);
    let expression = format!("[\"ObjectUnionOf\",{}]", union_of);
    expression
}

pub fn translate_one_of(s: &owl::OneOf) -> String { 
    let one_of = translate(&s.owl_one_of[0].object);
    let expression = format!("[\"ObjectOneOf\",{}]", one_of);
    expression
}

pub fn translate_complement_of(s: &owl::ComplementOf) -> String { 
    let complement_of = translate(&s.owl_complement_of[0].object);
    let expression = format!("[\"ObjectComplementOf\",{}]", complement_of);
    expression
} 
