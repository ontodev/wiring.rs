use serde_json::{Value};
use crate::ofn2man::axiom_translation as axiom_translation; 
use crate::ofn2man::class_translation as class_translation; 
use crate::ofn2man::property_translation as property_translation;

pub fn parse_ofn(t: &str) -> String {
    //deserialise JSON as a (serde) Value
    let thick_triple: Value = serde_json::from_str(t).unwrap(); 
    //start the translation process
    let out = translate_triple(&thick_triple); 
    out 
}

pub fn translate_triple(v : &Value) -> String {

    match v[0].as_str() {
         Some("SubClassOf") => axiom_translation::translate_subclass_of_axiom(v),
         Some("DisjointClasses") => axiom_translation::translate_disjoint_classes_axiom(v),
         Some("DisjointUnionOf") => axiom_translation::translate_disjoint_union_of_axiom(v),
         Some("EquivalentClasses") => axiom_translation::translate_equivalent_classes_axiom(v),
         Some("ThinTriple") => axiom_translation::translate_thin_triple(v),

         Some("ObjectSomeValuesFrom") => class_translation::translate_some_values_from(v), 
         Some("ObjectAllValuesFrom") => class_translation::translate_all_values_from(v), 
         Some("ObjectHasValue") => class_translation::translate_has_value(v), 
         Some("ObjectMinCardinality") => class_translation::translate_min_cardinality(v), 
         Some("ObjectMinQualifiedCardinality") => class_translation::translate_min_qualified_cardinality(v), 
         Some("ObjectMaxCardinality") => class_translation::translate_max_cardinality(v), 
         Some("ObjectMaxQualifiedCardinality") => class_translation::translate_max_qualified_cardinality(v), 
         Some("ObjectExactCardinality") => class_translation::translate_exact_cardinality(v), 
         Some("ObjectExactQualifiedCardinality") => class_translation::translate_exact_qualified_cardinality(v), 
         Some("ObjectHasSelf") => class_translation::translate_has_self(v), 
         Some("ObjectIntersectionOf") => class_translation::translate_intersection_of(v), 
         Some("ObjectUnionOf") => class_translation::translate_union_of(v), 
         Some("ObjectOneOf") => class_translation::translate_one_of(v), 
         Some("ObjectComplementOf") => class_translation::translate_complement_of(v), 
         Some("DataSomeValuesFrom") => class_translation::translate_some_values_from(v), 
         Some("DataAllValuesFrom") => class_translation::translate_all_values_from(v), 
         Some("DataHasValue") => class_translation::translate_has_value(v), 
         Some("DataMinCardinality") => class_translation::translate_min_cardinality(v), 
         Some("DataMinQualifiedCardinality") => class_translation::translate_min_qualified_cardinality(v), 
         Some("DataMaxCardinality") => class_translation::translate_max_cardinality(v), 
         Some("DataMaxQualifiedCardinality") => class_translation::translate_max_qualified_cardinality(v), 
         Some("DataExactCardinality") => class_translation::translate_exact_cardinality(v), 
         Some("DataExactQualifiedCardinality") => class_translation::translate_exact_qualified_cardinality(v), 
         Some("DataHasSelf") => class_translation::translate_has_self(v), 
         Some("DataIntersectionOf") => class_translation::translate_intersection_of(v), 
         Some("DataUnionOf") => class_translation::translate_union_of(v), 
         Some("DataOneOf") => class_translation::translate_one_of(v), 
         Some("DataComplementOf") => class_translation::translate_complement_of(v), 

         Some("ObjectInverseOf") => property_translation::translate_inverse_of(v), 
         Some(_) => panic!(),
         None => panic!(),
     }
}
