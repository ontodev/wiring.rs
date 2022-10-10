use crate::ldtab_2_ofn::class_translation as class_translation; 
use crate::owl::thick_triple as owl;
use serde_json::{Value};

/// Given two OWL expressions subclass and superclass
/// return the OFN S-expression ["SubClassOf",T(subclass),T(superclass)],
/// where T(subclass) and T(superclass) are OFN S-expressions
/// of both subclass and superclass respectively.
pub fn translate_subclass_of_axiom(subclass: &owl::OWL, superclass: &owl::OWL) -> Value {

    let lhs : Value = class_translation::translate(subclass);
    let rhs: Value = class_translation::translate(superclass); 

    let operator = Value::String(String::from("SubClassOf"));
    let v = vec![operator, lhs, rhs];
    Value::Array(v) 
}

/// Given two OWL expressions subject and object
/// return the OFN S-expression ["EquivalentClasses",T(subject),T(object)],
/// where T(subject) and T(object) are OFN S-expressions for class expressions
/// of both subject and object respectively.
/// If the types of subject and object are unknown,
/// then return ["Equivalent",T(subject),T(object)].
pub fn translate_equivalent_class(subject: &owl::OWL, object: &owl::OWL) -> Value {

    let lhs : Value = class_translation::translate(subject);
    let mut rhs: Value = class_translation::translate(object); 

    match object {
        owl::OWL::Members(_) => {
            let operator = Value::String(String::from("EquivalentClasses"));
            let mut equivalent = vec![operator];
            let arguments = rhs.as_array_mut().unwrap();
            //equivalent.push(lhs); //LHS is a (generated) blank node
            equivalent.append(arguments);
            Value::Array(equivalent.to_vec())
        },
        _ => {

            //type ambiguity:
            //"owl:equivalentClass" is also used for DatatypeDefinition
            let operator = Value::String(String::from("Equivalent"));
            let v = vec![operator, lhs, rhs];
            Value::Array(v) 
        },
    }
}

/// Given two OWL expressions subject and object
/// return the OFN S-expression ["EquivalentProperties",T(subject),T(object)],
/// where T(subject) and T(object) are OFN S-expressions
/// of both subject and object respectively.
pub fn translate_equivalent_properties(subject: &owl::OWL, object: &owl::OWL) -> Value {

    let lhs : Value = class_translation::translate(subject);
    let mut rhs: Value = class_translation::translate(object); 

    match object {
        owl::OWL::Members(_) => {
            let operator = Value::String(String::from("EquivalentProperties"));
            let mut equivalent = vec![operator];
            let arguments = rhs.as_array_mut().unwrap();
            //equivalent.push(lhs); //LHS is a (generated) blank node
            equivalent.append(arguments);
            Value::Array(equivalent.to_vec())
        },
        _ => { 
            let operator = Value::String(String::from("EquivalentProperties"));
            let v = vec![operator, lhs, rhs];
            Value::Array(v) 
        },
    }
}

/// Given two OWL expressions subject and object
/// return the OFN S-expression ["DisjointProperties",T(subject),T(object)],
/// where T(subject) and T(object) are OFN S-expressions
/// of both subject and object respectively.
pub fn translate_property_disjoint_with(subject :&owl::OWL, object: &owl::OWL) -> Value {

    let lhs : Value = class_translation::translate(subject);
    let rhs: Value = class_translation::translate(object); 

    let operator = Value::String(String::from("DisjointProperties"));
    let v = vec![operator, lhs, rhs];
    Value::Array(v) 
}

/// Given two OWL expressions subject and object
/// return the OFN S-expression ["SubPropertyOf",T(subject),T(object)],
/// where T(subject) and T(object) are OFN S-expressions
/// of both subject and object respectively.
pub fn translate_sub_property_of(subject :&owl::OWL, object: &owl::OWL) -> Value {

    let lhs : Value = class_translation::translate(subject);
    let rhs: Value = class_translation::translate(object); 

    let operator = Value::String(String::from("SubPropertyOf"));

    let v = vec![operator, lhs, rhs];
    Value::Array(v) 
}

/// Given two OWL expressions subject and object
/// return the OFN S-expression ["DisjointProperties",T(object)],
/// where T(object) is a list of either object or data properties.
/// The argument subject corresponds to a skolemised blanknode
/// that can be ignored.
pub fn translate_all_disjoint_properties(_subject :&owl::OWL, object: &owl::OWL) -> Value {

    //let lhs : Value = class_translation::translate(subject);
    let mut rhs: Value = class_translation::translate(object); 

    let operator = Value::String(String::from("DisjointProperties"));

    let mut equivalent = vec![operator];
    let arguments = rhs.as_array_mut().unwrap();
    //equivalent.push(lhs); //LHS is a (generated) blank node
    equivalent.append(arguments);
    Value::Array(equivalent.to_vec()) 
}

/// Given an OWL expressions operands,
/// return the OFN S-expression ["DisjointClasses",T(object)],
/// where T(object) is a list of class expressions.
pub fn translate_disjoint_classes(operands: &owl::OWL) -> Value {

    let mut arguments: Value = class_translation::translate(operands); 

    let operator = Value::String(String::from("DisjointClasses"));
    let mut disjoint = vec![operator];
    let arguments = arguments.as_array_mut().unwrap();
    disjoint.append(arguments);
    Value::Array(disjoint.to_vec())
}

/// Given the OWL expressions subject and object,
/// return the OFN S-expression ["DisjointClasses",T(subject),T(object)],
/// where T(subject) and T(object) are OFN S-expressions for classes.
pub fn translate_disjoint_with(subject: &owl::OWL, object: &owl::OWL) -> Value {

    let lhs : Value = class_translation::translate(subject);
    let rhs: Value = class_translation::translate(object); 

    let operator = Value::String(String::from("DisjointClasses"));
    let v = vec![operator, lhs, rhs];
    Value::Array(v) 
}

/// Given the OWL expressions union and operands,
/// return the OFN S-expression ["DisjointUnion",T(union),T(operands)],
/// where T(union) and T(operands) are OFN S-expressions for classes.
pub fn translate_disjoint_union(union: &owl::OWL, operands: &owl::OWL) -> Value {

    let lhs : Value = class_translation::translate(union);
    let mut rhs: Value = class_translation::translate(operands); 

    let operator = Value::String(String::from("DisjointUnionOf"));
    let mut union = vec![operator];
    union.push(lhs);
    let arguments = rhs.as_array_mut().unwrap();
    union.append(arguments);
    Value::Array(union.to_vec())
}

/// Given an OWL operator represented as a string in RDF,
/// return its associated operator for OFN S-expressions. 
pub fn get_ofn_operator(op : &str) -> Value {

    match op {
        "rdfs:Datatype" => Value::String(String::from("Datatype")),
        "owl:Class" => Value::String(String::from("Class")),
        "owl:ObjectProperty" => Value::String(String::from("ObjectProperty")),
        "owl:DatatypeProperty" => Value::String(String::from("DataProperty")),
        "owl:AnnotationProperty" => Value::String(String::from("AnnotationProperty")),
        "owl:NamedIndividual" => Value::String(String::from("NamedIndividual")),

        "owl:FunctionalProperty" => Value::String(String::from("FunctionalProperty")),

        "owl:InverseFunctionalProperty" => Value::String(String::from("InverseObjectFunctionalProperty")),
        "owl:ReflexiveProperty" => Value::String(String::from("ReflexiveObjectProperty")),
        "owl:IrreflexiveProperty" => Value::String(String::from("IrreflexiveObjectProperty")),
        "owl:SymmetricProperty" => Value::String(String::from("SymmetricObjectProperty")),
        "owl:AsymmetricProperty" => Value::String(String::from("AsymmetricObjectProperty")),
        "owl:TransitiveProperty" => Value::String(String::from("TransitiveObjectProperty")),

        "owl:AllDifferent" => Value::String(String::from("DifferentIndividuals")),

        "owl:Ontology" => Value::String(String::from("ThinTriple")),

        _ => Value::String(String::from("ClassAssertion")), 
    } 
} 

/// Given the OWL expressions lhs and rhs of an RDF triple with property "rdf:type",
/// return the corresponding OFN S-expression. 
pub fn translate_rdf_type(lhs: &owl::OWL, rhs: &owl::OWL) -> Value {

    let operator = match rhs {
        owl::OWL::Named(x) => get_ofn_operator(x),
        _ => Value::String(String::from("ClassAssertion")),
    };

    let lhs : Value = class_translation::translate(lhs);
    let rhs: Value = class_translation::translate(rhs); 

    //check whether operator is a declaration
    if operator.to_string().eq("\"Datatype\"") ||
       operator.to_string().eq("\"Class\"") ||
       operator.to_string().eq("\"ObjectProperty\"") ||
       operator.to_string().eq("\"DataProperty\"") ||
       operator.to_string().eq("\"AnnotationProperty\"") ||
       operator.to_string().eq("\"NamedIndividual\"")
    {
        let v = vec![operator, lhs];
        let v = Value::Array(v);

        let v = vec![Value::String(String::from("Declaration")),v];
        Value::Array(v)

    } else if operator.to_string().eq("\"FunctionalProperty\"") ||
            operator.to_string().eq("\"InverseObjectFunctionalProperty\"") ||
            operator.to_string().eq("\"ReflexiveObjectProperty\"") ||
            operator.to_string().eq("\"IrreflexiveObjectProperty\"") ||
            operator.to_string().eq("\"SymmetricObjectProperty\"") ||
            operator.to_string().eq("\"AsymmetricObjectProperty\"") ||
            operator.to_string().eq("\"TransitiveObjectProperty\"")
    {
        let v = vec![operator, lhs];
        Value::Array(v) 

    } else if operator.to_string().eq("\"ClassAssertion\"") {

        let v = vec![operator, rhs, lhs];
        Value::Array(v) 

    }
    //else if operator.to_string().eq("\"DifferentIndividuals\"") {
    //    let v = vec![operator, lhs, rhs];
    //    Value::Array(v) 
    //}
    else { 
        let v = vec![operator, lhs, rhs];
        Value::Array(v) 
    }
}

/// Given two OWL expressions property and domain
/// return the OFN S-expression ["Domain",T(property),T(domain)],
/// where T(property) and T(domain) are OFN S-expressions.
pub fn translate_domain(property: &owl::OWL, domain: &owl::OWL) -> Value {

    let operator = Value::String(String::from("Domain"));

    let property : Value = class_translation::translate(property);//TODO: refactor class/property translation
    let domain: Value = class_translation::translate(domain); 

    let v = vec![operator, property, domain];
    Value::Array(v) 
} 

/// Given two OWL expressions property and domain
/// return the OFN S-expression ["Range",T(property),T(domain)],
/// where T(property) and T(domain) are OFN S-expressions.
pub fn translate_range(property: &owl::OWL, domain: &owl::OWL) -> Value {

    let operator = Value::String(String::from("Range"));

    let property : Value = class_translation::translate(property);//TODO: refactor class/property translation
    let domain: Value = class_translation::translate(domain); 

    let v = vec![operator, property, domain];
    Value::Array(v) 
} 

/// Given two OWL expressions lhs and rhs
/// return the OFN S-expression ["InverseObjectProperties",T(lhs),T(rhs)],
/// where T(lhs) and T(rhs) are OFN S-expressions.
pub fn translate_inverse_object_properties(lhs: &owl::OWL, rhs: &owl::OWL) -> Value {

    let lh : Value = class_translation::translate(lhs);//TODO: refactor class/property translation
    let rh : Value = class_translation::translate(rhs);//TODO: refactor class/property translation

    let operator = Value::String(String::from("InverseObjectProperties")); 
    let v = vec![operator, lh, rh];
    Value::Array(v) 
} 

/// Given two OWL expressions lhs and rhs
/// return the OFN S-expression ["SameIndividual",T(lhs),T(rhs)],
/// where T(lhs) and T(rhs) are OFN S-expressions.
pub fn translate_same_as(lhs: &owl::OWL, rhs: &owl::OWL) -> Value {

    let lh : Value = class_translation::translate(lhs);
    let rh : Value = class_translation::translate(rhs);

    let operator = Value::String(String::from("SameIndividual")); 
    let v = vec![operator, lh, rh];
    Value::Array(v) 
} 

/// Given two OWL expressions lhs and rhs
/// return the OFN S-expression ["SameIndividual",T(lhs),T(rhs)],
/// where T(lhs) and T(rhs) are OFN S-expressions.
pub fn translate_all_same_as(_lhs: &owl::OWL, rhs: &owl::OWL) -> Value {

    let mut arguments: Value = class_translation::translate(rhs); 

    let operator = Value::String(String::from("SameIndividuals"));
    let mut res = vec![operator];
    let arguments = arguments.as_array_mut().unwrap();
    res.append(arguments);
    Value::Array(res.to_vec()) 
}

/// Given two OWL expressions lhs and rhs
/// return the OFN S-expression ["DifferentIndividuals",T(lhs),T(rhs)],
/// where T(lhs) and T(rhs) are OFN S-expressions.
pub fn translate_different_from(lhs: &owl::OWL, rhs: &owl::OWL) -> Value {

    let lh : Value = class_translation::translate(lhs);
    let rh : Value = class_translation::translate(rhs);

    let operator = Value::String(String::from("DifferentIndividuals")); 
    let v = vec![operator, lh, rh];
    Value::Array(v) 
} 

/// Given two OWL expressions lhs and rhs
/// return the OFN S-expression ["DifferentIndividuals",T(lhs),T(rhs)],
/// where T(lhs) and T(rhs) are OFN S-expressions.
pub fn translate_all_different(_lhs: &owl::OWL, rhs: &owl::OWL) -> Value {

    let mut arguments: Value = class_translation::translate(rhs); 

    let operator = Value::String(String::from("DifferentIndividuals"));
    let mut res = vec![operator];
    let arguments = arguments.as_array_mut().unwrap();
    res.append(arguments);
    Value::Array(res.to_vec()) 
}

/// Given two OWL expressions lhs and rhs
/// return the OFN S-expression ["ObjectPropertyChain",T(lhs),T(rhs)],
/// where T(lhs) and T(rhs) are OFN S-expressions.
pub fn translate_property_chain(lhs: &owl::OWL, rhs: &owl::OWL) -> Value {

    let lhs : Value = class_translation::translate(lhs);
    let mut rhs: Value = class_translation::translate(rhs); //this is a list

    let operator = Value::String(String::from("ObjectPropertyChain"));
    let mut res = vec![operator];
    let arguments = rhs.as_array_mut().unwrap();
    res.append(arguments);
    let chain = Value::Array(res);

    let operator = Value::String(String::from("SubObjectPropertyOf")); 
    let v = vec![operator, chain, lhs];
    Value::Array(v) 
}

/// Given the OWL expression rhs encoding a NegativePropertyAssertion
/// return the corresponding OFN S-expression.
pub fn translate_negative_property_assertion(_lhs: &owl::OWL, rhs: &owl::OWL) -> Value { 
    //NB: this returns an axiom rather than an expression 
    let axiom : Value = class_translation::translate(rhs); 
    axiom
}

/// Given the OWL expressions lhs and rhs, 
/// return the OFN S-expression ["HasKey", T(lhs), T(rhs)]
/// where T(lhs) and T(rhs) are lists of OFN S-expressions.
pub fn translate_has_key(lhs: &owl::OWL, rhs: &owl::OWL) -> Value { 
    let class : Value = class_translation::translate(lhs); 
    let mut keys : Value = class_translation::translate(rhs); //this is a list

    let operator = Value::String(String::from("HasKey"));
    let mut res = vec![operator];
    let arguments = keys.as_array_mut().unwrap();
    res.push(class);
    res.append(arguments);
    Value::Array(res.to_vec()) 
}

/// Given the OWL expressions lhs and rhs, 
/// return the OFN S-expression ["Import", T(lhs), T(rhs)]
/// where T(lhs) and T(rhs) are lists of OFN S-expressions.
pub fn translate_import(lhs :&owl::OWL, rhs : &owl::OWL) -> Value {

    let ontology : Value = class_translation::translate(lhs); 
    let import : Value = class_translation::translate(rhs); 

    let operator = Value::String(String::from("Import"));
    let mut res = vec![operator];
    res.push(ontology);
    res.push(import);

    Value::Array(res)

}

/// Given a subject s, a property p, and an object o,
/// return the OFN S-expression ["ThinTriple", T(s), T(p), T(o)]
/// where T(s), T(p), T(o) are OFN S-expressions.
pub fn translate_thin_triple(s: &str, p: &str, o: &str) -> Value {
    //NB: AnnotationAssertions are ambiguous and are translated as ThickTriples
    //T(Property Subject Object)  rather then T(Subject Property Object)

    let subject: Value = serde_json::from_str(s).unwrap(); 
    let predicate: Value = serde_json::from_str(p).unwrap(); 
    let object: Value = serde_json::from_str(o).unwrap(); 

    let operator = Value::String(String::from("ThinTriple"));
    let v = vec![operator, subject, predicate, object];
    Value::Array(v) 
} 
