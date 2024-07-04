#[cfg(test)]
mod tests_u_1 {
    use headless_chrome::Browser;
    use std::error::Error;

    const E_10: &str = r#"
    prefix :       <http://example.org/>
prefix xsd:    <http://www.w3.org/2001/XMLSchema#>
prefix schema: <http://schema.org/>

:book1  :title      "Book One" ;
        :author     :author1 ;
        :published  "2020-01-01"^^xsd:date ;
        :genre      "Fiction" ;
        :related    :book2 .

:book2  :title      "Book Two" ;
        :author     :author2 ;
        :published  "2021-05-15"^^xsd:date ;
        :genre      "Non-Fiction" ;
        :related    :book1, :book3 .
"#;

    const E_11: &str = r#"
prefix :       <http://example.org/>
prefix xsd:    <http://www.w3.org/2001/XMLSchema#>
prefix schema: <http://schema.org/>

:author1  :name    "Author One" ;
          :birthDate  "1970-12-05"^^xsd:date .

:author2  :name    "Author Two" ;
          :birthDate  "1985-03-22"^^xsd:date .
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
            window.yateInstance.setValue(`{}`);
            "#,
                E_10.replace("`", "\\`")
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
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), E_10.trim())
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
            window.yateInstance.setValue(`{}`);
            "#,
                E_11.replace("`", "\\`")
            ),
            false,
        )?;

        tab.evaluate(
            &format!(
                r#"
            window.yateInstance.setValue(`{}`);
            "#,
                E_11.replace("`", "\\`")
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
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), E_11.trim())
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
        window.yateInstance.setValue(`{}`);
        "#,
                E_11.replace("`", "\\`")
            ),
            false,
        )?;

        tab.evaluate(
            r#"
        window.yateInstance.setValue(``);
        "#,
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
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), "")
            }
            _ => unreachable!(),
        };

        Ok(())
    }
}
