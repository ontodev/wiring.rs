use crate::ldtab_2_ofn::class_translation;
use crate::owl::thick_triple as owl;
use serde_json::Value;

/// Given two OWL expressions subclass and superclass
/// return the OFN S-expression ["SubClassOf",T(subclass),T(superclass)],
/// where T(subclass) and T(superclass) are OFN S-expressions
/// of both subclass and superclass respectively.
///
/// #Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let subclass = "\"obo:IAO_0000120\"";
/// let superclass = "\"obo:IAO_0000121\"";
///
/// let subclass_owl : owl::OWL = serde_json::from_str(subclass).unwrap();
/// let superclass_owl : owl::OWL = serde_json::from_str(superclass).unwrap();
///
/// let axiom : Value = translation::translate_subclass_of_axiom(&subclass_owl, &superclass_owl);
/// let axiom_expected_string = r#"["SubClassOf","obo:IAO_0000120","obo:IAO_0000121"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
/// ```
pub fn translate_subclass_of_axiom(subclass: &owl::OWL, superclass: &owl::OWL) -> Value {
    let lhs: Value = class_translation::translate(subclass);
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
///
/// #Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let subclass = "\"obo:IAO_0000120\"";
/// let superclass = "\"obo:IAO_0000121\"";
///
/// let subclass_owl : owl::OWL = serde_json::from_str(subclass).unwrap();
/// let superclass_owl : owl::OWL = serde_json::from_str(superclass).unwrap();
///
/// let axiom : Value = translation::translate_equivalent_class(&subclass_owl, &superclass_owl);
/// let axiom_expected_string = r#"["EquivalentClasses","obo:IAO_0000120","obo:IAO_0000121"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
///
/// let subclass = "\"_:genID123\"";
/// let superclass = r#"{"owl:members":[{"datatype":"_JSON","object":{"datatype":"_JSON","rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000120"}],"rdf:rest":[{"datatype":"_JSON","object":{"datatype":"_JSON","rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000121"}],"rdf:rest":[{"datatype":"_JSON","object":{"datatype":"_JSON","rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000122"}],"rdf:rest":[{"datatype":"_IRI","object":"rdf:nil"}]}}]}}]}}]}"#;
///
/// let subclass_owl : owl::OWL = serde_json::from_str(subclass).unwrap();
/// let superclass_owl : owl::OWL = serde_json::from_str(superclass).unwrap();
///
/// let axiom : Value = translation::translate_equivalent_class(&subclass_owl, &superclass_owl);
/// let axiom_expected_string = r#"["EquivalentClasses","obo:IAO_0000120","obo:IAO_0000121","obo:IAO_0000122"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
/// ```
pub fn translate_equivalent_class(subject: &owl::OWL, object: &owl::OWL) -> Value {

    let lhs = class_translation::translate(subject);
    let rhs = class_translation::translate(object);

    match (object, rhs.clone()) {
        //equivalent classes axiom with multiple arguments
        //(in this case, the lhs is a blank node)
        (owl::OWL::Members(_), Value::Array(mut arr)) => {
            let mut axiom = Vec::with_capacity(arr.len() + 1);
            axiom.push(Value::String("EquivalentClasses".into()));
            axiom.append(&mut arr);
            Value::Array(axiom)
        }
        //equivalent classes axiom with 2 arguments
        (_, _) => {
            Value::Array(vec![
                Value::String("EquivalentClasses".into()),
                lhs,
                rhs,
            ])
        }
    }
}



/// Given two OWL expressions subject and object
/// return the OFN S-expression ["EquivalentProperties",T(subject),T(object)],
/// where T(subject) and T(object) are OFN S-expressions
/// of both subject and object respectively.
///
/// # Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let subproperty = "\"obo:IAO_0000120\"";
/// let superproperty = "\"obo:IAO_0000121\"";
///
/// let subproperty_owl : owl::OWL = serde_json::from_str(subproperty).unwrap();
/// let superproperty_owl : owl::OWL = serde_json::from_str(superproperty).unwrap();
///
/// let axiom : Value = translation::translate_equivalent_properties(&subproperty_owl,
/// &superproperty_owl);
/// let axiom_expected_string = r#"["EquivalentProperties","obo:IAO_0000120","obo:IAO_0000121"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
///```
pub fn translate_equivalent_properties(subject: &owl::OWL, object: &owl::OWL) -> Value {
    let lhs = class_translation::translate(subject);
    let rhs = class_translation::translate(object);

    match (object, rhs) {
        (owl::OWL::Members(_), Value::Array(mut arr)) => {
            let mut axiom = Vec::with_capacity(arr.len() + 1);
            axiom.push(Value::String("EquivalentProperties".into()));

            axiom.append(&mut arr);
            Value::Array(axiom)
        }
        (_, rhs) => {
            Value::Array(vec![
                Value::String("EquivalentProperties".into()),
                lhs,
                rhs,
            ])
        }
    }
}




/// Given two OWL expressions subject and object
/// return the OFN S-expression ["DisjointProperties",T(subject),T(object)],
/// where T(subject) and T(object) are OFN S-expressions
/// of both subject and object respectively.
///
/// #Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let property_1 = "\"obo:IAO_0000120\"";
/// let property_2 = "\"obo:IAO_0000121\"";
///
/// let property_1_owl : owl::OWL = serde_json::from_str(property_1).unwrap();
/// let property_2_owl : owl::OWL = serde_json::from_str(property_2).unwrap();
///
/// let axiom : Value = translation::translate_property_disjoint_with(&property_1_owl,
/// &property_2_owl);
/// let axiom_expected_string = r#"["DisjointProperties","obo:IAO_0000120","obo:IAO_0000121"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
///```
pub fn translate_property_disjoint_with(subject: &owl::OWL, object: &owl::OWL) -> Value {
    let lhs: Value = class_translation::translate(subject);
    let rhs: Value = class_translation::translate(object);

    let operator = Value::String(String::from("DisjointProperties"));
    let v = vec![operator, lhs, rhs];
    Value::Array(v)
}

/// Given two OWL expressions subject and object
/// return the OFN S-expression ["SubPropertyOf",T(subject),T(object)],
/// where T(subject) and T(object) are OFN S-expressions
/// of both subject and object respectively.
///
/// #Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let property_1 = "\"obo:IAO_0000120\"";
/// let property_2 = "\"obo:IAO_0000121\"";
///
/// let property_1_owl : owl::OWL = serde_json::from_str(property_1).unwrap();
/// let property_2_owl : owl::OWL = serde_json::from_str(property_2).unwrap();
///
/// let axiom : Value = translation::translate_sub_property_of(&property_1_owl,
/// &property_2_owl);
/// let axiom_expected_string = r#"["SubPropertyOf","obo:IAO_0000120","obo:IAO_0000121"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
///```
///
pub fn translate_sub_property_of(subject: &owl::OWL, object: &owl::OWL) -> Value {
    let lhs: Value = class_translation::translate(subject);
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
///
/// #Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let blank_node = "\"_:genID123\"";
/// let properties = r#"{"owl:members":[{"datatype":"_JSON","object":{"rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000120"}],"rdf:rest":[{"datatype":"_JSON","object":{"rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000121"}],"rdf:rest":[{"datatype":"_JSON","object":{"rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000122"}],"rdf:rest":[{"datatype":"_IRI","object":"rdf:nil"}]}}]}}]}}]}"#;
///
/// let blank_node_owl : owl::OWL = serde_json::from_str(blank_node).unwrap();
/// let properties_owl : owl::OWL = serde_json::from_str(properties).unwrap();
///
/// let axiom : Value = translation::translate_all_disjoint_properties(&blank_node_owl, &properties_owl);
/// let axiom_expected_string = r#"["DisjointProperties","obo:IAO_0000120","obo:IAO_0000121","obo:IAO_0000122"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
///```
pub fn translate_all_disjoint_properties(_subject: &owl::OWL, object: &owl::OWL) -> Value {
    let rhs = class_translation::translate(object);

    let operator = Value::String("DisjointProperties".into());

    match rhs {
        Value::Array(mut arr) => {
            let mut axiom = Vec::with_capacity(arr.len() + 1);
            axiom.push(operator);
            axiom.append(&mut arr);
            Value::Array(axiom)
        }
        _ => {
            Value::Array(vec![operator]) //TODO: error handling
        }
    }
}

/// Given an OWL expressions operands,
/// return the OFN S-expression ["DisjointClasses",T(object)],
/// where T(object) is a list of class expressions.
///
/// #Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let operands = r#"{"owl:members":[{"datatype":"_JSON","object":{"datatype":"_JSON","rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000120"}],"rdf:rest":[{"datatype":"_JSON","object":{"datatype":"_JSON","rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000121"}],"rdf:rest":[{"datatype":"_JSON","object":{"datatype":"_JSON","rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000122"}],"rdf:rest":[{"datatype":"_IRI","object":"rdf:nil"}]}}]}}]}}]}"#;
/// let operands_owl : owl::OWL = serde_json::from_str(operands).unwrap();
///
/// let axiom : Value = translation::translate_disjoint_classes(&operands_owl);
/// let axiom_expected_string = r#"["DisjointClasses","obo:IAO_0000120","obo:IAO_0000121","obo:IAO_0000122"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
/// ```
pub fn translate_disjoint_classes(operands: &owl::OWL) -> Value {
    let arguments = class_translation::translate(operands);

    let operator = Value::String("DisjointClasses".into());

    match arguments {
        Value::Array(mut arr) => {
            let mut axiom = Vec::with_capacity(arr.len() + 1);
            axiom.push(operator);
            axiom.append(&mut arr);
            Value::Array(axiom)
        }
        _ => {
            Value::Array(vec![operator]) //TODO: error handling
        }
    }
}

/// Given the OWL expressions subject and object,
/// return the OFN S-expression ["DisjointClasses",T(subject),T(object)],
/// where T(subject) and T(object) are OFN S-expressions for classes.
///
/// #Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let class_1 = "\"obo:IAO_0000120\"";
/// let class_2 = "\"obo:IAO_0000121\"";
///
/// let class_1_owl : owl::OWL = serde_json::from_str(class_1).unwrap();
/// let class_2_owl : owl::OWL = serde_json::from_str(class_2).unwrap();
///
/// let axiom : Value = translation::translate_disjoint_with(&class_1_owl,
/// &class_2_owl);
/// let axiom_expected_string = r#"["DisjointClasses","obo:IAO_0000120","obo:IAO_0000121"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
///```
pub fn translate_disjoint_with(subject: &owl::OWL, object: &owl::OWL) -> Value {
    let lhs: Value = class_translation::translate(subject);
    let rhs: Value = class_translation::translate(object);

    let operator = Value::String(String::from("DisjointClasses"));
    let v = vec![operator, lhs, rhs];
    Value::Array(v)
}

/// Given the OWL expressions union and operands,
/// return the OFN S-expression ["DisjointUnion",T(union),T(operands)],
/// where T(union) and T(operands) are OFN S-expressions for classes.
///
/// # Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let union = "\"obo:IAO_0000120\"";
/// let operands = r#"{"datatype":"_JSON","rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000121"}],"rdf:rest":[{"datatype":"_JSON","object":{"datatype":"_JSON","rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000122"}],"rdf:rest":[{"datatype":"_IRI","object":"rdf:nil"}]}}]}"#;
///
/// let union_owl : owl::OWL = serde_json::from_str(union).unwrap();
/// let operands_owl : owl::OWL = serde_json::from_str(operands).unwrap();
///
/// let axiom : Value = translation::translate_disjoint_union(&union_owl,
/// &operands_owl);
/// let axiom_expected_string = r#"["DisjointUnionOf","obo:IAO_0000120","obo:IAO_0000121","obo:IAO_0000122"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
///```
pub fn translate_disjoint_union(union: &owl::OWL, operands: &owl::OWL) -> Value {
    let lhs = class_translation::translate(union);
    let rhs = class_translation::translate(operands);

    let operator = Value::String("DisjointUnionOf".into());

    match rhs {
        Value::Array(mut arr) => {
            let mut axiom = Vec::with_capacity(arr.len() + 2);
            axiom.push(operator);
            axiom.push(lhs);
            axiom.append(&mut arr);
            Value::Array(axiom)
        }
        other => Value::Array(vec![operator, lhs, other]), //TODO: error handling
    }
}

/// Given an OWL operator represented as a string in RDF,
/// return its associated operator for OFN S-expressions.
/// # Examples
///
/// ```
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
///
/// let operator = "rdfs:Datatype";
/// let ofn_operator = translation::get_ofn_operator(operator);
/// let ofn_operator_expected = "Datatype";
///
/// assert_eq!(ofn_operator, ofn_operator_expected);
///```
pub fn get_ofn_operator(op: &str) -> Value {
    match op {
        "rdfs:Datatype" => Value::String(String::from("Datatype")),
        "owl:Class" => Value::String(String::from("Class")),
        "owl:ObjectProperty" => Value::String(String::from("ObjectProperty")),
        "owl:DatatypeProperty" => Value::String(String::from("DataProperty")),
        "owl:AnnotationProperty" => Value::String(String::from("AnnotationProperty")),
        "owl:NamedIndividual" => Value::String(String::from("NamedIndividual")),

        "owl:FunctionalProperty" => Value::String(String::from("FunctionalProperty")),

        "owl:InverseFunctionalProperty" => {
            Value::String(String::from("InverseObjectFunctionalProperty"))
        }
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
///
/// #Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let lhs = "\"obo:IAO_0000120\"";
/// let rhs = "\"owl:FunctionalProperty\"";
///
/// let lhs_owl : owl::OWL = serde_json::from_str(lhs).unwrap();
/// let rhs_owl : owl::OWL = serde_json::from_str(rhs).unwrap();
///
/// let axiom = translation::translate_rdf_type(&lhs_owl, &rhs_owl);
/// let axiom_expected_string = r#"["FunctionalProperty","obo:IAO_0000120"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
/// ```
pub fn translate_rdf_type(lhs: &owl::OWL, rhs: &owl::OWL) -> Value {
    let operator = match rhs {
        owl::OWL::Named(x) => get_ofn_operator(x),
        _ => Value::String(String::from("ClassAssertion")),
    };

    let lhs: Value = class_translation::translate(lhs);
    let rhs: Value = class_translation::translate(rhs);

    //check whether operator is a declaration
    if operator.to_string().eq("\"Datatype\"")
        || operator.to_string().eq("\"Class\"")
        || operator.to_string().eq("\"ObjectProperty\"")
        || operator.to_string().eq("\"DataProperty\"")
        || operator.to_string().eq("\"AnnotationProperty\"")
        || operator.to_string().eq("\"NamedIndividual\"")
    {
        let v = vec![operator, lhs];
        let v = Value::Array(v);

        let v = vec![Value::String(String::from("Declaration")), v];
        Value::Array(v)
    } else if operator.to_string().eq("\"FunctionalProperty\"")
        || operator
            .to_string()
            .eq("\"InverseObjectFunctionalProperty\"")
        || operator.to_string().eq("\"ReflexiveObjectProperty\"")
        || operator.to_string().eq("\"IrreflexiveObjectProperty\"")
        || operator.to_string().eq("\"SymmetricObjectProperty\"")
        || operator.to_string().eq("\"AsymmetricObjectProperty\"")
        || operator.to_string().eq("\"TransitiveObjectProperty\"")
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
///
/// #Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let property = "\"obo:IAO_0000120\"";
/// let domain = "\"obo:IAO_0000121\"";
///
/// let property_owl : owl::OWL = serde_json::from_str(property).unwrap();
/// let domain_owl : owl::OWL = serde_json::from_str(domain).unwrap();
///
/// let axiom : Value = translation::translate_domain(&property_owl, &domain_owl);
/// let axiom_expected_string = r#"["Domain","obo:IAO_0000120","obo:IAO_0000121"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
/// ```
pub fn translate_domain(property: &owl::OWL, domain: &owl::OWL) -> Value {
    let operator = Value::String(String::from("Domain"));

    let property: Value = class_translation::translate(property); //TODO: refactor class/property translation
    let domain: Value = class_translation::translate(domain);

    let v = vec![operator, property, domain];
    Value::Array(v)
}

/// Given two OWL expressions property and range
/// return the OFN S-expression ["Range",T(property),T(range)],
/// where T(property) and T(range) are OFN S-expressions.
///
/// #Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let property = "\"obo:IAO_0000120\"";
/// let range = "\"obo:IAO_0000121\"";
///
/// let property_owl : owl::OWL = serde_json::from_str(property).unwrap();
/// let domain_owl : owl::OWL = serde_json::from_str(range).unwrap();
///
/// let axiom : Value = translation::translate_range(&property_owl, &domain_owl);
/// let axiom_expected_string = r#"["Range","obo:IAO_0000120","obo:IAO_0000121"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
/// ```
pub fn translate_range(property: &owl::OWL, range: &owl::OWL) -> Value {
    let operator = Value::String(String::from("Range"));

    let property: Value = class_translation::translate(property); //TODO: refactor class/property translation
    let range: Value = class_translation::translate(range);

    let v = vec![operator, property, range];
    Value::Array(v)
}

/// Given two OWL expressions lhs and rhs
/// return the OFN S-expression ["InverseObjectProperties",T(lhs),T(rhs)],
/// where T(lhs) and T(rhs) are OFN S-expressions.
///
/// # Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let lhs = "\"obo:IAO_0000120\"";
/// let rhs = "\"obo:IAO_0000121\"";
///
/// let lhs_owl : owl::OWL = serde_json::from_str(lhs).unwrap();
/// let rhs_owl : owl::OWL = serde_json::from_str(rhs).unwrap();
///
/// let axiom : Value = translation::translate_inverse_object_properties(&lhs_owl, &rhs_owl);
/// let axiom_expected_string = r#"["InverseObjectProperties","obo:IAO_0000120","obo:IAO_0000121"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
/// ```
pub fn translate_inverse_object_properties(lhs: &owl::OWL, rhs: &owl::OWL) -> Value {
    let lh: Value = class_translation::translate(lhs); //TODO: refactor class/property translation
    let rh: Value = class_translation::translate(rhs); //TODO: refactor class/property translation

    let operator = Value::String(String::from("InverseObjectProperties"));
    let v = vec![operator, lh, rh];
    Value::Array(v)
}

/// Given two OWL expressions lhs and rhs
/// return the OFN S-expression ["SameIndividual",T(lhs),T(rhs)],
/// where T(lhs) and T(rhs) are OFN S-expressions.
///
/// # Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let individual_1 = "\"obo:IAO_0000120\"";
/// let individual_2 = "\"obo:IAO_0000121\"";
///
/// let individual_1_owl : owl::OWL = serde_json::from_str(individual_1).unwrap();
/// let individual_2_owl : owl::OWL = serde_json::from_str(individual_2).unwrap();
///
/// let axiom : Value = translation::translate_same_as(&individual_1_owl,
/// &individual_2_owl);
/// let axiom_expected_string = r#"["SameIndividual","obo:IAO_0000120","obo:IAO_0000121"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
/// ```
pub fn translate_same_as(lhs: &owl::OWL, rhs: &owl::OWL) -> Value {
    let lh: Value = class_translation::translate(lhs);
    let rh: Value = class_translation::translate(rhs);

    let operator = Value::String(String::from("SameIndividual"));
    let v = vec![operator, lh, rh];
    Value::Array(v)
}

/// Given two OWL expressions lhs and rhs
/// return the OFN S-expression ["SameIndividual",T(lhs),T(rhs)],
/// where T(lhs) and T(rhs) are OFN S-expressions.
///
/// # Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let blank_node = "\"_:genID123\"";
/// let arguments = r#"{"owl:members":[{"datatype":"_JSON","object":{"datatype":"_JSON","rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000120"}],"rdf:rest":[{"datatype":"_JSON","object":{"datatype":"_JSON","rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000121"}],"rdf:rest":[{"datatype":"_JSON","object":{"datatype":"_JSON","rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000122"}],"rdf:rest":[{"datatype":"_IRI","object":"rdf:nil"}]}}]}}]}}]}"#;
///
/// let blank_node_owl : owl::OWL = serde_json::from_str(blank_node).unwrap();
/// let arguments_owl : owl::OWL = serde_json::from_str(arguments).unwrap();
///
/// let axiom : Value = translation::translate_all_same_as(&blank_node_owl,
/// &arguments_owl);
/// let axiom_expected_string = r#"["SameIndividual","obo:IAO_0000120","obo:IAO_0000121","obo:IAO_0000122"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
///```
pub fn translate_all_same_as(_lhs: &owl::OWL, rhs: &owl::OWL) -> Value {
    let arguments = class_translation::translate(rhs);
    let operator = Value::String("SameIndividual".into());

    match arguments {
        Value::Array(mut arr) => {
            let mut axiom = Vec::with_capacity(arr.len() + 1);
            axiom.push(operator);
            axiom.append(&mut arr);
            Value::Array(axiom)
        }
        other => Value::Array(vec![operator, other]),//TODO: error handling
    }
}

/// Given two OWL expressions lhs and rhs
/// return the OFN S-expression ["DifferentIndividuals",T(lhs),T(rhs)],
/// where T(lhs) and T(rhs) are OFN S-expressions.
///
/// # Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let individual_1 = "\"obo:IAO_0000120\"";
/// let individual_2 = "\"obo:IAO_0000121\"";
///
/// let individual_1_owl : owl::OWL = serde_json::from_str(individual_1).unwrap();
/// let individual_2_owl : owl::OWL = serde_json::from_str(individual_2).unwrap();
///
/// let axiom : Value = translation::translate_different_from(&individual_1_owl,
/// &individual_2_owl);
/// let axiom_expected_string = r#"["DifferentIndividuals","obo:IAO_0000120","obo:IAO_0000121"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
/// ```
pub fn translate_different_from(lhs: &owl::OWL, rhs: &owl::OWL) -> Value {
    let lh: Value = class_translation::translate(lhs);
    let rh: Value = class_translation::translate(rhs);

    let operator = Value::String(String::from("DifferentIndividuals"));
    let v = vec![operator, lh, rh];
    Value::Array(v)
}

/// Given two OWL expressions lhs and rhs
/// return the OFN S-expression ["DifferentIndividuals",T(lhs),T(rhs)],
/// where T(lhs) and T(rhs) are OFN S-expressions.
///
/// # Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let blank_node = "\"_:genID123\"";
/// let arguments = r#"{"owl:members":[{"datatype":"_JSON","object":{"datatype":"_JSON","rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000120"}],"rdf:rest":[{"datatype":"_JSON","object":{"datatype":"_JSON","rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000121"}],"rdf:rest":[{"datatype":"_JSON","object":{"datatype":"_JSON","rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000122"}],"rdf:rest":[{"datatype":"_IRI","object":"rdf:nil"}]}}]}}]}}]}"#;
///
/// let blank_node_owl : owl::OWL = serde_json::from_str(blank_node).unwrap();
/// let arguments_owl : owl::OWL = serde_json::from_str(arguments).unwrap();
///
/// let axiom : Value = translation::translate_all_different(&blank_node_owl,
/// &arguments_owl);
/// let axiom_expected_string = r#"["DifferentIndividuals","obo:IAO_0000120","obo:IAO_0000121","obo:IAO_0000122"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
///```
pub fn translate_all_different(_lhs: &owl::OWL, rhs: &owl::OWL) -> Value {
    let arguments = class_translation::translate(rhs);
    let operator = Value::String("DifferentIndividuals".into());

    match arguments {
        Value::Array(mut arr) => {
            let mut axiom = Vec::with_capacity(arr.len() + 1);
            axiom.push(operator);
            axiom.append(&mut arr);
            Value::Array(axiom)
        }
        other => Value::Array(vec![operator, other]), //TODO: error handling
    }
}

/// Given two OWL expressions lhs and rhs
/// return the OFN S-expression ["ObjectPropertyChain",T(lhs),T(rhs)],
/// where T(lhs) and T(rhs) are OFN S-expressions.
///
/// let lhs = "\"obo:IAO_0000122\"";
/// let rhs = r#"{"rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000120"}],"rdf:rest":[{"datatype":"_JSON","object":{"rdf:first":[{"datatype":"_IRI","object":"obo:IAO_0000121"}],"rdf:rest":[{"datatype":"_IRI","object":"rdf:nil"}]}}]}"#;
///
/// let lhs_owl : owl::OWL = serde_json::from_str(lhs).unwrap();
/// let rhs_owl : owl::OWL = serde_json::from_str(rhs).unwrap();
///
/// let axiom : Value = translation::translate_all_disjoint_properties(&lhs_owl, &rhs_owl);
/// let axiom_expected_string = r#"["SubObjectPropertyOf",["ObjectPropertyChain","obo:IAO_0000120","obo:IAO_0000121"],"obo:IAO_0000122"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
///```
pub fn translate_property_chain(lhs: &owl::OWL, rhs: &owl::OWL) -> Value {
    let lhs_value = class_translation::translate(lhs);
    let rhs_value = class_translation::translate(rhs); // this is a list

    let chain = match rhs_value {
        Value::Array(mut arr) => {
            let mut chain_items = Vec::with_capacity(arr.len() + 1);
            chain_items.push(Value::String("ObjectPropertyChain".into()));
            chain_items.append(&mut arr);
            Value::Array(chain_items)
        }
        other => {
            Value::Array(vec![Value::String("ObjectPropertyChain".into()), other]) //TODO: error handling
        }
    };

    Value::Array(vec![
        Value::String("SubObjectPropertyOf".into()),
        chain,
        lhs_value,
    ])
}

/// Given the OWL expression rhs encoding a NegativePropertyAssertion
/// return the corresponding OFN S-expression.
///
/// #Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let blank_node = "\"_:genID123\"";
/// let assertion = r#"{"owl:assertionProperty":[{"datatype":"_IRI","object":"obo:IAO_0000120"}],"owl:sourceIndividual":[{"datatype":"_IRI","object":"obo:IAO_0000121"}],"owl:targetIndividual":[{"datatype":"_IRI","object":"obo:IAO_0000122"}]}"#;

///
/// let blank_node_owl : owl::OWL = serde_json::from_str(blank_node).unwrap();
/// let assertion_owl : owl::OWL = serde_json::from_str(assertion).unwrap();
///
/// let axiom : Value = translation::translate_negative_property_assertion(&blank_node_owl,
/// &assertion_owl);
/// let axiom_expected_string = r#"["NegativeObjectPropertyAssertion","obo:IAO_0000120","obo:IAO_0000121","obo:IAO_0000122"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
/// ```
pub fn translate_negative_property_assertion(_lhs: &owl::OWL, rhs: &owl::OWL) -> Value {
    //NB: this returns an axiom rather than an expression
    let axiom: Value = class_translation::translate(rhs);
    axiom
}

/// Given the OWL expressions lhs and rhs,
/// return the OFN S-expression ["HasKey", T(lhs), T(rhs)]
/// where T(lhs) and T(rhs) are lists of OFN S-expressions.
/// TODO: example
pub fn translate_has_key(lhs: &owl::OWL, rhs: &owl::OWL) -> Value {
    let class_value = class_translation::translate(lhs);
    let keys_value = class_translation::translate(rhs); // Expected to be a list

    let operator = Value::String("HasKey".into());

    match keys_value {
        Value::Array(mut arr) => {
            let mut axiom = Vec::with_capacity(arr.len() + 2);
            axiom.push(operator);
            axiom.push(class_value);
            // Move the array elements without extra cloning
            axiom.append(&mut arr);
            Value::Array(axiom)
        }
        other => Value::Array(vec![operator, class_value, other]),//TODO: error handling
    }
}

/// Given the OWL expressions lhs and rhs,
/// return the OFN S-expression ["Import", T(lhs), T(rhs)]
/// where T(lhs) and T(rhs) are lists of OFN S-expressions.
///
/// #Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
///
/// let lhs = "\"obo:IAO_0000120\"";
/// let rhs = "\"obo:IAO_0000122\"";
///
/// let lhs_owl : owl::OWL = serde_json::from_str(lhs).unwrap();
/// let rhs_owl : owl::OWL = serde_json::from_str(rhs).unwrap();
///
/// let axiom : Value = translation::translate_import(&lhs_owl, &rhs_owl);
/// let axiom_expected_string = r#"["Import","obo:IAO_0000120","obo:IAO_0000122"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
/// ```
pub fn translate_import(lhs: &owl::OWL, rhs: &owl::OWL) -> Value {
    let ontology: Value = class_translation::translate(lhs);
    let import: Value = class_translation::translate(rhs);

    let operator = Value::String(String::from("Import"));
    let mut res = vec![operator];
    res.push(ontology);
    res.push(import);

    Value::Array(res)
}

/// Given a subject s, a property p, and an object o,
/// return the OFN S-expression ["ThinTriple", T(s), T(p), T(o)]
/// where T(s), T(p), T(o) are OFN S-expressions.
///
/// #Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::axiom_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
///
/// let s = "\"obo:IAO_0000120\"";
/// let p = "\"obo:IAO_0000121\"";
/// let o = "\"obo:IAO_0000122\"";
///
/// let axiom : Value = translation::translate_thin_triple(&s, &p, &o);
/// let axiom_expected_string = r#"["ThinTriple","obo:IAO_0000120","obo:IAO_0000121","obo:IAO_0000122"]"#;
/// let axiom_expected : Value = serde_json::from_str(axiom_expected_string).unwrap();
///
/// assert_eq!(axiom, axiom_expected);
/// ```
fn parse_json_or_string(input: &str) -> Value {
    match serde_json::from_str(input) {
        Ok(val) => val,
        Err(_) => Value::String(input.to_owned()),
    }
}

pub fn translate_thin_triple(s: &str, p: &str, o: &str) -> Value {
    // NB: AnnotationAssertions are ambiguous and are translated as ThickTriples
    // T(Property Subject Object) rather than T(Subject Property Object)
    let subject = parse_json_or_string(s);
    let predicate = parse_json_or_string(p);
    let object = parse_json_or_string(o);

    Value::Array(vec![
        Value::String("ThinTriple".into()),
        subject,
        predicate,
        object,
    ])
}
