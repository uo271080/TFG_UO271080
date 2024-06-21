// src/api.rs
use reqwasm::http::Request;

use web_sys::console;


// src/models.rs
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Data {
    pub content: String,
    pub source: String, // Siempre es = "byText"
    pub format: String, // Siempre es "Turtle"
    pub inference: String, //Siempre es = "NONE"
}

#[derive(Serialize, Deserialize)]
pub struct Schema {
    pub content: String,
    pub source: String, // Siempre es = "byText"
    pub format: String, // Siempre es = "ShEc"
    pub engine: String, //siempre es = "ShEx"
}

#[derive(Serialize, Deserialize)]
pub struct ShapeMap {
    pub content: String,
    pub source: String, // Siempre es = "byText"
    pub format: String, // Siempre es = "Compact"
}

#[derive(Serialize, Deserialize)]
pub struct TriggerMode {
    #[serde(rename = "type")]
    pub trigger_type: String, // Siempre es "ShapeMap"
    #[serde(rename = "shape-map")]
    pub shape_map: ShapeMap,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    pub data: Data,
    pub schema: Schema,
    #[serde(rename = "triggerMode")]
    pub trigger_mode: TriggerMode,
}

#[derive(Serialize, Deserialize,Default)]
#[derive(Clone)]
#[serde(rename_all="camelCase")]
pub struct ValidationResult {
    pub result:ApiResult
}

/// Respuesta del API
#[derive(Serialize, Deserialize,Default)]
#[derive(Clone)]
#[serde(rename_all="camelCase")]
pub struct ApiResult{
    pub valid: bool,
    pub message: String,
    pub shape_map: Vec<ShapeMapEntry>,
}

/// Entrada Shape Map
#[derive(Serialize, Deserialize,Default)]
#[derive(Clone)]
#[serde(rename_all="camelCase")]
pub struct ShapeMapEntry {
    pub node: String,
    pub shape: String,
    pub status: String,
    pub reason: Option<String>,
}


pub fn create_request_body(rdf_content:String,shex_content:String,shapemap_content:String) -> RequestBody{    
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

pub async fn call_validation_api(rdf_content: String, shex_content: String, shapemap_content: String) -> (ValidationResult, String) {
    let mut error_message = "".to_string();
    let request_body = create_request_body(rdf_content, shex_content, shapemap_content);

    let validation_endpoint = "https://api.rdfshape.weso.es/api/schema/validate";
    let request_body_json = serde_json::to_string(&request_body).unwrap();
    let mut validation_result:ValidationResult = Default::default();

    let response = Request::post(validation_endpoint)
        .body(request_body_json)
        .send()
        .await
        ;

    match response {
        Ok(response) =>{
            let json: Result<ValidationResult, _> = response.json().await;
            match json {
                Ok(vr) => {
                    validation_result= vr;
                },
                Err(e)=>{
                    error_message = e.to_string();
                }
            }
        },
        Err(e) =>{
            error_message = e.to_string();
        }
    }

    let formatted_result = format_shape_maps(validation_result); 
    
    let printvresult = serde_json::to_string(&formatted_result).unwrap();
    console::log_1(&printvresult.into());
    
    (formatted_result,error_message)
}


/// Conversión de las shape maps para mostrar al usuario
pub fn format_shape_maps(response: ValidationResult) -> ValidationResult {
    let mut formatted_result = response.clone();
    let shapes = &mut formatted_result.result.shape_map;
    for entry in shapes.iter_mut() {
      entry.node = ":".to_owned()+&extract_last_segment(&entry.node);
      entry.shape =  ":".to_owned()+&extract_last_segment(&entry.shape);
      entry.status = format_status(&entry.status);
    }
    formatted_result
}

/// Formatea el estado al formato deseado
fn format_status(status:&str) -> String{
    if status == "conformant"{
        return "Valid".to_string();
    }
    else{
        return "Invalid".to_string();
    }
}

/// Extrae el nodo y el shape
fn extract_last_segment(uri: &str) -> String {
    if let Some(start) = uri.rfind('/') {
        if let Some(end) = uri.find('>') {
            return uri[start + 1..end].to_string();
        }
    }
    uri.to_string() 
}


