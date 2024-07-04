#[cfg(test)]
mod tests_i_7 {
    use headless_chrome::Browser;
    use std::{error::Error, time::Duration};

    fn normalize_string(input: &str) -> String {
        input.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    #[test]
    fn i_7_1() -> Result<(), Box<dyn Error>> {
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
            r#"
                $('#validate-btn').click();
            "#,
            false,
        )?;

        std::thread::sleep(Duration::from_secs(2));

        // Click on the details button for node :a
        tab.evaluate(
            r#"
                let nodeRow = Array.from(document.querySelectorAll('#result-table tr')).find(row => row.cells[0].innerText.trim() === ':b');
                if (nodeRow) {
                    let detailsButton = nodeRow.querySelector('td button.show-btn');
                    if (detailsButton) {
                        detailsButton.click();
                    }
                }
            "#,
            false,
        )?;

        std::thread::sleep(Duration::from_secs(2));

        // Verify that the modal with detailed information is displayed
        let modal_displayed = tab
            .evaluate(
                r#"
                document.querySelector('.reason-modal') !== null;
            "#,
                false,
            )?
            .value
            .unwrap()
            .as_bool()
            .unwrap_or(false);
        assert!(
            modal_displayed,
            "No se mostró el modal con la información detallada del nodo válido."
        );

        // Verify the modal content
        let expected_content = normalize_string(
            r#"
                <http://example.org/b> is an IRI

                "Title B" has datatype <http://www.w3.org/2001/XMLSchema#string>

                <http://example.org/b> is an IRI

                "Title B" has datatype <http://www.w3.org/2001/XMLSchema#string>
            "#,
        );

        let modal_content = normalize_string(
            tab
                .evaluate(
                    r#"
                    Array.from(document.querySelectorAll('.reason-modal-body p')).map(p => p.innerText.trim()).join("\n");
                "#,
                    false,
                )?
                .value
                .unwrap()
                .as_str()
                .unwrap(),
        );

        assert_eq!(
            modal_content, expected_content,
            "El contenido del modal no es el esperado para el nodo válido."
        );

        Ok(())
    }

    #[test]
    fn i_7_2() -> Result<(), Box<dyn Error>> {
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
            r#"
                $('#validate-btn').click();
            "#,
            false,
        )?;

        std::thread::sleep(Duration::from_secs(2));

        // Click on the details button for node :e
        tab.evaluate(
            r#"
                let nodeRow = Array.from(document.querySelectorAll('#result-table tr')).find(row => row.cells[0].innerText.trim() === ':e');
                if (nodeRow) {
                    let detailsButton = nodeRow.querySelector('td button.show-btn');
                    if (detailsButton) {
                        detailsButton.click();
                    }
                }
            "#,
            false,
        )?;

        std::thread::sleep(Duration::from_secs(2));

        // Verify that the modal with detailed error information is displayed
        let modal_displayed = tab
            .evaluate(
                r#"
                document.querySelector('.reason-modal') !== null;
            "#,
                false,
            )?
            .value
            .unwrap()
            .as_bool()
            .unwrap_or(false);
        assert!(
            modal_displayed,
            "No se mostró el modal con la información detallada del nodo inválido."
        );

        // Verify the modal content
        let expected_content = normalize_string(
            r#"
                Error matching expression.

                Error: es.weso.rbe.NonNullableError: Required properties not found: :name,:related*

                Regular expression: :name,:related*

                Properties found: {|  |}

                Extra symbols: :name/1

                Open?: false

                Attempt: Attempt: [<http://example.org/e>,<http://example.org/Book>]

                path:

                Candidate line:

                Candidate line:

                which corresponds to bag:

                {|  |}

                does not match expression:

                :name,:related*

                Table:Constraints:

                :related->@<http://example.org/Book>

                :name-><http://www.w3.org/2001/XMLSchema#string>

                Paths:

                <http://example.org/name>->:name

                <http://example.org/related>->:related

                ---endTable
            "#,
        );

        let modal_content = normalize_string(
            tab
                .evaluate(
                    r#"
                    Array.from(document.querySelectorAll('.reason-modal-body p')).map(p => p.innerText.trim()).join("\n");
                "#,
                    false,
                )?
                .value
                .unwrap()
                .as_str()
                .unwrap(),
        );

        assert_eq!(
            modal_content, expected_content,
            "El contenido del modal no es el esperado para el nodo inválido."
        );

        Ok(())
    }
}
