# Wiring (with Rust)

**The code is in active development and subject to frequent changes.**

**Feedback and PRs are welcome! There is always more to do and things to improve.**

`wiring` reads OWL axioms serialised as *thick triples* and generates *OWL Functional Notation S-expressions* (OFN S-expression).
For example, the OWL axiom 
`obo:OBI_1110023 equivalentTo obo:RO_0000087 some obo:OBI_1110082` 
is encoded by the thick triple
```
{"object":{"owl:onProperty":[{"object":"obo:RO_0000087"}],"owl:someValuesFrom":[{"object":"obo:OBI_1110082"}],"rdf:type":[{"object":"owl:Restriction"}]},"predicate":"owl:equivalentClass","subject":"obo:OBI_1110023"} 
```
which can be translated with `wiring` into the OFN S-expression
`["EquivalentClasses","obo:OBI_1110023",["ObjectSomeValuesFrom","obo:RO_0000087","obo:OBI_1110082"]]`.

An OFN S-expression can be processed in `wiring`, e.g. by replacing named entities with their corresponding labels: `["EquivalentClasses","'immunogen@en'",["ObjectSomeValuesFrom","'has role@en'","obo:OBI_1110082"]]`. Such (processed) OFN S-expressions can be serialised into Manchester Syntax: `EquivalentClasses: 'immunogen@en', 'has role@en' some obo:OBI_1110082`.

## tl;dr

The `main.rs` file contains example code showcasing the basic functionality of `wiring` by translating thick triples into OFN S-expressions and serialising them in Manchester Syntax. 

## Things to do
- `wiring` currently only supports class expression axioms (excluding data types) and should be extended to all of OWL 


