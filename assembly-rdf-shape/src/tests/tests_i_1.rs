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

    #[test]
    fn i_1_1() -> Result<(), Box<dyn Error>> {
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
}
