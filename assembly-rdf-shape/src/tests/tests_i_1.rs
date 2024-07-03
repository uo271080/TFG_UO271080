#[cfg(test)]
mod tests_i_1 {
    use headless_chrome::Browser;
    use std::error::Error;

    const E_3: &str = r#"
PREFIX :       <http://example.org/>
PREFIX schema: <http://schema.org/>
PREFIX xsd:    <http://www.w3.org/2001/XMLSchema#>
PREFIX foaf:   <http://xmlns.com/foaf/0.1/>

:alice schema:name           "Alice" ;             :bob .
"#;

    const E_4: &str = r#"
prefix :       <http://example.org/>
prefix xsd:    <http://www.w3.org/2001/XMLSchema#>
prefix schema: <http://schema.org/>

:a  :name    "Title A" ;
    :related :b  .

:b  :related :a ;
    :name    "Title B".

:c  :name  "Title C1", "Title C2" .

:d  :name  234 .

:e  :namme  "Title E" .

:f  :name    "Title F" ;
    :related :a, _:1 .

_:1 :name  "Unknown title" .
"#;

    const E_5: &str = r#"
<http://example.org/e> <http://example.org/namme> "Title E" .
<http://example.org/c> <http://example.org/name> "Title C1" .
<http://example.org/c> <http://example.org/name> "Title C2" .
<http://example.org/a> <http://example.org/name> "Title A" .
<http://example.org/a> <http://example.org/related> <http://example.org/b> .
<http://example.org/f> <http://example.org/name> "Title F" .
<http://example.org/f> <http://example.org/related> <http://example.org/a> .
<http://example.org/f> <http://example.org/related> _:B1 .
<http://example.org/d> <http://example.org/name> "234"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:B1 <http://example.org/name> "Unknown title" .
<http://example.org/b> <http://example.org/related> <http://example.org/a> .
<http://example.org/b> <http://example.org/name> "Title B" .
"#;

    const E_6: &str = r#"
<http://example.org/e> <http://example.org/namme> "Title E" .
<http://example.org/c> <http://example.org/name> "Title C1" .
<http://example.org/c> <http://example.org/name> "Title C2" .
<http://example.org/a> <http://example.org/name> "Title A" .
<http://example.org/a> <http://example.org/related> <http://example.org/b> .
<http://example.org/f> <http://example.org/name> "Title F" .
<http://example.org/f> <http://example.org/related> <http://example.org/a> .
<http://example.org/f> <http://example.org/related> _:B1 .
<http://example.org/d> <http://example.org/name> "234"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:B1 <http://example.org/name> "Unknown title" .
<http://example.org/b> <http://example.org/related> <http://example.org/a> .
<http://example.org/b> <http://example.org/name> "Title B" .
"#;

    const E_7: &str = r#"
[{"@id":"_:b0","http://example.org/name":[{"@value":"Unknown title"}]},{"@id":"http://example.org/a","http://example.org/name":[{"@value":"Title A"}],"http://example.org/related":[{"@id":"http://example.org/b"}]},{"@id":"http://example.org/b","http://example.org/related":[{"@id":"http://example.org/a"}],"http://example.org/name":[{"@value":"Title B"}]},{"@id":"http://example.org/c","http://example.org/name":[{"@value":"Title C1"},{"@value":"Title C2"}]},{"@id":"http://example.org/d","http://example.org/name":[{"@value":234}]},{"@id":"http://example.org/e","http://example.org/namme":[{"@value":"Title E"}]},{"@id":"http://example.org/f","http://example.org/name":[{"@value":"Title F"}],"http://example.org/related":[{"@id":"http://example.org/a"},{"@id":"_:b0"}]}]
"#;

    const E_8: &str = r#"
<?xml version="1.0" encoding="utf-8" ?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:ns0="http://example.org/">

  <rdf:Description rdf:about="http://example.org/e">
    <ns0:namme>Title E</ns0:namme>
  </rdf:Description>

  <rdf:Description rdf:about="http://example.org/c">
    <ns0:name>Title C1</ns0:name>
    <ns0:name>Title C2</ns0:name>
  </rdf:Description>

  <rdf:Description rdf:about="http://example.org/a">
    <ns0:name>Title A</ns0:name>
    <ns0:related>
      <rdf:Description rdf:about="http://example.org/b">
        <ns0:related rdf:resource="http://example.org/a"/>
        <ns0:name>Title B</ns0:name>
      </rdf:Description>
    </ns0:related>

  </rdf:Description>

  <rdf:Description rdf:about="http://example.org/f">
    <ns0:name>Title F</ns0:name>
    <ns0:related rdf:resource="http://example.org/a"/>
    <ns0:related>
      <rdf:Description>
        <ns0:name>Unknown title</ns0:name>
      </rdf:Description>
    </ns0:related>

  </rdf:Description>

  <rdf:Description rdf:about="http://example.org/d">
    <ns0:name rdf:datatype="http://www.w3.org/2001/XMLSchema#integer">234</ns0:name>
  </rdf:Description>

</rdf:RDF>
"#;

    const E_9: &str = r#"
{ 
  "http://example.org/d" : { 
    "http://example.org/name" : [ { 
      "type" : "literal" ,
      "value" : "234" ,
      "datatype" : "http://www.w3.org/2001/XMLSchema#integer"
    }
     ]
  }
   ,
  "http://example.org/e" : { 
    "http://example.org/namme" : [ { 
      "type" : "literal" ,
      "value" : "Title E"
    }
     ]
  }
   ,
  "http://example.org/b" : { 
    "http://example.org/related" : [ { 
      "type" : "uri" ,
      "value" : "http://example.org/a"
    }
     ] ,
    "http://example.org/name" : [ { 
      "type" : "literal" ,
      "value" : "Title B"
    }
     ]
  }
   ,
  "http://example.org/c" : { 
    "http://example.org/name" : [ { 
      "type" : "literal" ,
      "value" : "Title C2"
    }
    , { 
      "type" : "literal" ,
      "value" : "Title C1"
    }
     ]
  }
   ,
  "http://example.org/f" : { 
    "http://example.org/related" : [ { 
      "type" : "uri" ,
      "value" : "http://example.org/a"
    }
    , { 
      "type" : "bnode" ,
      "value" : "_:1"
    }
     ] ,
    "http://example.org/name" : [ { 
      "type" : "literal" ,
      "value" : "Title F"
    }
     ]
  }
   ,
  "http://example.org/a" : { 
    "http://example.org/related" : [ { 
      "type" : "uri" ,
      "value" : "http://example.org/b"
    }
     ] ,
    "http://example.org/name" : [ { 
      "type" : "literal" ,
      "value" : "Title A"
    }
     ]
  }
   ,
  "_:1" : { 
    "http://example.org/name" : [ { 
      "type" : "literal" ,
      "value" : "Unknown title"
    }
     ]
  }
}
"#;

    #[test]
    fn i_1_1() -> Result<(), Box<dyn Error>> {
        println!("Running test: Analyze RDF invalid entry");

        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            &format!(
                r#"
            window.yateInstance.setValue(`{}`);
            "#,
                E_3.replace("`", "\\`")
            ),
            false,
        )?;

        let remote_object = tab.evaluate(
            r#"
            window.yateInstance.getValue();
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), E_3.trim())
            }
            _ => unreachable!(),
        };

        tab.evaluate(
            r#"
            $('#analyze-rdf').click();
            "#,
            false,
        )?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        let remote_object = tab.evaluate(
            r#"
            (function() {
                let modalBody = document.getElementById('modal-body');
                return modalBody ? modalBody.textContent : '';
            })()
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(
                    returned_value.as_str().unwrap_or("").trim(),
                    "Status: RDF is not well-formed."
                )
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    #[test]
    fn i_1_2() -> Result<(), Box<dyn Error>> {
        println!("Running test: Analyze Turtle RDF valid entry");

        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            &format!(
                r#"
            window.yateInstance.setValue(`{}`);
            "#,
                E_4.replace("`", "\\`")
            ),
            false,
        )?;

        let remote_object = tab.evaluate(
            r#"
            window.yateInstance.getValue();
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), E_4.trim())
            }
            _ => unreachable!(),
        };

        tab.evaluate(
            r#"
            $('#analyze-rdf').click();
            "#,
            false,
        )?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        let remote_object = tab.evaluate(
            r#"
            (function() {
                let modalBody = document.getElementById('modal-body');
                return modalBody ? modalBody.textContent : '';
            })()
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(
                    returned_value.as_str().unwrap_or("").trim(),
                    "Status: Well formed RDFNumber of statements: 12"
                )
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    #[test]
    fn i_1_3() -> Result<(), Box<dyn Error>> {
        println!("Running test: Analyze N-Triples RDF valid entry");

        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            &format!(
                r#"
            window.yateInstance.setValue(`{}`);
            "#,
                E_5.replace("`", "\\`")
            ),
            false,
        )?;

        let remote_object = tab.evaluate(
            r#"
            window.yateInstance.getValue();
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), E_5.trim())
            }
            _ => unreachable!(),
        };

        tab.evaluate(
            r#"
            document.getElementById('select-rdf').value = 'N-Triples';
            "#,
            false,
        )?;

        tab.evaluate(
            r#"
            $('#analyze-rdf').click();
            "#,
            false,
        )?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        let remote_object = tab.evaluate(
            r#"
            (function() {
                let modalBody = document.getElementById('modal-body');
                return modalBody ? modalBody.textContent : '';
            })()
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(
                    returned_value.as_str().unwrap_or("").trim(),
                    "Status: Well formed RDFNumber of statements: 12"
                )
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    #[test]
    fn i_1_4() -> Result<(), Box<dyn Error>> {
        println!("Running test: Analyze N-Quads RDF valid entry");
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            &format!(
                r#"
            window.yateInstance.setValue(`{}`);
            "#,
                E_6.replace("`", "\\`")
            ),
            false,
        )?;

        let remote_object = tab.evaluate(
            r#"
            window.yateInstance.getValue();
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), E_6.trim())
            }
            _ => unreachable!(),
        };

        tab.evaluate(
            r#"
            document.getElementById('select-rdf').value = 'N-Quads';
            "#,
            false,
        )?;

        tab.evaluate(
            r#"
            $('#analyze-rdf').click();
            "#,
            false,
        )?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        let remote_object = tab.evaluate(
            r#"
            (function() {
                let modalBody = document.getElementById('modal-body');
                return modalBody ? modalBody.textContent : '';
            })()
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(
                    returned_value.as_str().unwrap_or("").trim(),
                    "Status: Well formed RDFNumber of statements: 12"
                )
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    #[test]
    fn i_1_5() -> Result<(), Box<dyn Error>> {
        println!("Running test: Analyze JSON-LD RDF valid entry");
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            &format!(
                r#"
            window.yateInstance.setValue(`{}`);
            "#,
                E_7.replace("`", "\\`")
            ),
            false,
        )?;

        let remote_object = tab.evaluate(
            r#"
            window.yateInstance.getValue();
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), E_7.trim())
            }
            _ => unreachable!(),
        };

        tab.evaluate(
            r#"
            document.getElementById('select-rdf').value = 'JSON-LD';
            // Trigger change event
            var event = new Event('change', { bubbles: true });
            document.getElementById('select-rdf').dispatchEvent(event);
            "#,
            false,
        )?;
        tab.evaluate(
            r#"
            $('#analyze-rdf').click();
            "#,
            false,
        )?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        let remote_object = tab.evaluate(
            r#"
            (function() {
                let modalBody = document.getElementById('modal-body');
                return modalBody ? modalBody.textContent : '';
            })()
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(
                    returned_value.as_str().unwrap_or("").trim(),
                    "Status: Well formed RDFNumber of statements: 12"
                )
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    #[test]
    fn i_1_6() -> Result<(), Box<dyn Error>> {
        println!("Running test: Analyze RDF/XML RDF valid entry");
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            &format!(
                r#"
            window.yateInstance.setValue(`{}`);
            "#,
                E_8.replace("`", "\\`")
            ),
            false,
        )?;

        let remote_object = tab.evaluate(
            r#"
            window.yateInstance.getValue();
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), E_8.trim())
            }
            _ => unreachable!(),
        };

        tab.evaluate(
            r#"
            document.getElementById('select-rdf').value = 'RDF/XML';
            // Trigger change event
            var event = new Event('change', { bubbles: true });
            document.getElementById('select-rdf').dispatchEvent(event);
            "#,
            false,
        )?;
        tab.evaluate(
            r#"
            $('#analyze-rdf').click();
            "#,
            false,
        )?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        let remote_object = tab.evaluate(
            r#"
            (function() {
                let modalBody = document.getElementById('modal-body');
                return modalBody ? modalBody.textContent : '';
            })()
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(
                    returned_value.as_str().unwrap_or("").trim(),
                    "Status: Well formed RDFNumber of statements: 12"
                )
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    #[test]
    fn i_1_7() -> Result<(), Box<dyn Error>> {
        println!("Running test: Analyze RDF/JSON RDF valid entry");
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            &format!(
                r#"
            window.yateInstance.setValue(`{}`);
            "#,
                E_9.replace("`", "\\`")
            ),
            false,
        )?;

        let remote_object = tab.evaluate(
            r#"
            window.yateInstance.getValue();
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), E_9.trim())
            }
            _ => unreachable!(),
        };

        tab.evaluate(
            r#"
            document.getElementById('select-rdf').value = 'RDF/JSON';
            // Trigger change event
            var event = new Event('change', { bubbles: true });
            document.getElementById('select-rdf').dispatchEvent(event);
            "#,
            false,
        )?;
        tab.evaluate(
            r#"
            $('#analyze-rdf').click();
            "#,
            false,
        )?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        let remote_object = tab.evaluate(
            r#"
            (function() {
                let modalBody = document.getElementById('modal-body');
                return modalBody ? modalBody.textContent : '';
            })()
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(
                    returned_value.as_str().unwrap_or("").trim(),
                    "Status: Well formed RDFNumber of statements: 12"
                )
            }
            _ => unreachable!(),
        };

        Ok(())
    }
}
