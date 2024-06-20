// use reqwasm::http::Request;
// // src/api.rs
// use reqwasm::http::Request;
// use wasm_bindgen::JsValue;
// use wasm_bindgen_futures::JsFuture;
// use web_sys::{console, Blob, FileReader};
// #[derive(Serialize, Deserialize,Default)]
// #[derive(Clone)]
// #[serde(rename_all="camelCase")]
// pub struct ExampleData{
//     pub rdf: String,
//     pub shex: String,
//     pub shape_map: String,
// }


// pub async fn read_json_file(file: Blob) -> Result<ExampleData, JsValue> {
//     let file_reader = FileReader::new()?;
//     let result = file_reader.read_as_text(&file)?;
//     let promise = JsFuture::from(result).await?;
//     let text = promise.as_string().unwrap();
//     let data: ExampleData = serde_json::from_str(&text)?;
//     Ok(data)
// }