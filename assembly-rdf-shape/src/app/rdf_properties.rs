// use oxrdf::{NamedNodeRef, vocab::rdf};
// use oxttl::TurtleParser;

// pub fn testOxttl(){
//     let file = b"@base <http://example.com/> .
//     @prefix schema: <http://schema.org/> .
//     <foo> a schema:Person ;
//         schema:name \"Foo\" .
//     <bar> a schema:Person ;
//         schema:name \"Bar\" .";
    
//     let schema_person = NamedNodeRef::new("http://schema.org/Person").unwrap();
//     let mut count = 0;
//     for triple in TurtleParser::new().parse_read(file.as_ref()) {
//         let triple = triple.unwrap();
//         if triple.predicate == rdf::TYPE && triple.object == schema_person.into() {
//             count += 1;
//         }
//     }
//     print!("Number of statements ===> "+count);
//     assert_eq!(2, count);
// }