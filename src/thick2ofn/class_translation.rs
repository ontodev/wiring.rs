use crate::owl::typing as owl;

pub fn translate(b: &owl::OWL) -> String {
     match &*b {//TODO: don't quite understand why &* is necessary here
        //OWL::Named(x) => println!("Got 50 {:?}", x),
         owl::OWL::Named(x) => translate_named(x.to_string()),
        //OWL::SomeValuesFrom(y) => println!("Matched, y = {:?}", y),

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
        //_ => println!("Default case, x = {:?}", x), 
        //OWL::InverseOf(x) => property_translation::translate(x),
        owl::OWL::InverseOf(x) => translate_inverse_of(x),
    }
}

pub fn translate_named(s: String) -> String {
    let expression = format!("\"{}\"", s);
        expression
}

pub fn translate_some_values_from(s: &owl::SomeValuesFrom) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let filler =  translate(&s.owl_some_values_from[0].object);
    let expression = format!("[\"ObjectSomeValuesFrom\",{},{}]", property, filler);
    expression
}

pub fn translate_all_values_from(s: &owl::AllValuesFrom) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let filler =  translate(&s.owl_all_values_from[0].object);
    let expression = format!("[\"ObjectAllValuesFrom\",{},{}]", property, filler);
    expression
}

pub fn translate_has_value(s: &owl::HasValue) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let filler =  translate(&s.owl_has_value[0].object);
    let expression = format!("[\"ObjectHasValue\",{},{}]", property, filler);
    expression
}

pub fn translate_has_self(s: &owl::HasSelf) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let expression = format!("[\"ObjectHasSelf\",{}]", property);
    //ignoring "owl_has_self" because that only contains "true^^xsd:boolean"
    expression
}

pub fn translate_min_cardinality(s: &owl::MinCardinality) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let cardinality =  translate(&s.owl_min_cardinality[0].object);
    let expression = format!("[\"ObjectMinCardinality\",{},{}]", property, cardinality);
    expression
}

pub fn translate_min_qualified_cardinality(s: &owl::MinQualifiedCardinality) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let cardinality =  translate(&s.owl_min_qualified_cardinality[0].object);
    let filler =  translate(&s.owl_on_class[0].object);
    let expression = format!("[\"ObjectMinCardinality\",{},{},{}]", property, cardinality, filler);
    expression
}

pub fn translate_max_cardinality(s: &owl::MaxCardinality) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let cardinality =  translate(&s.owl_max_cardinality[0].object);
    let expression = format!("[\"ObjectMaxCardinality\",{},{}]", property, cardinality);
    expression
}

pub fn translate_max_qualified_cardinality(s: &owl::MaxQualifiedCardinality) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let cardinality =  translate(&s.owl_max_qualified_cardinality[0].object);
    let filler =  translate(&s.owl_on_class[0].object);
    let expression = format!("[\"ObjectMaxCardinality\",{},{},{}]", property, cardinality, filler);
    expression
}

pub fn translate_exact_cardinality(s: &owl::ExactCardinality) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let cardinality =  translate(&s.owl_cardinality[0].object);
    let expression = format!("[\"ObjectExactCardinality\",{},{}]", property, cardinality);
    expression
}

pub fn translate_exact_qualified_cardinality(s: &owl::ExactQualifiedCardinality) -> String { 
    let property = translate(&s.owl_on_property[0].object);
    let cardinality =  translate(&s.owl_qualified_cardinality[0].object);
    let filler =  translate(&s.owl_on_class[0].object);
    let expression = format!("[\"ObjectExactCardinality\",{},{},{}]", property, cardinality, filler);
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

pub fn translate_intersection_of(s: &owl::IntersectionOf) -> String { 
    let intersection_of = translate(&s.owl_intersection_of[0].object);
    let expression = format!("[\"ObjectIntersectionOf\",{}]", intersection_of);
    expression
}

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

pub fn translate_inverse_of(s: &owl::InverseOf) -> String { 
    let inverse_of = translate(&s.owl_inverse_of[0].object);
    let expression = format!("[\"ObjectInverseOf\",{}]", inverse_of);
    expression
}
