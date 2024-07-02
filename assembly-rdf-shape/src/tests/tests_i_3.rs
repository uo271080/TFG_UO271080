#[cfg(test)]
mod tests_i_3 {
    use headless_chrome::Browser;
    use std::error::Error;
    use std::time::Duration;

    const RDF_EXAMPLE_1: &str = r#"
    PREFIX : <http://example.org/>
    PREFIX schema: <http://schema.org/>
    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
    PREFIX foaf: <http://xmlns.com/foaf/0.1/>

    :alice schema:name "Alice" ;
           schema:gender schema:Female ;
           schema:knows :bob .

    :bob schema:gender schema:Male ;
         schema:name "Robert" ;
         schema:birthDate "1980-03-10"^^xsd:date .

    :carol schema:name "Carol" ;
           schema:gender "unspecified" ;
           foaf:name "Carol" .

    :dave schema:name "Dave" ;
          schema:gender "XYY" ;
          schema:birthDate "1980-01-01"^^xsd:date .

    :emily schema:name "Emily" ;
           schema:alternateName "Emilee" ;
           schema:gender schema:Female .

    :frank schema:name "Frank" ;
           schema:gender schema:Male .

    :grace schema:name "Grace" ;
           schema:gender schema:Male ;
           schema:knows :bob .

    :harold schema:name "Harold" ;
            schema:gender schema:Male ;
            schema:knows :grace .
    "#;

    const SHEX_EXAMPLE_1: &str = r#"
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

    const SHAPEMAP_EXAMPLE_1: &str = r#"
    :alice@:User,:bob@:User,:carol@:User,:emily@:User,:frank@:User,:grace@:User,:harold@:User
    "#;

    const RDF_EXAMPLE_3: &str = r#"
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

    const SHEX_EXAMPLE_3: &str = r#"
    prefix :       <http://example.org/>
    prefix xsd:    <http://www.w3.org/2001/XMLSchema#>
    prefix schema: <http://schema.org/>

    :Book IRI and {
     :name   xsd:string   ;
     :related      @:Book     *
    }
    "#;

    const SHAPEMAP_EXAMPLE_3: &str = r#"
    :a@:Book,
    :b@:Book,
    :c@:Book,
    :d@:Book,
    :e@:Book,
    :f@:Book
    "#;

    #[test]
    fn i_3_1() -> Result<(), Box<dyn Error>> {
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element_with_custom_timeout(selector, Duration::from_secs(10))?;

        // Hacer click en el elemento #example-1 usando el índice 0
        tab.evaluate(
            r#"
            document.querySelectorAll('#example-1')[0].click();
            "#,
            false,
        )?;

        // Esperar a que los editores carguen los datos del ejemplo
        std::thread::sleep(Duration::from_secs(5)); // Ajusta la duración si es necesario

        // Obtener el valor del editor RDF
        let rdf_content = tab.evaluate(
            r#"
            window.yateInstance.getValue();
            "#,
            false,
        )?;

        let rdf_content_trimmed = rdf_content
            .value
            .unwrap()
            .as_str()
            .unwrap()
            .replace(|c: char| c.is_whitespace(), "");
        let rdf_example_trimmed = RDF_EXAMPLE_1.replace(|c: char| c.is_whitespace(), "");

        assert_eq!(rdf_content_trimmed, rdf_example_trimmed);

        // Obtener el valor del editor ShEx
        let shex_content = tab.evaluate(
            r#"
            window.yasheInstance.getValue();
            "#,
            false,
        )?;

        let shex_content_trimmed = shex_content
            .value
            .unwrap()
            .as_str()
            .unwrap()
            .replace(|c: char| c.is_whitespace(), "");
        let shex_example_trimmed = SHEX_EXAMPLE_1.replace(|c: char| c.is_whitespace(), "");

        assert_eq!(shex_content_trimmed, shex_example_trimmed);

        // Obtener el valor del editor ShapeMap
        let shapemap_content = tab.evaluate(
            r#"
            document.querySelector('#shapemap-editor').value;
            "#,
            false,
        )?;

        let shapemap_content_trimmed = shapemap_content
            .value
            .unwrap()
            .as_str()
            .unwrap()
            .replace(|c: char| c.is_whitespace(), "");
        let shapemap_example_trimmed = SHAPEMAP_EXAMPLE_1.replace(|c: char| c.is_whitespace(), "");

        assert_eq!(shapemap_content_trimmed, shapemap_example_trimmed);

        Ok(())
    }

    #[test]
    fn i_3_2() -> Result<(), Box<dyn Error>> {
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to("http://localhost:8000/")?;
        let selector = "#editors-container";
        tab.wait_for_element_with_custom_timeout(selector, Duration::from_secs(10))?;

        // Hacer click en el elemento #example-1 usando el índice 0
        tab.evaluate(
            r#"
            document.querySelectorAll('#example-1')[0].click();
            "#,
            false,
        )?;

        // Esperar a que los editores carguen los datos del ejemplo 1
        std::thread::sleep(Duration::from_secs(5)); // Ajusta la duración si es necesario

        // Hacer click en el elemento #example-3 usando el índice 2
        tab.evaluate(
            r#"
            document.querySelectorAll('#example-3')[0].click();
            "#,
            false,
        )?;

        // Esperar a que los editores carguen los datos del ejemplo 3
        std::thread::sleep(Duration::from_secs(5)); // Ajusta la duración si es necesario

        // Obtener el valor del editor RDF
        let rdf_content = tab.evaluate(
            r#"
            window.yateInstance.getValue();
            "#,
            false,
        )?;

        let rdf_content_trimmed = rdf_content
            .value
            .unwrap()
            .as_str()
            .unwrap()
            .replace(|c: char| c.is_whitespace(), "");
        let rdf_example_trimmed = RDF_EXAMPLE_3.replace(|c: char| c.is_whitespace(), "");

        assert_eq!(rdf_content_trimmed, rdf_example_trimmed);

        // Obtener el valor del editor ShEx
        let shex_content = tab.evaluate(
            r#"
            window.yasheInstance.getValue();
            "#,
            false,
        )?;

        let shex_content_trimmed = shex_content
            .value
            .unwrap()
            .as_str()
            .unwrap()
            .replace(|c: char| c.is_whitespace(), "");
        let shex_example_trimmed = SHEX_EXAMPLE_3.replace(|c: char| c.is_whitespace(), "");

        assert_eq!(shex_content_trimmed, shex_example_trimmed);

        // Obtener el valor del editor ShapeMap
        let shapemap_content = tab.evaluate(
            r#"
            document.querySelector('#shapemap-editor').value;
            "#,
            false,
        )?;

        let shapemap_content_trimmed = shapemap_content
            .value
            .unwrap()
            .as_str()
            .unwrap()
            .replace(|c: char| c.is_whitespace(), "");
        let shapemap_example_trimmed = SHAPEMAP_EXAMPLE_3.replace(|c: char| c.is_whitespace(), "");

        assert_eq!(shapemap_content_trimmed, shapemap_example_trimmed);

        Ok(())
    }
}
