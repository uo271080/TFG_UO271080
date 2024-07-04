#[cfg(test)]
mod tests_i_4 {
    use headless_chrome::Browser;
    use std::{error::Error, time::Duration};

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

    const E_7: &str = r#"
    [{"@id":"_:b0","http://example.org/name":[{"@value":"Unknown title"}]},{"@id":"http://example.org/a","http://example.org/name":[{"@value":"Title A"}],"http://example.org/related":[{"@id":"http://example.org/b"}]},{"@id":"http://example.org/b","http://example.org/related":[{"@id":"http://example.org/a"}],"http://example.org/name":[{"@value":"Title B"}]},{"@id":"http://example.org/c","http://example.org/name":[{"@value":"Title C1"},{"@value":"Title C2"}]},{"@id":"http://example.org/d","http://example.org/name":[{"@value":234}]},{"@id":"http://example.org/e","http://example.org/namme":[{"@value":"Title E"}]},{"@id":"http://example.org/f","http://example.org/name":[{"@value":"Title F"}],"http://example.org/related":[{"@id":"http://example.org/a"},{"@id":"_:b0"}]}]
    "#;

    const E_13: &str = r#"
prefix :       <http://example.org/>
prefix xsd:    <http://www.w3.org/2001/XMLSchema#>
prefix schema: <http://schema.org/>

:Book IRIing   ;
 :related      @:Book     *
}
"#;

    const E_14: &str = r#"
PREFIX : <http://example.org/>
PREFIX schema: <http://schema.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:User {
  schema:name xsd:string ;
  schema:birthDate xsd:date? ;
  schema:gender [ schema:Male schema:Female ] OR xsd:string ;
  schema:knows IRI @:User*
}
"#;

    const E_15: &str = r#"
{
  "type" : "Schema",
  "@context" : "http://www.w3.org/ns/shex.jsonld",
  "shapes" : [
    {
      "type" : "ShapeDecl",
      "id" : "http://example.org/User",
      "shapeExpr" : {
        "type" : "Shape",
        "expression" : {
          "type" : "EachOf",
          "expressions" : [
            {
              "type" : "TripleConstraint",
              "predicate" : "http://schema.org/name",
              "valueExpr" : {
                "type" : "NodeConstraint",
                "datatype" : "http://www.w3.org/2001/XMLSchema#string"
              }
            },
            {
              "predicate" : "http://schema.org/birthDate",
              "valueExpr" : {
                "type" : "NodeConstraint",
                "datatype" : "http://www.w3.org/2001/XMLSchema#date"
              },
              "min" : 0,
              "max" : 1,
              "type" : "TripleConstraint"
            },
            {
              "type" : "TripleConstraint",
              "predicate" : "http://schema.org/gender",
              "valueExpr" : {
                "type" : "ShapeOr",
                "shapeExprs" : [
                  {
                    "type" : "NodeConstraint",
                    "values" : [
                      "http://schema.org/Male",
                      "http://schema.org/Female"
                    ]
                  },
                  {
                    "type" : "NodeConstraint",
                    "datatype" : "http://www.w3.org/2001/XMLSchema#string"
                  }
                ]
              }
            },
            {
              "predicate" : "http://schema.org/knows",
              "valueExpr" : {
                "type" : "ShapeAnd",
                "shapeExprs" : [
                  {
                    "type" : "NodeConstraint",
                    "nodeKind" : "iri"
                  },
                  "http://example.org/User"
                ]
              },
              "min" : 0,
              "max" : -1,
              "type" : "TripleConstraint"
            }
          ]
        }
      }
    }
  ]
}
"#;

    const E_16: &str = r#"
{
    "@graph": [
        {
            "@id": "http://example.org/emily",
            "schema:name": "Emily",
            "schema:alternateName": "Emilee",
            "schema:gender": {
                "@id": "schema:Female"
            }
        },
        {
            "@id": "http://example.org/frank",
            "schema:name": "Frank",
            "schema:gender": {
                "@id": "schema:Male"
            }
        },
        {
            "@id": "http://example.org/bob",
            "schema:gender": {
                "@id": "schema:Male"
            },
            "schema:name": "Robert",
            "schema:birthDate": {
                "@value": "1980-03-10",
                "@type": "xsd:date"
            }
        },
        {
            "@id": "http://example.org/carol",
            "schema:name": "Carol",
            "schema:gender": "unspecified",
            "foaf:name": "Carol"
        },
        {
            "@id": "http://example.org/grace",
            "schema:name": "Grace",
            "schema:gender": {
                "@id": "schema:Male"
            },
            "schema:knows": {
                "@id": "http://example.org/bob"
            }
        },
        {
            "@id": "http://example.org/dave",
            "schema:name": "Dave",
            "schema:gender": "XYY",
            "schema:birthDate": {
                "@value": "1980-01-01",
                "@type": "xsd:date"
            }
        },
        {
            "@id": "http://example.org/harold",
            "schema:name": "Harold",
            "schema:gender": {
                "@id": "schema:Male"
            },
            "schema:knows": {
                "@id": "http://example.org/grace"
            }
        },
        {
            "@id": "http://example.org/alice",
            "schema:name": "Alice",
            "schema:gender": {
                "@id": "schema:Female"
            },
            "schema:knows": {
                "@id": "http://example.org/bob"
            }
        }
    ],
    "@context": {
        "schema": "http://schema.org/",
        "xsd": "http://www.w3.org/2001/XMLSchema#",
        "foaf": "http://xmlns.com/foaf/0.1/",
        "@vocab": "http://example.org/"
    }
}"#;

    const E_17: &str = r#"
<rdf:RDF
    xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
    xmlns="http://example.org/"
    xmlns:schema="http://schema.org/"
    xmlns:foaf="http://xmlns.com/foaf/0.1/"
    xmlns:xsd="http://www.w3.org/2001/XMLSchema#">
  <rdf:Description rdf:about="http://example.org/harold">
    <schema:name>Harold</schema:name>
    <schema:gender rdf:resource="http://schema.org/Male"/>
    <schema:knows>
      <rdf:Description rdf:about="http://example.org/grace">
        <schema:name>Grace</schema:name>
        <schema:gender rdf:resource="http://schema.org/Male"/>
        <schema:knows>
          <rdf:Description rdf:about="http://example.org/bob">
            <schema:gender rdf:resource="http://schema.org/Male"/>
            <schema:name>Robert</schema:name>
            <schema:birthDate rdf:datatype="http://www.w3.org/2001/XMLSchema#date"
            >1980-03-10</schema:birthDate>
          </rdf:Description>
        </schema:knows>
      </rdf:Description>
    </schema:knows>
  </rdf:Description>
  <rdf:Description rdf:about="http://example.org/dave">
    <schema:name>Dave</schema:name>
    <schema:gender>XYY</schema:gender>
    <schema:birthDate rdf:datatype="http://www.w3.org/2001/XMLSchema#date"
    >1980-01-01</schema:birthDate>
  </rdf:Description>
  <rdf:Description rdf:about="http://example.org/alice">
    <schema:name>Alice</schema:name>
    <schema:gender rdf:resource="http://schema.org/Female"/>
    <schema:knows rdf:resource="http://example.org/bob"/>
  </rdf:Description>
  <rdf:Description rdf:about="http://example.org/frank">
    <schema:name>Frank</schema:name>
    <schema:gender rdf:resource="http://schema.org/Male"/>
  </rdf:Description>
  <rdf:Description rdf:about="http://example.org/carol">
    <schema:name>Carol</schema:name>
    <schema:gender>unspecified</schema:gender>
    <foaf:name>Carol</foaf:name>
  </rdf:Description>
  <rdf:Description rdf:about="http://example.org/emily">
    <schema:name>Emily</schema:name>
    <schema:alternateName>Emilee</schema:alternateName>
    <schema:gender rdf:resource="http://schema.org/Female"/>
  </rdf:Description>
</rdf:RDF>
    "#;

    #[test]
    fn i_4_1() -> Result<(), Box<dyn Error>> {
        println!("Running test: Validation with invalid entries");

        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            &format!(
                r#"
            window.yasheInstance.setValue(`invalid shex entry`);
            "#
            ),
            false,
        )?;

        tab.evaluate(
            &format!(
                r#"
            window.yateInstance.setValue(`invalid rdf entry`);
            "#
            ),
            false,
        )?;

        // Cambiar el contenido del editor
        tab.evaluate(
            r#"
                document.querySelector('#shapemap-editor').value = 'invalid shapemap entry';
            "#,
            false,
        )?;

        tab.evaluate(
            r#"
            $('#validate-btn').click();
            "#,
            false,
        )?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        tab.wait_for_element("#alert-modal")?;

        Ok(())
    }

    #[test]
    fn i_4_2() -> Result<(), Box<dyn Error>> {
        println!("Running test: Validation with invalid entries");

        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            r#"
                    document.querySelectorAll('#example-1')[0].click();
                    "#,
            false,
        )?;

        std::thread::sleep(Duration::from_secs(5));

        tab.evaluate(
            r#"
            $('#validate-btn').click();
            "#,
            false,
        )?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        tab.evaluate(
            r#"
            $('#show-all').click();
            "#,
            false,
        )?;

        let num_rows = tab.evaluate(
            r#"
            document.querySelectorAll('#result-table tr').length - 1;
            "#,
            false,
        )?;

        let all_status_valid = tab.evaluate(
            r#"
            Array.from(document.querySelectorAll('#result-table tr')).slice(1).every(row => {
                const statusCell = row.querySelector('.details-row');
                return statusCell && statusCell.textContent.trim() === 'Valid';
            });
            "#,
            false,
        )?;

        // Convertir los resultados a tipos manejables en Rust
        let num_entries = num_rows.value.unwrap().as_i64().unwrap();
        let all_status_valid = all_status_valid.value.unwrap().as_bool().unwrap();

        assert_eq!(num_entries, 7, "The table should have 7 entries.");
        assert_eq!(
            all_status_valid, true,
            "All status columns should be 'Valid'."
        );

        Ok(())
    }

    #[test]
    fn i_4_3() -> Result<(), Box<dyn Error>> {
        println!("Running test: Validation with invalid entries");

        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            r#"
                    document.querySelectorAll('#example-1')[0].click();
                    "#,
            false,
        )?;

        std::thread::sleep(Duration::from_secs(5));

        tab.evaluate(
            &format!(
                r#"
            window.yateInstance.setValue(`{}`);
            "#,
                E_16.replace("`", "\\`")
            ),
            false,
        )?;

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
            $('#validate-btn').click();
            "#,
            false,
        )?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        tab.evaluate(
            r#"
            $('#show-all').click();
            "#,
            false,
        )?;

        let num_rows = tab.evaluate(
            r#"
            document.querySelectorAll('#result-table tr').length - 1;
            "#,
            false,
        )?;

        let all_status_valid = tab.evaluate(
            r#"
            Array.from(document.querySelectorAll('#result-table tr')).slice(1).every(row => {
                const statusCell = row.querySelector('.details-row');
                return statusCell && statusCell.textContent.trim() === 'Valid';
            });
            "#,
            false,
        )?;

        // Convertir los resultados a tipos manejables en Rust
        let num_entries = num_rows.value.unwrap().as_i64().unwrap();
        let all_status_valid = all_status_valid.value.unwrap().as_bool().unwrap();

        assert_eq!(num_entries, 7, "The table should have 7 entries.");
        assert_eq!(
            all_status_valid, true,
            "All status columns should be 'Valid'."
        );

        Ok(())
    }

    #[test]
    fn i_4_4() -> Result<(), Box<dyn Error>> {
        println!("Running test: Validation with invalid entries");

        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            r#"
                    document.querySelectorAll('#example-1')[0].click();
                    "#,
            false,
        )?;

        std::thread::sleep(Duration::from_secs(5));

        tab.evaluate(
            &format!(
                r#"
            window.yasheInstance.setValue(`{}`);
            "#,
                E_15.replace("`", "\\`")
            ),
            false,
        )?;

        tab.evaluate(
            r#"
            document.getElementById('select-shex').value = 'ShExJ';
            var event = new Event('change', { bubbles: true });
            document.getElementById('select-shex').dispatchEvent(event);
            "#,
            false,
        )?;

        tab.evaluate(
            r#"
            $('#validate-btn').click();
            "#,
            false,
        )?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        tab.evaluate(
            r#"
            $('#show-all').click();
            "#,
            false,
        )?;

        let num_rows = tab.evaluate(
            r#"
            document.querySelectorAll('#result-table tr').length - 1;
            "#,
            false,
        )?;

        let all_status_valid = tab.evaluate(
            r#"
            Array.from(document.querySelectorAll('#result-table tr')).slice(1).every(row => {
                const statusCell = row.querySelector('.details-row');
                return statusCell && statusCell.textContent.trim() === 'Valid';
            });
            "#,
            false,
        )?;

        // Convertir los resultados a tipos manejables en Rust
        let num_entries = num_rows.value.unwrap().as_i64().unwrap();
        let all_status_valid = all_status_valid.value.unwrap().as_bool().unwrap();

        assert_eq!(num_entries, 7, "The table should have 7 entries.");
        assert_eq!(
            all_status_valid, true,
            "All status columns should be 'Valid'."
        );

        Ok(())
    }

    #[test]
    fn i_4_5() -> Result<(), Box<dyn Error>> {
        println!("Running test: Validation with invalid entries");

        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            r#"
                    document.querySelectorAll('#example-1')[0].click();
                    "#,
            false,
        )?;

        std::thread::sleep(Duration::from_secs(5));

        tab.evaluate(
            &format!(
                r#"
            window.yasheInstance.setValue(`{}`);
            "#,
                E_15.replace("`", "\\`")
            ),
            false,
        )?;

        tab.evaluate(
            r#"
            document.getElementById('select-shex').value = 'ShExJ';
            var event = new Event('change', { bubbles: true });
            document.getElementById('select-shex').dispatchEvent(event);
            "#,
            false,
        )?;

        tab.evaluate(
            r#"
            $('#validate-btn').click();
            "#,
            false,
        )?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        tab.evaluate(
            r#"
            $('#show-all').click();
            "#,
            false,
        )?;

        let num_rows = tab.evaluate(
            r#"
            document.querySelectorAll('#result-table tr').length - 1;
            "#,
            false,
        )?;

        let all_status_valid = tab.evaluate(
            r#"
            Array.from(document.querySelectorAll('#result-table tr')).slice(1).every(row => {
                const statusCell = row.querySelector('.details-row');
                return statusCell && statusCell.textContent.trim() === 'Valid';
            });
            "#,
            false,
        )?;

        // Convertir los resultados a tipos manejables en Rust
        let num_entries = num_rows.value.unwrap().as_i64().unwrap();
        let all_status_valid = all_status_valid.value.unwrap().as_bool().unwrap();

        assert_eq!(num_entries, 7, "The table should have 7 entries.");
        assert_eq!(
            all_status_valid, true,
            "All status columns should be 'Valid'."
        );

        Ok(())
    }

    #[test]
    fn i_4_6() -> Result<(), Box<dyn Error>> {
        println!("Running test: Validation with RDF/XML, ShExC and Compact entries.");

        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        std::thread::sleep(Duration::from_secs(5));

        tab.evaluate(
            r#"
                    document.querySelectorAll('#example-1')[0].click();
                    "#,
            false,
        )?;

        tab.evaluate(
            &format!(
                r#"
            window.yateInstance.setValue(`{}`);
            "#,
                E_17.replace("`", "\\`")
            ),
            false,
        )?;

        tab.evaluate(
            r#"
            document.getElementById('select-rdf').value = 'RDF/XML';
            var event = new Event('change', { bubbles: true });
            document.getElementById('select-rdf').dispatchEvent(event);
            "#,
            false,
        )?;

        tab.evaluate(
            r#"
            $('#validate-btn').click();
            "#,
            false,
        )?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        tab.evaluate(
            r#"
            $('#show-all').click();
            "#,
            false,
        )?;

        let num_rows = tab.evaluate(
            r#"
            document.querySelectorAll('#result-table tr').length - 1;
            "#,
            false,
        )?;

        let all_status_valid = tab.evaluate(
            r#"
            Array.from(document.querySelectorAll('#result-table tr')).slice(1).every(row => {
                const statusCell = row.querySelector('.details-row');
                return statusCell && statusCell.textContent.trim() === 'Valid';
            });
            "#,
            false,
        )?;

        // Convertir los resultados a tipos manejables en Rust
        let num_entries = num_rows.value.unwrap().as_i64().unwrap();
        let all_status_valid = all_status_valid.value.unwrap().as_bool().unwrap();

        assert_eq!(num_entries, 7, "The table should have 7 entries.");
        assert_eq!(
            all_status_valid, true,
            "All status columns should be 'Valid'."
        );

        Ok(())
    }

    #[test]
    fn i_4_7() -> Result<(), Box<dyn Error>> {
        println!("Running test: Validation with RDF/XML, ShExC and Compact entries.");

        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            r#"
                    document.querySelectorAll('#example-3')[0].click();
                    "#,
            false,
        )?;

        std::thread::sleep(Duration::from_secs(5));

        tab.evaluate(
            &format!(
                r#"
            window.yateInstance.setValue(`{}`);
            "#,
                E_7.replace("`", "\\`")
            ),
            false,
        )?;

        tab.evaluate(
            r#"
            document.getElementById('select-rdf').value = 'JSON-LD';
            var event = new Event('change', { bubbles: true });
            document.getElementById('select-rdf').dispatchEvent(event);
            "#,
            false,
        )?;

        tab.evaluate(
            r#"
            $('#validate-btn').click();
            "#,
            false,
        )?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        tab.evaluate(
            r#"
            $('#show-all').click();
            "#,
            false,
        )?;

        let num_rows = tab.evaluate(
            r#"
            document.querySelectorAll('#result-table tr').length - 1;
            "#,
            false,
        )?;

        let all_status_valid = tab.evaluate(
            r#"
            Array.from(document.querySelectorAll('#result-table tr')).slice(1).every(row => {
                const statusCell = row.querySelector('.details-row');
                return statusCell && statusCell.textContent.trim() === 'Valid';
            });
            "#,
            false,
        )?;

        let num_valid_rows = tab.evaluate(
            r#"
            Array.from(document.querySelectorAll('#result-table tr')).slice(1).filter(row => {
                const statusCell = row.querySelector('.details-row');
                return statusCell && statusCell.textContent.trim() === 'Valid';
            }).length;
            "#,
            false,
        )?;

        // Convertir los resultados a tipos manejables en Rust
        let num_entries = num_rows.value.unwrap().as_i64().unwrap();
        let all_status_valid = all_status_valid.value.unwrap().as_bool().unwrap();
        let num_valid_entries = num_valid_rows.value.unwrap().as_i64().unwrap();

        assert_eq!(num_entries, 6, "The table should have 6 entries.");
        assert_eq!(
            all_status_valid, false,
            "All status columns should be 'Valid'."
        );
        assert_eq!(
            num_valid_entries, 2,
            "All status columns should be 'Valid'."
        );
        Ok(())
    }

    #[test]
    fn i_4_8() -> Result<(), Box<dyn Error>> {
        println!("Running test: Validation with RDF/XML, ShExC and Compact entries.");

        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            r#"
                    document.querySelectorAll('#example-3')[0].click();
                    "#,
            false,
        )?;

        std::thread::sleep(Duration::from_secs(5));

        tab.evaluate(
            &format!(
                r#"
            window.yateInstance.setValue(`{}`);
            "#,
                E_5.replace("`", "\\`")
            ),
            false,
        )?;

        tab.evaluate(
            r#"
            document.getElementById('select-rdf').value = 'N-Triples';
            var event = new Event('change', { bubbles: true });
            document.getElementById('select-rdf').dispatchEvent(event);
            "#,
            false,
        )?;

        tab.evaluate(
            r#"
            $('#validate-btn').click();
            "#,
            false,
        )?;

        tab.evaluate(
            r#"
            $('#show-all').click();
            "#,
            false,
        )?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        tab.evaluate(
            r#"
            $('#show-all').click();
            "#,
            false,
        )?;

        let num_rows = tab.evaluate(
            r#"
            document.querySelectorAll('#result-table tr').length - 1;
            "#,
            false,
        )?;

        let all_status_valid = tab.evaluate(
            r#"
            Array.from(document.querySelectorAll('#result-table tr')).slice(1).every(row => {
                const statusCell = row.querySelector('.details-row');
                return statusCell && statusCell.textContent.trim() === 'Valid';
            });
            "#,
            false,
        )?;

        let num_valid_rows = tab.evaluate(
            r#"
            Array.from(document.querySelectorAll('#result-table tr')).slice(1).filter(row => {
                const statusCell = row.querySelector('.details-row');
                return statusCell && statusCell.textContent.trim() === 'Valid';
            }).length;
            "#,
            false,
        )?;

        // Convertir los resultados a tipos manejables en Rust
        let num_entries = num_rows.value.unwrap().as_i64().unwrap();
        let all_status_valid = all_status_valid.value.unwrap().as_bool().unwrap();
        let num_valid_entries = num_valid_rows.value.unwrap().as_i64().unwrap();

        assert_eq!(num_entries, 6, "The table should have 6 entries.");
        assert_eq!(
            all_status_valid, false,
            "All status columns should be 'Valid'."
        );
        assert_eq!(
            num_valid_entries, 2,
            "All status columns should be 'Valid'."
        );

        Ok(())
    }
}
