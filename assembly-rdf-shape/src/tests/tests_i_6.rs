#[cfg(test)]
mod tests_i_6 {
    use headless_chrome::Browser;
    use std::{error::Error, time::Duration};

    #[test]
    fn i_6_1() -> Result<(), Box<dyn Error>> {
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

        // Realizar búsqueda de un nodo existente
        let search_term = "carol"; // Ajusta según el caso de prueba
        tab.evaluate(
            &format!(
                r#"
                document.getElementById('search-input').value = "{}";
                document.getElementById('search-input').dispatchEvent(new Event('input'));
                "#,
                search_term
            ),
            false,
        )?;

        std::thread::sleep(Duration::from_secs(2));

        // Verificar que se ha mostrado el nodo correspondiente
        let node_displayed = tab
            .evaluate(
                r#"
                document.querySelectorAll('#result-table tr').length > 1; 
                "#,
                false,
            )?
            .value
            .unwrap()
            .as_bool()
            .unwrap_or(false);
        assert!(
            node_displayed,
            "No se mostró el nodo correspondiente después de la búsqueda."
        );

        let node_value_result = tab.evaluate(
            r#"
                document.querySelectorAll('#result-table tr:not(:first-child) td:first-child')[0].innerText.trim();
            "#,
            false,
        )?;

        // Now use `node_value` in your assertions or any further operations
        assert_eq!(
            node_value_result.value.unwrap().as_str().unwrap(),
            ":carol",
            "El valor en la primera fila de 'Node' debería ser 'carol'."
        );

        Ok(())
    }

    #[test]
    fn i_6_2() -> Result<(), Box<dyn Error>> {
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

        let search_term = "teresa";
        tab.evaluate(
            &format!(
                r#"
                document.getElementById('search-input').value = "{}";
                document.getElementById('search-input').dispatchEvent(new Event('input'));
                "#,
                search_term
            ),
            false,
        )?;

        std::thread::sleep(Duration::from_secs(2));

        // Verificar que no se ha mostrado ningún nodo correspondiente
        let node_displayed = tab
            .evaluate(
                r#"
                document.querySelectorAll('#result-table tr').length <= 1; // Verifica que no hay más de una fila (solo la cabecera)
                "#,
                false,
            )?
            .value
            .unwrap()
            .as_bool()
            .unwrap_or(false);
        assert!(
            node_displayed,
            "Se mostró un nodo correspondiente después de la búsqueda, pero no debería haber ninguno."
        );

        Ok(())
    }
}
