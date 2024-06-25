use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExampleData {
    pub rdf: String,
    pub shex: String,
    pub shapemap: String,
}

pub async fn load_example(file: String) -> Result<ExampleData, String> {
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
