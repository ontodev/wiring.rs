// ── RDF ──────────────────────────────────────────────────────────────────────
pub const RDF_TYPE: &str = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>";

// ── RDFS ─────────────────────────────────────────────────────────────────────
pub const RDFS_DATATYPE: &str = "<http://www.w3.org/2000/01/rdf-schema#Datatype>";
pub const RDFS_DOMAIN: &str = "<http://www.w3.org/2000/01/rdf-schema#domain>";
pub const RDFS_RANGE: &str = "<http://www.w3.org/2000/01/rdf-schema#range>";
pub const RDFS_SUB_CLASS_OF: &str = "<http://www.w3.org/2000/01/rdf-schema#subClassOf>";
pub const RDFS_SUB_PROPERTY_OF: &str = "<http://www.w3.org/2000/01/rdf-schema#subPropertyOf>";

// ── XSD ──────────────────────────────────────────────────────────────────────
pub const XSD_BOOLEAN: &str = "<http://www.w3.org/2001/XMLSchema#boolean>";
pub const XSD_BYTE: &str = "<http://www.w3.org/2001/XMLSchema#byte>";
pub const XSD_DECIMAL: &str = "<http://www.w3.org/2001/XMLSchema#decimal>";
pub const XSD_INT: &str = "<http://www.w3.org/2001/XMLSchema#int>";
pub const XSD_INTEGER: &str = "<http://www.w3.org/2001/XMLSchema#integer>";
pub const XSD_LONG: &str = "<http://www.w3.org/2001/XMLSchema#long>";
pub const XSD_NEGATIVE_INTEGER: &str = "<http://www.w3.org/2001/XMLSchema#negativeInteger>";
pub const XSD_NON_NEGATIVE_INTEGER: &str = "<http://www.w3.org/2001/XMLSchema#nonNegativeInteger>";
pub const XSD_NON_POSITIVE_INTEGER: &str = "<http://www.w3.org/2001/XMLSchema#nonPositiveInteger>";
pub const XSD_POSITIVE_INTEGER: &str = "<http://www.w3.org/2001/XMLSchema#positiveInteger>";
pub const XSD_SHORT: &str = "<http://www.w3.org/2001/XMLSchema#short>";
pub const XSD_STRING: &str = "<http://www.w3.org/2001/XMLSchema#string>";
pub const XSD_UNSIGNED_BYTE: &str = "<http://www.w3.org/2001/XMLSchema#unsignedByte>";
pub const XSD_UNSIGNED_INT: &str = "<http://www.w3.org/2001/XMLSchema#unsignedInt>";
pub const XSD_UNSIGNED_LONG: &str = "<http://www.w3.org/2001/XMLSchema#unsignedLong>";
pub const XSD_UNSIGNED_SHORT: &str = "<http://www.w3.org/2001/XMLSchema#unsignedShort>";

// ── OWL classes / types ──────────────────────────────────────────────────────
pub const OWL_AXIOM: &str = "<http://www.w3.org/2002/07/owl#Axiom>";
pub const OWL_ALL_DIFFERENT: &str = "<http://www.w3.org/2002/07/owl#AllDifferent>";
pub const OWL_ALL_DISJOINT_CLASSES: &str = "<http://www.w3.org/2002/07/owl#AllDisjointClasses>";
pub const OWL_ALL_DISJOINT_PROPERTIES: &str = "<http://www.w3.org/2002/07/owl#AllDisjointProperties>";
pub const OWL_ALL_SAME_AS: &str = "<http://www.w3.org/2002/07/owl#AllSameAs>";
pub const OWL_ANNOTATION_PROPERTY: &str = "<http://www.w3.org/2002/07/owl#AnnotationProperty>";
pub const OWL_ASYMMETRIC_PROPERTY: &str = "<http://www.w3.org/2002/07/owl#AsymmetricProperty>";
pub const OWL_CLASS: &str = "<http://www.w3.org/2002/07/owl#Class>";
pub const OWL_DATATYPE_PROPERTY: &str = "<http://www.w3.org/2002/07/owl#DatatypeProperty>";
pub const OWL_FUNCTIONAL_PROPERTY: &str = "<http://www.w3.org/2002/07/owl#FunctionalProperty>";
pub const OWL_INVERSE_FUNCTIONAL_PROPERTY: &str = "<http://www.w3.org/2002/07/owl#InverseFunctionalProperty>";
pub const OWL_IRREFLEXIVE_PROPERTY: &str = "<http://www.w3.org/2002/07/owl#IrreflexiveProperty>";
pub const OWL_NAMED_INDIVIDUAL: &str = "<http://www.w3.org/2002/07/owl#NamedIndividual>";
pub const OWL_NEGATIVE_PROPERTY_ASSERTION: &str = "<http://www.w3.org/2002/07/owl#NegativePropertyAssertion>";
pub const OWL_OBJECT_PROPERTY: &str = "<http://www.w3.org/2002/07/owl#ObjectProperty>";
pub const OWL_REAL: &str = "<http://www.w3.org/2002/07/owl#real>";
pub const OWL_RATIONAL: &str = "<http://www.w3.org/2002/07/owl#rational>";
pub const OWL_REFLECTIVE_PROPERTY: &str = "<http://www.w3.org/2002/07/owl#ReflexiveProperty>";
pub const OWL_RESTRICTION: &str = "<http://www.w3.org/2002/07/owl#Restriction>";
pub const OWL_SYMMETRIC_PROPERTY: &str = "<http://www.w3.org/2002/07/owl#SymmetricProperty>";
pub const OWL_TRANSITIVE_PROPERTY: &str = "<http://www.w3.org/2002/07/owl#TransitiveProperty>";

// ── OWL properties ───────────────────────────────────────────────────────────
pub const OWL_ALL_VALUES_FROM: &str = "<http://www.w3.org/2002/07/owl#allValuesFrom>";
pub const OWL_ASSERTION_PROPERTY: &str = "<http://www.w3.org/2002/07/owl#assertionProperty>";
pub const OWL_CARDINALITY: &str = "<http://www.w3.org/2002/07/owl#cardinality>";
pub const OWL_COMPLEMENT_OF: &str = "<http://www.w3.org/2002/07/owl#complementOf>";
pub const OWL_DIFFERENT_FROM: &str = "<http://www.w3.org/2002/07/owl#differentFrom>";
pub const OWL_DISJOINT_UNION_OF: &str = "<http://www.w3.org/2002/07/owl#disjointUnionOf>";
pub const OWL_DISJOINT_WITH: &str = "<http://www.w3.org/2002/07/owl#disjointWith>";
pub const OWL_DISTINCT_MEMBERS: &str = "<http://www.w3.org/2002/07/owl#distinctMembers>";
pub const OWL_EQUIVALENT_CLASS: &str = "<http://www.w3.org/2002/07/owl#equivalentClass>";
pub const OWL_EQUIVALENT_PROPERTY: &str = "<http://www.w3.org/2002/07/owl#equivalentProperty>";
pub const OWL_HAS_KEY: &str = "<http://www.w3.org/2002/07/owl#hasKey>";
pub const OWL_HAS_SELF: &str = "<http://www.w3.org/2002/07/owl#hasSelf>";
pub const OWL_HAS_VALUE: &str = "<http://www.w3.org/2002/07/owl#hasValue>";
pub const OWL_IMPORTS: &str = "<http://www.w3.org/2002/07/owl#imports>";
pub const OWL_INTERSECTION_OF: &str = "<http://www.w3.org/2002/07/owl#intersectionOf>";
pub const OWL_INVERSE_OF: &str = "<http://www.w3.org/2002/07/owl#inverseOf>";
pub const OWL_MAX_CARDINALITY: &str = "<http://www.w3.org/2002/07/owl#maxCardinality>";
pub const OWL_MAX_QUALIFIED_CARDINALITY: &str = "<http://www.w3.org/2002/07/owl#maxQualifiedCardinality>";
pub const OWL_MEMBERS: &str = "<http://www.w3.org/2002/07/owl#members>";
pub const OWL_MIN_CARDINALITY: &str = "<http://www.w3.org/2002/07/owl#minCardinality>";
pub const OWL_MIN_QUALIFIED_CARDINALITY: &str = "<http://www.w3.org/2002/07/owl#minQualifiedCardinality>";
pub const OWL_ON_CLASS: &str = "<http://www.w3.org/2002/07/owl#onClass>";
pub const OWL_ON_DATA_RANGE: &str = "<http://www.w3.org/2002/07/owl#onDataRange>";
pub const OWL_ON_PROPERTY: &str = "<http://www.w3.org/2002/07/owl#onProperty>";
pub const OWL_ONE_OF: &str = "<http://www.w3.org/2002/07/owl#oneOf>";
pub const OWL_PROPERTY_CHAIN_AXIOM: &str = "<http://www.w3.org/2002/07/owl#propertyChainAxiom>";
pub const OWL_PROPERTY_DISJOINT_WITH: &str = "<http://www.w3.org/2002/07/owl#propertyDisjointWith>";
pub const OWL_QUALIFIED_CARDINALITY: &str = "<http://www.w3.org/2002/07/owl#qualifiedCardinality>";
pub const OWL_SAME_AS: &str = "<http://www.w3.org/2002/07/owl#sameAs>";
pub const OWL_SOME_VALUES_FROM: &str = "<http://www.w3.org/2002/07/owl#someValuesFrom>";
pub const OWL_SOURCE_INDIVIDUAL: &str = "<http://www.w3.org/2002/07/owl#sourceIndividual>";
pub const OWL_TARGET_INDIVIDUAL: &str = "<http://www.w3.org/2002/07/owl#targetIndividual>";
pub const OWL_TARGET_VALUE: &str = "<http://www.w3.org/2002/07/owl#targetValue>";
pub const OWL_UNION_OF: &str = "<http://www.w3.org/2002/07/owl#unionOf>";
pub const OWL_VERSION_IRI: &str = "<http://www.w3.org/2002/07/owl#versionIRI>";

// ── SWRL ─────────────────────────────────────────────────────────────────────
pub const SWRL_ARGUMENT1: &str = "<http://www.w3.org/2003/11/swrl#argument1>";
pub const SWRL_ARGUMENT2: &str = "<http://www.w3.org/2003/11/swrl#argument2>";
pub const SWRL_ARGUMENTS: &str = "<http://www.w3.org/2003/11/swrl#arguments>";
pub const SWRL_BODY: &str = "<http://www.w3.org/2003/11/swrl#body>";
pub const SWRL_BUILTIN: &str = "<http://www.w3.org/2003/11/swrl#builtin>";
pub const SWRL_BUILTIN_ATOM: &str = "<http://www.w3.org/2003/11/swrl#BuiltinAtom>";
pub const SWRL_CLASS_ATOM: &str = "<http://www.w3.org/2003/11/swrl#ClassAtom>";
pub const SWRL_CLASS_PREDICATE: &str = "<http://www.w3.org/2003/11/swrl#classPredicate>";
pub const SWRL_DATA_RANGE: &str = "<http://www.w3.org/2003/11/swrl#dataRange>";
pub const SWRL_DATA_RANGE_ATOM: &str = "<http://www.w3.org/2003/11/swrl#DataRangeAtom>";
pub const SWRL_DATAVALUED_PROPERTY_ATOM: &str = "<http://www.w3.org/2003/11/swrl#DatavaluedPropertyAtom>";
pub const SWRL_DIFFERENT_INDIVIDUALS_ATOM: &str = "<http://www.w3.org/2003/11/swrl#DifferentIndividualsAtom>";
pub const SWRL_HEAD: &str = "<http://www.w3.org/2003/11/swrl#head>";
pub const SWRL_IMP: &str = "<http://www.w3.org/2003/11/swrl#Imp>";
pub const SWRL_INDIVIDUAL_PROPERTY_ATOM: &str = "<http://www.w3.org/2003/11/swrl#IndividualPropertyAtom>";
pub const SWRL_PROPERTY_PREDICATE: &str = "<http://www.w3.org/2003/11/swrl#propertyPredicate>";
pub const SWRL_SAME_INDIVIDUAL_ATOM: &str = "<http://www.w3.org/2003/11/swrl#SameIndividualAtom>";
