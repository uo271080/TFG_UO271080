// src/api.rs
use reqwasm::http::Request;
use web_sys::console;

use serde::{Deserialize, Serialize};

/// Objeto Data de la respuesta al andpoint validation
#[derive(Serialize, Deserialize)]
pub struct Data {
    pub content: String,
    pub source: String,
    pub format: String,
    pub inference: String,
}

/// Define esquema ShEx
#[derive(Serialize, Deserialize)]
pub struct Schema {
    pub content: String,
    pub source: String,
    pub format: String,
    pub engine: String,
}

/// Define esquema ShapeMap
#[derive(Serialize, Deserialize)]
pub struct ShapeMap {
    pub content: String,
    pub source: String,
    pub format: String,
}

/// Define objeto TriggerMode
#[derive(Serialize, Deserialize)]
pub struct TriggerMode {
    #[serde(rename = "type")]
    pub trigger_type: String,
    #[serde(rename = "shape-map")]
    pub shape_map: ShapeMap,
}

/// Define el esquema para el body de Validation
#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    pub data: Data,
    pub schema: Schema,
    #[serde(rename = "triggerMode")]
    pub trigger_mode: TriggerMode,
}

/// Define el esquema para la respuesta del método Validation
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    pub result: ApiResult,
}

/// Define el esquema para el objeto Result de la respuesta del método Validation
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiResult {
    pub valid: bool,
    pub message: String,
    pub shape_map: Vec<ShapeMapEntry>,
}

/// Define el esquema para cada ShapeMap de la respuesta del método Validation
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShapeMapEntry {
    pub node: String,
    pub shape: String,
    pub status: String,
    pub reason: String,
}

/// Define el esquema para el body del método data/info
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InfoRdfRequest {
    pub data: InfoRdfRequestContent,
}

/// Define el esquema para el objeto data del body del método data/info
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InfoRdfRequestContent {
    pub content: String,
    pub format: String,
    pub inference: String,
    pub source: String,
}

/// Define el esquema para la respuesta del método data/info
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InfoRdfResponse {
    pub message: String,
    pub result: InfoRdfResult,
}

/// Define el esquema objeto result para la respuesta del método data/info
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InfoRdfResult {
    pub number_of_statements: i32,
}

/// Define el esquema para el body del método schema/info
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InfoShexRequest {
    pub schema: InfoShexRequestContent,
}

/// Define el esquema para el objeto schema el body del método schema/info
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InfoShexRequestContent {
    pub content: String,
    pub engine: String,
    pub format: String,
    pub source: String,
}

/// Define el esquema la respuesta del método schema/info
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InfoShexResponse {
    pub message: String,
    pub result: InfoShexResult,
}

/// Define el esquema el objeto result la respuesta del método schema/info
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InfoShexResult {
    pub shapes: Vec<String>,
    pub prefix_map: Vec<Prefix>,
}

/// Define el esquema el objeto Prefix la respuesta del método schema/info
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Prefix {
    prefix_name: String,
    prefix_IRI: String,
}

/// Construye el cuerpo de la solicitud para validar RDF, ShEx y ShapeMap.
///
/// # Parámetros
/// * `rdf_content` - Contenido RDF
/// * `shex_content` - Esquema ShEx
/// * `shapemap_content` - Contenido del ShapeMap
///
/// # Retorna
/// Devuelve un `RequestBody` estructurado con los datos proporcionados.
pub fn create_validation_request_body(
    rdf_content: String,
    shex_content: String,
    shapemap_content: String,
) -> RequestBody {
    let data = Data {
        content: rdf_content,
        source: "byText".to_string(),
        format: "turtle".to_string(),
        inference: "NONE".to_string(),
    };

    let schema = Schema {
        content: shex_content,
        source: "byText".to_string(),
        format: "ShExC".to_string(),
        engine: "ShEx".to_string(),
    };

    let shape_map = ShapeMap {
        content: shapemap_content,
        source: "byText".to_string(),
        format: "Compact".to_string(),
    };

    let trigger_mode = TriggerMode {
        trigger_type: "ShapeMap".to_string(),
        shape_map,
    };

    let request_body = RequestBody {
        data,
        schema,
        trigger_mode,
    };

    return request_body;
}

/// Realiza una llamada a la API para validar RDF, ShEx y ShapeMap y maneja los resultados.
///
/// # Parámetros
/// * `rdf_content` - Contenido RDF
/// * `shex_content` - Esquema ShEx
/// * `shapemap_content` - Contenido del ShapeMap
///
/// # Retorna
/// Retorna un tuple `(ValidationResult, String)` donde `ValidationResult` es el resultado de la validación
/// y `String` es un mensaje de error, si ocurrió alguno durante la operación.
pub async fn call_validation_api(
    rdf_content: String,
    shex_content: String,
    shapemap_content: String,
) -> (ValidationResult, String) {
    let mut error_message = "".to_string();
    let request_body = create_validation_request_body(rdf_content, shex_content, shapemap_content);

    let validation_endpoint = "https://api.rdfshape.weso.es/api/schema/validate";
    let request_body_json = serde_json::to_string(&request_body).unwrap();
    let mut validation_result: ValidationResult = Default::default();

    let response = Request::post(validation_endpoint)
        .body(request_body_json)
        .send()
        .await;

    match response {
        Ok(response) => {
            let json: Result<ValidationResult, _> = response.json().await;
            match json {
                Ok(vr) => {
                    validation_result = vr;
                }
                Err(e) => {
                    error_message = e.to_string();
                }
            }
        }
        Err(e) => {
            error_message = e.to_string();
        }
    }

    let formatted_result = format_shape_maps(validation_result);

    let printvresult = serde_json::to_string(&formatted_result).unwrap();
    console::log_1(&printvresult.into());

    (formatted_result, error_message)
}

/// Realiza una solicitud a la API para obtener información sobre RDF.
///
/// Esta función envía una solicitud POST a un endpoint específico, esperando recibir
/// detalles sobre la validación RDF como el número de declaraciones.
///
/// # Parámetros
/// * `rdf` - El contenido RDF a analizar.
///
/// # Retorna
/// Retorna un tuple (`InfoRdfResponse`, `String`), donde `InfoRdfResponse` contiene la
/// respuesta de la API y `String` contiene un mensaje de error en caso de que ocurra uno.
pub async fn call_rdf_info_api(rdf: String, format: String) -> (InfoRdfResponse, String) {
    let mut error_message = "".to_string();
    let request_body = create_rdf_info_request_body(rdf, format);

    let info_endpoint = "https://api.rdfshape.weso.es/api/data/info";
    let request_body_json = serde_json::to_string(&request_body).unwrap();
    let mut info_response: InfoRdfResponse = Default::default();

    let response = Request::post(info_endpoint)
        .body(request_body_json)
        .send()
        .await;

    match response {
        Ok(response) => {
            let json: Result<InfoRdfResponse, _> = response.json().await;
            match json {
                Ok(vr) => {
                    info_response = vr;
                }
                Err(e) => {
                    error_message = e.to_string();
                }
            }
        }
        Err(e) => {
            error_message = e.to_string();
        }
    }

    let printvresult = serde_json::to_string(&info_response).unwrap();
    console::log_1(&printvresult.into());

    (info_response, error_message)
}

/// Realiza una solicitud a la API para obtener información sobre ShEx.
///
/// Envía una solicitud POST para obtener detalles de un esquema ShEx, como las formas y
/// mapas de prefijos utilizados en el esquema.
///
/// # Parámetros
/// * `shex` - El contenido ShEx a analizar.
///
/// # Retorna
/// Retorna un tuple (`InfoShexResponse`, `String`), donde `InfoShexResponse` contiene la
/// respuesta de la API y `String` contiene un mensaje de error en caso de que ocurra uno.
pub async fn call_shex_info_api(shex: String) -> (InfoShexResponse, String) {
    let mut error_message = "".to_string();
    let request_body = create_shex_info_request_body(shex);

    let info_endpoint = "https://api.rdfshape.weso.es/api/schema/info";
    let request_body_json = serde_json::to_string(&request_body).unwrap();
    let mut info_response: InfoShexResponse = Default::default();

    let response = Request::post(info_endpoint)
        .body(request_body_json)
        .send()
        .await;

    match response {
        Ok(response) => {
            let json: Result<InfoShexResponse, _> = response.json().await;
            match json {
                Ok(vr) => {
                    info_response = vr;
                }
                Err(e) => {
                    error_message = e.to_string();
                }
            }
        }
        Err(e) => {
            error_message = e.to_string();
        }
    }

    let printvresult = serde_json::to_string(&info_response).unwrap();
    console::log_1(&printvresult.into());

    (info_response, error_message)
}

/// Construye el cuerpo de la solicitud para obtener información RDF.
///
/// # Parámetros
/// * `rdf` - El contenido RDF
///
/// # Retorna
/// Retorna una estructura `InfoRdfRequest` preparada para ser enviada a la API.
pub fn create_rdf_info_request_body(rdf: String, format: String) -> InfoRdfRequest {
    let data_request = InfoRdfRequestContent {
        content: rdf,
        format: format,
        inference: "NONE".to_string(),
        source: "byText".to_string(),
    };

    let request_body = InfoRdfRequest { data: data_request };

    request_body
}

/// Construye el cuerpo de la solicitud para obtener información ShEx.
///
/// # Parámetros
/// * `shex` - El contenido ShEx
///
/// # Retorna
/// Retorna una estructura `InfoShexRequest` preparada para ser enviada a la API.
pub fn create_shex_info_request_body(shex: String) -> InfoShexRequest {
    let data_request = InfoShexRequestContent {
        content: shex,
        engine: "ShEx".to_string(),
        format: "ShExC".to_string(),
        source: "byText".to_string(),
    };

    let request_body = InfoShexRequest {
        schema: data_request,
    };

    request_body
}

/// Formatea las entradas de ShapeMap para mejorar la legibilidad en la visualización final.
///
/// # Parámetros
/// * `response` - Resultado de la validación que contiene el mapa de formas.
///
/// # Retorna
/// Devuelve un `ValidationResult` modificado con los nodos y formas formateados.
pub fn format_shape_maps(response: ValidationResult) -> ValidationResult {
    let mut formatted_result = response.clone();
    let shapes = &mut formatted_result.result.shape_map;
    for entry in shapes.iter_mut() {
        entry.node = ":".to_owned() + &extract_last_segment(&entry.node);
        entry.shape = ":".to_owned() + &extract_last_segment(&entry.shape);
        entry.status = format_status(&entry.status);
    }
    formatted_result
}

/// Formatea el estado de la validación para ser mostrado de manera clara.
///
/// # Parámetros
/// * `status` - Estado de validación original.
///
/// # Retorna
/// Devuelve una cadena que representa el estado formateado ("Valid" o "Invalid").
fn format_status(status: &str) -> String {
    if status == "conformant" {
        return "Valid".to_string();
    } else {
        return "Invalid".to_string();
    }
}

/// Extrae la última parte de una URI, usada para simplificar las referencias en las visualizaciones.
///
/// # Parámetros
/// * `uri` - La URI completa desde la cual extraer el segmento.
///
/// # Retorna
/// Devuelve una cadena que representa el último segmento de la URI.
fn extract_last_segment(uri: &str) -> String {
    if let Some(start) = uri.rfind('/') {
        if let Some(end) = uri.find('>') {
            return uri[start + 1..end].to_string();
        }
    }
    uri.to_string()
}
