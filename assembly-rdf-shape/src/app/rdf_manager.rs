use std::convert::TryInto;

use log::info;
use oxrdf::{vocab::rdf, NamedNodeRef};
use oxttl::TurtleParser;

pub fn get_rdf_properties(input: String) -> i32 {
    let file = b"@base <http://example.org/>
@prefix schema: <http://schema.org/>
@prefix xsd:    <http://www.w3.org/2001/XMLSchema#>
@prefix foaf:   <http://xmlns.com/foaf/0.1/>

:alice schema:name           \"Alice\" .;            
       schema:gender         schema:Female .;
       schema:knows          :bob .
:carol schema:name           \"Carol\" ;        
       schema:gender         \"unspecified\" .;
       foaf:name             \"Carol\" .";

    // let file: &[u8; 608] = rdf_template
    //     .as_bytes()
    //     .try_into()
    //     .expect("Incorrect length");
    let schema_person = NamedNodeRef::new("http://schema.org/Person").unwrap();
    let mut count = 0;
    for triple in TurtleParser::new().parse_read(file.as_ref()) {
        let triple = triple.unwrap();
        if triple.predicate == rdf::TYPE && triple.object == schema_person.into() {
            count += 1;
        }
    }
    info!("llegamoos a analyzer");
    info!("{}", count);
    count
}
