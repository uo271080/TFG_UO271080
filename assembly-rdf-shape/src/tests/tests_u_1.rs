#[cfg(test)]
mod tests_u_1 {
    use headless_chrome::Browser;
    use std::error::Error;

    const E_1: &str = r#"
PREFIX :       <http://example.org/>
PREFIX schema: <http://schema.org/>
PREFIX xsd:    <http://www.w3.org/2001/XMLSchema#>
PREFIX foaf:   <http://xmlns.com/foaf/0.1/>

:alice schema:name      "Alice" ;
       schema:gender    schema:Female ;
       schema:knows     :bob .

:bob   schema:gender    schema:Male ;
       schema:name      "Robert" ;
       schema:birthDate "1980-03-10"^^xsd:date .
"#;

    const E_2: &str = r#"
PREFIX :       <http://example.org/>
PREFIX schema: <http://schema.org/>
PREFIX xsd:    <http://www.w3.org/2001/XMLSchema#>
PREFIX foaf:   <http://xmlns.com/foaf/0.1/>

:charlie schema:name      "Charlie" ;
         schema:gender    schema:Male ;
         schema:knows     :alice .

:alice   schema:gender    schema:Female ;
         schema:name      "Alice" ;
         schema:birthDate "1990-04-15"^^xsd:date .
"#;

    #[test]
    fn u_1_1() -> Result<(), Box<dyn Error>> {
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
                E_1.replace("`", "\\`")
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
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), E_1.trim())
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    #[test]
    fn u_1_2() -> Result<(), Box<dyn Error>> {
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
                E_1.replace("`", "\\`")
            ),
            false,
        )?;

        tab.evaluate(
            &format!(
                r#"
            window.yasheInstance.setValue(`{}`);
            "#,
                E_2.replace("`", "\\`")
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
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), E_2.trim())
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    #[test]
    fn u_1_3() -> Result<(), Box<dyn Error>> {
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
                E_1.replace("`", "\\`")
            ),
            false,
        )?;

        tab.evaluate(
            r#"
        window.yasheInstance.setValue(``);
        "#,
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
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), "")
            }
            _ => unreachable!(),
        };

        Ok(())
    }
}
