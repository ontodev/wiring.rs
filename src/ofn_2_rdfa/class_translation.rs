use crate::constants::*;
use serde_json::{Value};
use serde_json::json; 
use std::collections::HashMap; 

//TODO: need to encode literals differntly from named entities? 

pub fn get_type(ofn: &Value) -> &str {

     match ofn[0].as_str() {
         Some("ObjectSomeValuesFrom") => OWL_RESTRICTION, 
         Some("ObjectAllValuesFrom") => OWL_RESTRICTION,
         Some("ObjectHasValue") => OWL_RESTRICTION, 
         Some("ObjectMinCardinality") => OWL_RESTRICTION, 
         Some("ObjectMinQualifiedCardinality") => OWL_RESTRICTION, 
         Some("ObjectMaxCardinality") => OWL_RESTRICTION, 
         Some("ObjectMaxQualifiedCardinality") => OWL_RESTRICTION, 
         Some("ObjectExactCardinality") => OWL_RESTRICTION, 
         Some("ObjectExactQualifiedCardinality") => OWL_RESTRICTION, 
         Some("ObjectHasSelf") => OWL_RESTRICTION, 
         Some("ObjectIntersectionOf") => OWL_CLASS, 
         Some("ObjectUnionOf") => OWL_CLASS, 
         Some("ObjectOneOf") => OWL_CLASS, 
         Some("ObjectComplementOf") => OWL_CLASS, 
         Some(_) => ofn.as_str().unwrap(),
         None => panic!(),
     }
}

pub fn is_named_class(ofn: &Value) -> bool {

    match ofn[0].as_str() {
         Some("ObjectSomeValuesFrom") => false, 
         Some("ObjectAllValuesFrom") => false,
         Some("ObjectHasValue") => false,
         Some("ObjectMinCardinality") => false,
         Some("ObjectMinQualifiedCardinality") => false,
         Some("ObjectMaxCardinality") => false,
         Some("ObjectMaxQualifiedCardinality") => false,
         Some("ObjectExactCardinality") => false,
         Some("ObjectExactQualifiedCardinality") => false,
         Some("ObjectHasSelf") => false,
         Some("ObjectIntersectionOf") => false,
         Some("ObjectUnionOf") => false,
         Some("ObjectComplementOf") => false,
         //TODO: datatype expressions

         //TODO: handle OneOf expressions properly or rename function to 'needs_parenthesis'
         Some("ObjectOneOf") => true,
         Some(_) => panic!(),
         None => true, 
    } 
}

pub fn translate(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value {

     match ofn[0].as_str() {
         Some("ObjectSomeValuesFrom") => translate_some_values_from(ofn, subject_2_label, rdfa_property), 
         Some("ObjectAllValuesFrom") => translate_all_values_from(ofn, subject_2_label, rdfa_property), 
         Some("ObjectMinCardinality") => translate_min_cardinality(ofn, subject_2_label, rdfa_property), 
         Some("ObjectMinQualifiedCardinality") => translate_min_qualified_cardinality(ofn, subject_2_label, rdfa_property), 
         Some("ObjectMaxCardinality") => translate_max_cardinality(ofn, subject_2_label, rdfa_property), 
         Some("ObjectMaxQualifiedCardinality") => translate_max_qualified_cardinality(ofn, subject_2_label, rdfa_property), 
         Some("ObjectExactCardinality") => translate_exact_cardinality(ofn, subject_2_label, rdfa_property), 
         Some("ObjectExactQualifiedCardinality") => translate_exact_qualified_cardinality(ofn, subject_2_label, rdfa_property), 
         Some("ObjectHasValue") => translate_has_value(ofn, subject_2_label, rdfa_property), 
         Some("ObjectHasSelf") => translate_has_self(ofn, subject_2_label, rdfa_property), 
         Some("ObjectIntersectionOf") => translate_intersection_of(ofn, subject_2_label, rdfa_property), 
         Some("ObjectUnionOf") => translate_union_of(ofn, subject_2_label, rdfa_property), 
         Some("ObjectOneOf") => translate_one_of(ofn, subject_2_label, rdfa_property), 
         Some("ObjectComplementOf") => translate_complement_of(ofn, subject_2_label, rdfa_property), 

         Some("ObjectInverseOf") => translate_inverse_of(ofn, subject_2_label, rdfa_property), 

         Some("DataSomeValuesFrom") => translate_some_values_from(ofn, subject_2_label, rdfa_property), 
         Some("DataAllValuesFrom") => translate_all_values_from(ofn, subject_2_label, rdfa_property), 
         Some("DataMinCardinality") => translate_min_cardinality(ofn, subject_2_label, rdfa_property), 
         Some("DataMinQualifiedCardinality") => translate_min_qualified_cardinality(ofn, subject_2_label, rdfa_property), 
         Some("DataMaxCardinality") => translate_max_cardinality(ofn, subject_2_label, rdfa_property), 
         Some("DataMaxQualifiedCardinality") => translate_max_qualified_cardinality(ofn, subject_2_label, rdfa_property), 
         Some("DataExactCardinality") => translate_exact_cardinality(ofn, subject_2_label, rdfa_property), 
         Some("DataExactQualifiedCardinality") => translate_exact_qualified_cardinality(ofn, subject_2_label, rdfa_property), 
         Some("DataHasValue") => translate_has_value(ofn, subject_2_label, rdfa_property), 
         Some("DataHasSelf") => translate_has_self(ofn, subject_2_label, rdfa_property), 
         Some("DataIntersectionOf") => translate_intersection_of(ofn, subject_2_label, rdfa_property), 
         Some("DataUnionOf") => translate_union_of(ofn, subject_2_label, rdfa_property), 
         Some("DataOneOf") => translate_one_of(ofn, subject_2_label, rdfa_property), 
         Some("DataComplementOf") => translate_complement_of(ofn, subject_2_label, rdfa_property), 

         Some(_) => json!("currently not supported"), //use instead of panicing for convenience
         //Some(_) => panic!(),
         None => base_translation(ofn, subject_2_label, rdfa_property),
     }
}

pub fn base_translation(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value {
    let named_entity = ofn.as_str().unwrap();
    if subject_2_label.contains_key(named_entity) {
        let label = subject_2_label.get(named_entity).unwrap().as_str();
        match rdfa_property {
            Some(p) => json!(["a", {"property":p, "resource":named_entity}, label]), 
            None => json!(["a", {"resource":named_entity}, label]),
        }
    } else {
        match rdfa_property {
            Some(p) => json!(["a", {"property":p,"resource":named_entity}, named_entity]),
            None => json!(["a", {"resource":named_entity}, named_entity]), 
        }
    }
} 

pub fn translate_list(arguments: Vec<Value>, subject_2_label: &HashMap<String,String>, modifier: Value) -> Value{
    //1. reverse array 
    let y: Vec<_> = arguments.into_iter().rev().collect();

    //build first element
    let first = translate(&y[0], subject_2_label, None);
    let mut list;
    if is_named_class(&y[0]) {
        list = json!(["span", {"property":RDF_REST, "typeof":RDF_LIST},modifier,first,["span",{"resource":RDF_NIL, "property":RDF_REST}]] );
    } else {
        list = json!(["span", {"property":RDF_REST, "typeof":RDF_LIST},modifier,"(",first,")",["span",{"resource":RDF_NIL, "property":RDF_REST}]] ); 
    }

    //build middle elements
    for arg in y[1..y.len()-1].iter() {
        let t_arg = translate(&arg, subject_2_label, Some(RDF_FIRST));
        if is_named_class(&arg) {
            list = json!(["span",{"property":RDF_REST,"typeof":RDF_LIST},modifier,t_arg,list]);
        } else { 
            list = json!(["span",{"property":RDF_REST,"typeof":RDF_LIST},modifier,"(",t_arg,")",list]);
        }
    } 

    //build last element
    let last = translate(&y[y.len()-1], subject_2_label, Some(RDF_FIRST));
    if is_named_class(&y[y.len()-1]) {
        list = json!(["span",last,list]);
    } else {
        list = json!(["span","(",last,")",list]); 
    }
    list
}


pub fn translate_intersection_of(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value {

    let operands: Vec<Value> = ofn.as_array().unwrap()[1..].to_vec();
    let modifier = json!(" and ");
    let ops = translate_list(operands, subject_2_label, modifier);

    match rdfa_property {
        Some(p) => json!(["span",{"property":p},["span",{"property":OWL_INTERSECTION_OF,"typeof":RDF_LIST},ops]]),
        None => json!(["span",["span",{"property":OWL_INTERSECTION_OF,"typeof":RDF_LIST},ops]]) 
    }
}

pub fn translate_union_of(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value {

    let operands: Vec<Value> = ofn.as_array().unwrap()[1..].to_vec();
    let modifier = json!(" or ");
    let ops = translate_list(operands, subject_2_label, modifier);

    match rdfa_property {
        Some(p) => json!(["span",{"property":p},["span",{"property":OWL_UNION_OF,"typeof":RDF_LIST},ops]]),
        None => json!(["span",["span",{"property":OWL_UNION_OF,"typeof":RDF_LIST},ops]]) 
    }
}

pub fn translate_one_of(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value {

    let operands: Vec<Value> = ofn.as_array().unwrap()[1..].to_vec();
    let modifier = json!(", ");
    let ops = translate_list(operands, subject_2_label, modifier);

    //TODO: we need to check for OneOf operator when adding parenthesis
    //currently, we translate ObjectOneOf(a,b,c) to ({a,b,c}) instead of {a,b,c}
    match rdfa_property {
        Some(p) => json!(["span",{"property":p},["span",{"property":OWL_ONE_OF,"typeof":RDF_LIST},"{",ops,"}"]]),
        None => json!(["span",["span",{"property":OWL_ONE_OF,"typeof":RDF_LIST},"{",ops,"}"]]) 
    }
}



pub fn render_restriction_base(prop: &Value, modifier: &Value, filler: &Value, rdfa_property: Option<&str>) -> Value {

    match rdfa_property {
        Some(p) => json!(["span", {"property":p, "typeof":OWL_RESTRICTION}, prop, modifier, filler]),
        None => json!(["span",{"typeof":OWL_RESTRICTION}, prop, modifier, filler]),
    }
}

pub fn render_restriction_nested(prop: &Value, modifier: &Value, filler: &Value, rdfa_property: Option<&str>) -> Value {

    match rdfa_property {
        Some(p) => json!(["span", {"property":p, "typeof":OWL_RESTRICTION}, prop, modifier, "(", filler, ")"]),
        None => json!(["span",{"typeof":OWL_RESTRICTION}, prop, modifier, "(", filler, ")"]),
    }
}

pub fn render_qualified_cardinality_restriction_base(prop: &Value,
                                                     modifier: &Value,
                                                     cardinality: &Value,
                                                     filler: &Value,
                                                     rdfa_property: Option<&str>) -> Value {

    match rdfa_property {
        Some(p) => json!(["span", {"property":p, "typeof":OWL_RESTRICTION}, prop, modifier, cardinality, " ", filler]),
        None => json!(["span",{"typeof":OWL_RESTRICTION}, prop, modifier, cardinality, " ", filler]),
    }
}

pub fn render_qualified_cardinality_restriction_nested(prop: &Value,
                                                       modifier: &Value,
                                                       cardinality: &Value,
                                                       filler: &Value,
                                                       rdfa_property: Option<&str>) -> Value {

    match rdfa_property {
        Some(p) => json!(["span", {"property":p, "typeof":OWL_RESTRICTION}, prop, modifier, cardinality, " ", "(", filler, ")"]),
        None => json!(["span",{"typeof":OWL_RESTRICTION}, prop, modifier, cardinality, " ", "(", filler, ")"]),
    }
}

pub fn span_opening(ofn: &Value, property : &str) -> Value {
    if is_named_class(&ofn) {
        if property.eq("") {
            //json!({"about": ofn})
            json!({"resource": ofn})
        }
        else {
            //json!({"about": ofn, "property": property})
            json!({"resource": ofn, "property": property})
        }
    } else {
        if property.eq("") {
            json!({"typeof": get_type(&ofn)}) 
        } 
        else {
            json!({"typeof": get_type(&ofn), "property" : property}) 
        }

    }
} 

pub fn label_substitution(named_class: &str, subject_2_label: &HashMap<String,String>) -> String {

    let element = String::from(named_class);

    if subject_2_label.contains_key(&element) {
        String::from(subject_2_label.get(&element).unwrap().as_str())
    } else {
        element
    }
}

pub fn translate_complement_of(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value { 

    //TODO: use propertytranslation?
    let argument = translate(&ofn[1], subject_2_label, Some(OWL_COMPLEMENT_OF));

    if ofn[1].is_array() { 
        match rdfa_property {
            Some(p) => json!(["span",{"property":p}," not ","(",argument,")"]),
            None => json!(["span"," not ","(",argument,")"])
        } 
    } else { 
        match rdfa_property {
            Some(p) => json!(["span",{"property":p}," not ",argument]),
            None => json!(["span"," not ",argument])
        } 
    }
}

pub fn translate_inverse_of(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value { 

    //TODO: use propertytranslation?
    let argument = translate(&ofn[1], subject_2_label, Some(OWL_INVERSE_OF));

    if ofn[1].is_array() { 
        match rdfa_property {
            Some(p) => json!(["span",{"property":p},"inverse","(",argument,")"]),
            None => json!(["span"," inverse ","(",argument,")"])
        } 
    } else {
        match rdfa_property {
            Some(p) => json!(["span",{"property":p},"inverse",argument]),
            None => json!(["span"," inverse ",argument])
        } 
    }
}


pub fn translate_some_values_from(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value { 

    //TODO: use propertytranslation?
    let property = translate(&ofn[1], subject_2_label, Some(OWL_ON_PROPERTY));
    let modifier = Value::String(String::from(" some "));
    let filler = translate(&ofn[2], subject_2_label, Some(OWL_SOME_VALUES_FROM)); 

    //check whether the filler of this expression is atomic or nested further
    if ofn[2].is_array() {
        //in the case of further nesting: introduce brackets around the filler
        render_restriction_nested(&property, &modifier, &filler, rdfa_property) 
    } else { 
        render_restriction_base(&property, &modifier, &filler, rdfa_property) 
    } 
}

pub fn translate_has_value(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value { 

    //TODO: use propertytranslation?
    let property = translate(&ofn[1], subject_2_label, Some(OWL_ON_PROPERTY));
    let modifier = Value::String(String::from(" value "));
    let filler = translate(&ofn[2], subject_2_label, Some(OWL_HAS_VALUE)); 

    //check whether the filler of this expression is atomic or nested further
    if ofn[2].is_array() {
        //in the case of further nesting: introduce brackets around the filler
        render_restriction_nested(&property, &modifier, &filler, rdfa_property) 
    } else { 
        render_restriction_base(&property, &modifier, &filler, rdfa_property) 
    } 
}

pub fn translate_has_self(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value { 

    //TODO: use propertytranslation?
    let property = translate(&ofn[1], subject_2_label, Some(OWL_ON_PROPERTY));
    let modifier = Value::String(String::from(" some Self "));
    let filler = json!(["span", {"property":OWL_HAS_SELF, "hidden":"true"}, format!("true^^{XSD_BOOLEAN}")]); 

    //check whether the filler of this expression is atomic or nested further
    if ofn[2].is_array() {
        //in the case of further nesting: introduce brackets around the filler
        render_restriction_nested(&property, &modifier, &filler, rdfa_property) 
    } else { 
        render_restriction_base(&property, &modifier, &filler, rdfa_property) 
    } 
}

pub fn translate_all_values_from(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value { 

    //TODO: use propertytranslation?
    let property = translate(&ofn[1], subject_2_label, Some(OWL_ON_PROPERTY));
    let modifier = Value::String(String::from(" only "));
    let filler = translate(&ofn[2], subject_2_label, Some(OWL_ALL_VALUES_FROM)); 

    if ofn[2].is_array() {
        render_restriction_nested(&property, &modifier, &filler, rdfa_property) 
    } else { 
        render_restriction_base(&property, &modifier, &filler, rdfa_property) 
    } 
}

pub fn translate_min_cardinality(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value { 

    //TODO: use propertytranslation?
    let property = translate(&ofn[1], subject_2_label, Some(OWL_ON_PROPERTY));
    let modifier = Value::String(String::from(" min "));

    //encode cardinality
    let number = String::from(ofn[2].as_str().unwrap());
    //I am expecting OFN S-expressions to follow OWL functional syntax
    //so, datatypes for numbers do not need to be added
    //number.push_str("^^xsd:nonNegativeInteger");
    let card = json!(["span", {"property":OWL_MIN_CARDINALITY}, number]); 

    //let filler = translate(&ofn[2], subject_2_label, Some(OWL_ALL_VALUES_FROM)); 

    if ofn[2].is_array() {
        render_restriction_nested(&property, &modifier, &card, rdfa_property) 
    } else { 
        render_restriction_base(&property, &modifier, &card, rdfa_property) 
    } 
}

pub fn translate_min_qualified_cardinality(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value { 

    //TODO: use propertytranslation?
    let property = translate(&ofn[1], subject_2_label, Some(OWL_ON_PROPERTY));
    let modifier = Value::String(String::from(" min "));

    //encode cardinality
    let number = String::from(ofn[2].as_str().unwrap());
    //I am expecting OFN S-expressions to follow OWL functional syntax
    //so, datatypes for numbers do not need to be added
    //number.push_str("^^xsd:nonNegativeInteger");
    let card = json!(["span", {"property":OWL_MIN_QUALIFIED_CARDINALITY}, number]); 

    let filler = translate(&ofn[3], subject_2_label, Some(OWL_ON_CLASS)); 

    if ofn[2].is_array() {
        render_qualified_cardinality_restriction_nested(&property, &modifier, &card, &filler, rdfa_property) 
    } else { 
        render_qualified_cardinality_restriction_base(&property, &modifier, &card, &filler, rdfa_property) 
    } 
}

pub fn translate_max_cardinality(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value { 

    //TODO: use propertytranslation?
    let property = translate(&ofn[1], subject_2_label, Some(OWL_ON_PROPERTY));
    let modifier = Value::String(String::from(" max "));

    //encode cardinality
    let number = String::from(ofn[2].as_str().unwrap());
    //I am expecting OFN S-expressions to follow OWL functional syntax
    //so, datatypes for numbers do not need to be added
    //number.push_str("^^xsd:nonNegativeInteger");
    let card = json!(["span", {"property":OWL_MAX_CARDINALITY}, number]); 

    //let filler = translate(&ofn[2], subject_2_label, Some(OWL_ALL_VALUES_FROM)); 

    if ofn[2].is_array() {
        render_restriction_nested(&property, &modifier, &card, rdfa_property) 
    } else { 
        render_restriction_base(&property, &modifier, &card, rdfa_property) 
    } 
}

pub fn translate_max_qualified_cardinality(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value { 

    //TODO: use propertytranslation?
    let property = translate(&ofn[1], subject_2_label, Some(OWL_ON_PROPERTY));
    let modifier = Value::String(String::from(" max "));

    //encode cardinality
    let number = String::from(ofn[2].as_str().unwrap());
    //I am expecting OFN S-expressions to follow OWL functional syntax
    //so, datatypes for numbers do not need to be added
    //number.push_str("^^xsd:nonNegativeInteger");
    let card = json!(["span", {"property":OWL_MAX_QUALIFIED_CARDINALITY}, number]); 

    let filler = translate(&ofn[3], subject_2_label, Some(OWL_ON_CLASS)); 

    if ofn[2].is_array() {
        render_qualified_cardinality_restriction_nested(&property, &modifier, &card, &filler, rdfa_property) 
    } else { 
        render_qualified_cardinality_restriction_base(&property, &modifier, &card, &filler, rdfa_property) 
    } 
}

pub fn translate_exact_cardinality(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value { 

    //TODO: use propertytranslation?
    let property = translate(&ofn[1], subject_2_label, Some(OWL_ON_PROPERTY));
    let modifier = Value::String(String::from(" exactly "));

    //encode cardinality
    let number = String::from(ofn[2].as_str().unwrap());
    //I am expecting OFN S-expressions to follow OWL functional syntax
    //so, datatypes for numbers do not need to be added
    //number.push_str("^^xsd:nonNegativeInteger");
    let card = json!(["span", {"property":OWL_CARDINALITY}, number]); 

    //let filler = translate(&ofn[2], subject_2_label, Some(OWL_ALL_VALUES_FROM)); 

    if ofn[2].is_array() {
        render_restriction_nested(&property, &modifier, &card, rdfa_property) 
    } else { 
        render_restriction_base(&property, &modifier, &card, rdfa_property) 
    } 
}

pub fn translate_exact_qualified_cardinality(ofn: &Value, subject_2_label: &HashMap<String,String>, rdfa_property: Option<&str>) -> Value { 

    //TODO: use propertytranslation?
    let property = translate(&ofn[1], subject_2_label, Some(OWL_ON_PROPERTY));
    let modifier = Value::String(String::from(" exactly "));

    //encode cardinality
    let number = String::from(ofn[2].as_str().unwrap());
    //I am expecting OFN S-expressions to follow OWL functional syntax
    //so, datatypes for numbers do not need to be added
    //number.push_str("^^xsd:nonNegativeInteger");
    let card = json!(["span", {"property":OWL_QUALIFIED_CARDINALITY}, number]); 

    let filler = translate(&ofn[3], subject_2_label, Some(OWL_ON_CLASS)); 

    if ofn[2].is_array() {
        render_qualified_cardinality_restriction_nested(&property, &modifier, &card, &filler, rdfa_property) 
    } else { 
        render_qualified_cardinality_restriction_base(&property, &modifier, &card, &filler, rdfa_property) 
    } 
}


