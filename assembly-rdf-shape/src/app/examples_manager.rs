use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use std::env;

/// Datos del fichero ejemplo
///
/// # Campos
/// * `rdf` - Emtrada RDF
/// * `shex` - Emtrada Shex
/// * `shapemap` - Emtrada ShapeMap
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExampleData {
    pub rdf: String,
    pub shex: String,
    pub shapemap: String,
    pub rdf_format: String,
    pub shex_format: String,
    pub shapemap_format: String,
}

/// Carga un ejemplo desde un archivo JSON específico.
///
/// La función realiza una solicitud HTTP GET para obtener los datos del archivo
/// especificado en formato JSON y los deserializa en un `ExampleData`.
///
/// # Parámetros
/// * `file` - El nombre del archivo a cargar
/// # Retorna
/// Esta función retorna un `Result` que es `Ok` conteniendo `ExampleData` si la carga es exitosa.
/// Retorna `Err` con un mensaje de error si la carga falla debido a problemas de red o de deserialización.
///
/// # Errores
/// * Retorna un error si la solicitud HTTP no se puede completar o si el estado HTTP no es 200.
/// * Retorna un error si la deserialización del JSON falla.
pub async fn load_example(file: String) -> Result<ExampleData, String> {
    // let mut public_url = env::var("https://uo271080.github.io/TFG_UO271080/")
    //     .unwrap_or("https://uo271080.github.io/TFG_UO271080/".to_string());
    // if !public_url.ends_with('/') {
    //     public_url.push('/');
    // }

    // let path = format!("{}static/{}.json", public_url, file);

    let path = format!("/static/{}.json", file);
    let response = Request::get(&path)
        .send()
        .await
        .map_err(|err| format!("Failed to fetch: {:?}", err))?;

    if response.ok() {
        let data: ExampleData = response
            .json()
            .await
            .map_err(|err| format!("Failed to parse JSON: {:?}", err))?;
        Ok(data)
    } else {
        Err(format!(
            "Failed to load example: HTTP {}",
            response.status()
        ))
    }
}
