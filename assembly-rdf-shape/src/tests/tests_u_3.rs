#[cfg(test)]
mod tests_u_3 {
    use headless_chrome::Browser;
    use std::error::Error;

    const E_12: &str = r#"
    :alice@:User,
:bob@:User,
:carol@:User
    "#;

    const E_13: &str = r#"
:a@:Book,
:b@:Book,
:c@:Book,
:d@:Book,
:e@:Book,
:f@:Book
    "#;

    #[test]
    fn u_3_1() -> Result<(), Box<dyn Error>> {
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#shapemap-editor";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            &format!(
                r#"
                document.querySelector('#shapemap-editor').value = `{}`;
                "#,
                E_12.replace("`", "\\`")
            ),
            false,
        )?;

        let remote_object = tab.evaluate(
            r#"
                document.querySelector('#shapemap-editor').value;
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), E_12.trim())
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    #[test]
    fn u_3_2() -> Result<(), Box<dyn Error>> {
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#shapemap-editor";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            &format!(
                r#"
                document.querySelector('#shapemap-editor').value = `{}`;
                "#,
                E_13.replace("`", "\\`")
            ),
            false,
        )?;

        let remote_object = tab.evaluate(
            r#"
                document.querySelector('#shapemap-editor').value;
            "#,
            false,
        )?;

        match remote_object.value {
            Some(returned_value) => {
                assert_eq!(returned_value.as_str().unwrap_or("").trim(), E_13.trim())
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    #[test]
    fn u_3_3() -> Result<(), Box<dyn Error>> {
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#shapemap-editor";
        tab.wait_for_element(selector)?;

        tab.evaluate(
            &format!(
                r#"
                document.querySelector('#shapemap-editor').value = `{}`;
                "#,
                E_13.replace("`", "\\`")
            ),
            false,
        )?;

        tab.evaluate(
            r#"
                document.querySelector('#shapemap-editor').value = ``;
            "#,
            false,
        )?;

        let remote_object = tab.evaluate(
            r#"
                document.querySelector('#shapemap-editor').value;
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
