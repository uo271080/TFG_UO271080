#[cfg(test)]
mod tests_i_2 {
    use headless_chrome::Browser;
    use std::error::Error;

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

    #[test]
    fn i_2_1() -> Result<(), Box<dyn Error>> {
        println!("Running test: Analyze ShEx invalid entry");

        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            &format!(
                r#"
            window.yasheInstance.setValue(`{}`);
            "#,
                E_13.replace("`", "\\`")
            ),
            false,
        )?;

        let remote_object = tab.evaluate(
            r#"
            window.yasheInstance.getValue();
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), E_13.trim())
            }
            _ => unreachable!(),
        };

        tab.evaluate(
            r#"
            $('#analyze-shex').click();
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
                    "Status: Schema is not well-formed."
                )
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    #[test]
    fn i_2_2() -> Result<(), Box<dyn Error>> {
        println!("Running test: Analyze ShExC valid entry");

        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            &format!(
                r#"
            window.yasheInstance.setValue(`{}`);
            "#,
                E_14.replace("`", "\\`")
            ),
            false,
        )?;

        let remote_object = tab.evaluate(
            r#"
            window.yasheInstance.getValue();
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), E_14.trim())
            }
            _ => unreachable!(),
        };

        tab.evaluate(
            r#"
            $('#analyze-shex').click();
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
                    "Status: Well formed SchemaNumber of shapes: 1Number of prefixes: 3"
                )
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    #[test]
    fn i_2_3() -> Result<(), Box<dyn Error>> {
        println!("Running test: Analyze ShExC valid entry");

        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            &format!(
                r#"
            window.yasheInstance.setValue(`{}`);
            "#,
                E_15.replace("`", "\\`")
            ),
            false,
        )?;

        let remote_object = tab.evaluate(
            r#"
            window.yasheInstance.getValue();
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), E_15.trim())
            }
            _ => unreachable!(),
        };

        tab.evaluate(
            r#"
            document.getElementById('select-shex').value = 'ShExJ';
            // Trigger change event
            var event = new Event('change', { bubbles: true });
            document.getElementById('select-shex').dispatchEvent(event);
            "#,
            false,
        )?;

        tab.evaluate(
            r#"
            $('#analyze-shex').click();
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
                    "Status: Well formed SchemaNumber of shapes: 1Number of prefixes: 3"
                )
            }
            _ => unreachable!(),
        };

        Ok(())
    }
}
