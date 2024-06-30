#[cfg(test)]
mod tests {
    use super::*;
    use headless_chrome::{protocol::cdp::Page, Browser};
    use std::error::Error;

    #[test]
    fn test_browse_wikipedia() -> Result<(), Box<dyn Error>> {
        let browser = Browser::default()?;

        let tab = browser.new_tab()?;

        // Navega a Wikipedia
        tab.navigate_to("https://www.wikipedia.org")?;
        tab.wait_for_element("input#searchInput")?.click()?;
        tab.type_str("WebKit")?.press_key("Enter")?;

        // Verificaciones
        let elem = tab.wait_for_element("#firstHeading")?;
        assert!(tab.get_url().ends_with("WebKit"));

        // Captura de pantalla
        let jpeg_data =
            tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;
        std::fs::write("screenshot.jpeg", jpeg_data)?;

        // let png_data = tab
        //     .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
        //     .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;
        // std::fs::write("screenshot.png", png_data)?;

        // Ejecución de JavaScript
        let remote_object = elem.call_js_fn(
            r#"
            function getIdTwice () {
                const id = this.id;
                return id + id;
            }
            "#,
            vec![],
            false,
        )?;
        match remote_object.value {
            Some(returned_string) => {
                assert_eq!(returned_string, "firstHeadingfirstHeading".to_string())
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    #[test]
    fn test_my_web() -> Result<(), Box<dyn Error>> {
        let browser = Browser::default()?;

        let tab = browser.new_tab()?;

        // Navega a Wikipedia
        tab.navigate_to("http://localhost:8000/")?;

        let selector = "#editors-container"; // Cambia esto por un selector adecuado
        tab.wait_for_element(selector)?;

        let jpeg_data =
            tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;
        std::fs::write("miweb.jpeg", jpeg_data)?;

        // let png_data = tab
        //     .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
        //     .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;
        // std::fs::write("screenshot.png", png_data)?;

        // Ejecución de JavaScript
        // let remote_object = elem.call_js_fn(
        //     r#"
        //     function getIdTwice () {
        //         const id = this.id;
        //         return id + id;
        //     }
        //     "#,
        //     vec![],
        //     false,
        // )?;
        // match remote_object.value {
        //     Some(returned_string) => {
        //         assert_eq!(returned_string, "firstHeadingfirstHeading".to_string())
        //     }
        //     _ => unreachable!(),
        // };

        Ok(())
    }
}
