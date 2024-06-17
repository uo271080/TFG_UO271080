// src/api.rs
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
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

pub fn createRequestBody(rdf_content:String,shex_content:String,shapemap_content:String) -> RequestBody{
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

    let request_body_json = serde_json::to_string(&request_body).unwrap();
    console::log_1(&request_body_json.into());

    return request_body;

}

pub async fn call_validation_api(rdf_content:String,shex_content:String,shapemap_content:String) {

    let request_body = createRequestBody(rdf_content, shex_content, shapemap_content);
    
    let validation_endpoint = "https://api.rdfshape.weso.es/api/schema/validate";
    let request_body_json = serde_json::to_string(&request_body).unwrap();

    spawn_local(async move {
        let validation_result = Request::post(validation_endpoint)
            .body(request_body_json)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        
        console::log_1(&validation_result.into());
    });
}
