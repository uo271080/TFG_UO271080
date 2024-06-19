mod api;
mod examples_manager;

use std::ptr::null;
use std::{any, vec};

use log::*;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, ToString};
use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::js_sys::{wasm_bindgen, JsString};
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};


const KEY: &str = "yew.todomvc.self";

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
    rdf_parameters:Vec<String>,
    shex_parameters:Vec<String>,
    shapemap_parameters:Vec<String>,
}

#[wasm_bindgen(inline_js = "
import YATE from 'perfectkb-yate';
export function setYate(input) {
    return window.yateInstance.setValue(input);
}
")]
extern "C" {
    fn setYate(input: &str);
}

#[wasm_bindgen(inline_js = "
export function setYashe(input) {
    return window.yasheInstance.setValue(input);
}
")]
extern "C" {
    fn setYashe(input: &str);
}

#[wasm_bindgen(inline_js = "
export function getYate() {
	return window.yateInstance.getValue();
}
")]
extern "C" {
    fn getYate() -> String;
}

#[wasm_bindgen(inline_js = "
export function getYashe() {
	return window.yasheInstance.getValue();
}
")]
extern "C" {
    fn getYashe() -> String;
}

#[wasm_bindgen(inline_js = "
import YATE from 'perfectkb-yate';
export function initializeYate() {
	var yate = YATE.fromTextArea(document.getElementById('editor-yate'), {})
    window.yateInstance = yate;
}
")]
extern "C" {
    fn initializeYate();
}

#[wasm_bindgen(inline_js = "
import YASHE from 'yashe';
export function initializeYashe() {
    var yashe = YASHE.fromTextArea(document.getElementById('editor-yashe'), {});
    window.yasheInstance = yashe;
}
")]
extern "C" {
    fn initializeYashe();
}

#[wasm_bindgen(inline_js = "export function scrollToElement(id) { const element = document.getElementById(id); if(element) element.scrollIntoView({ behavior: 'smooth' }); }")]
extern "C" {
    fn scrollToElement(id: &str);
}

#[derive(Serialize, Deserialize)]
pub struct State {
    filter: Filter,
    show_result: bool,
    scroll_needed: bool,
    shapemap_value:String,
    edit_value: String,
    search_text: String,
    validation_result:Option<api::ValidationResult>
}

#[derive(Serialize, Deserialize)]
pub struct ExampleData {
    rdf: String,
    shex: String,
    shapemap: String
}

pub enum Msg {
    Validate,
    ValidationResult(api::ValidationResult),
    UpdateSearch(String),
    UpdateShapeMapValue(String),
    LoadExample,
    Nope
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let state: State = State {
            filter: Filter::RDF,
            show_result:false,
            scroll_needed: false,
            edit_value: "".into(),
            shapemap_value:"".into(),
            search_text: "".into(),
            validation_result:None
        };
        App {
            link,
            storage,
            state,
            rdf_parameters : vec![
                "Turtle".to_string(),
                "N-Triples".to_string(),
                "N-Quads".to_string(),
                "TriG".to_string(),
                "JSON-LD".to_string(),
                "RDF/XML".to_string(),
                "RDF/JSON".to_string(),
                "mixed".to_string(),
                "html-rdfa11".to_string(),
                "html-microdata".to_string(),
            ],
            shex_parameters : vec![
                "ShExC".to_string(),
                "ShExJ".to_string(),
            ],
            shapemap_parameters : vec![
                "Compact".to_string(),
                "JSON".to_string(),
            ]
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Validate => {
                print!("Incompleto");
                self.state.show_result = true;
                self.state.scroll_needed = true;
                let rdf_content = getYate();
                let shex_content = getYashe();
                let shapemap_content = self.state.shapemap_value.clone();
                let link = self.link.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let result = api::call_validation_api(rdf_content, shex_content, shapemap_content).await;
                    link.send_message(Msg::ValidationResult(result)); // Manejando la respuesta de validación
                });
            },
            Msg::UpdateShapeMapValue(new_value) => {
                self.state.shapemap_value = new_value;
            },
            Msg::ValidationResult(result) => {
                self.state.validation_result = Some(result);  // Añadir esta línea
            },
            Msg::UpdateSearch(text) => {
                self.state.search_text = text.to_lowercase();
            },
            Msg::LoadExample =>{
                let yate = r#"PREFIX :       <http://example.org/>
PREFIX schema: <http://schema.org/>
PREFIX xsd:    <http://www.w3.org/2001/XMLSchema#>
PREFIX foaf:   <http://xmlns.com/foaf/0.1/>

:alice schema:name           "Alice" ;            # %* Passes{:User} *)
       schema:gender         schema:Female ;
       schema:knows          :bob .

:bob   schema:gender         schema:Male ;        # %* Passes{:User} *)
       schema:name           "Robert";
       schema:birthDate      "1980-03-10"^^xsd:date .

:carol schema:name           "Carol" ;            # %* Passes{:User} *)
       schema:gender         "unspecified" ;
       foaf:name             "Carol" .

:dave  schema:name           "Dave";         # %* Fails{:User} *)
       schema:gender         "XYY";          #
       schema:birthDate      1980 .          # %* 1980 is not an xsd:date *)

:emily schema:name "Emily", "Emilee" ;       # %* Fails{:User} *)
       schema:gender         schema:Female . # %* too many schema:names *)

:frank foaf:name             "Frank" ;       # %* Fails{:User} *)
       schema:gender:        schema:Male .   # %* missing schema:name *)

:grace schema:name           "Grace" ;       # %* Fails{:User} *)
       schema:gender         schema:Male ;   #
       schema:knows          _:x .           # %* _:x is not an IRI *)

:harold schema:name         "Harold" ;    # %* Fails{:User} *)
        schema:gender       schema:Male ;
        schema:knows        :grace .      # %* :grace does not conform to :User *)
    "#;
                setYate(&yate);
                setYashe("PREFIX : <http://example.org/>\nPREFIX schema: <http://schema.org/>\nPREFIX xsd: <http://www.w3.org/2001/XMLSchema#>\n\n:User {\n  schema:name xsd:string ;\n  schema:birthDate xsd:date? ;\n  schema:gender [ schema:Male schema:Female ] OR xsd:string ;\n  schema:knows IRI @:User*\n}");
                self.state.shapemap_value = ":alice@:User,:bob@:User,:carol@:User,:emily@:User,:frank@:User,:grace@:User,:harold@:User".to_string();
            },
            Msg::Nope => {}
        }
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if self.state.scroll_needed && !first_render {
           scrollToElement("result");
            self.state.scroll_needed = false;
        }
        if(first_render){
            initializeYate();
            initializeYashe();
        }
        // showYashe();
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <div class="todomvc-wrapper">
                <section class="app">
                    <header class="header">
                    <nav>
                            <div class="wrapper">
                                <div class="logo"><a href="#">{"WASM - RDF VALIDATOR"}</a></div>
                                <input type="radio" name="slider" id="menu-btn" />
                                <input type="radio" name="slider" id="close-btn" />
                                <ul class="nav-links">
                                    <label for="close-btn" class="btn close-btn"><i class="fas fa-times"></i></label>
                                    <li class="menu-btn"><a class="load-example" onclick=self.link.callback(|_| Msg::LoadExample)>{"CARGAR EJEMPLO"}</a></li>
                                </ul>
                                <label for="menu-btn" class="btn menu-btn"><i class="fas fa-bars"></i></label>
                            </div>
                        </nav>
                    </header>
                    <div class="content">
                        <div class="editors-container">
                            <div class="yashe-container">
                                <h3 class="title-editor">{"RDF"}</h3>
                                <textarea id="editor-yate"></textarea>
                                { self.view_parameters(Filter::RDF) }
                                <div class="shapemap-container">
                                    <h3 class="title-editor">{"ShapeMap"}</h3>
                                    <textarea class="shapemap-editor" oninput=self.link.callback(|e: InputData| Msg::UpdateShapeMapValue(e.value))>
                                        {self.state.shapemap_value.clone()}
                                    </textarea>
                                </div>
                            </div>
                            <div class="yate-container">
                                <h3 class="title-editor">{"ShEx"}</h3>
                                <textarea id="editor-yashe"></textarea>
                                <div>
                                    { self.view_parameters(Filter::ShEx) }
                                </div>
                                <div style="margin-top: auto;">
                                    <button class="clear-completed button-27" onclick=self.link.callback(|_| Msg::Validate)>
                                        { format!("VALIDAR") }
                                    </button>
                                </div>
                            </div>
                        </div>
                        <div class="footer-options">
                        // <button class="clear-completed validate-btn" onclick=self.link.callback(|_| Msg::LoadExample)>
                        //     { format!("Testing") }
                        // </button>
                        </div>
                        <div class="result-container">
                            {self.render_result()}
                        </div>
                    </div>
                </section>
            </div>
        }
    }
}

impl App {

    async fn callValidationAPI(){
        let request_body = r#"
{
  "data": {
    "content": "PREFIX :       <http://example.org/>\nPREFIX schema: <http://schema.org/>\nPREFIX xsd:    <http://www.w3.org/2001/XMLSchema#>\nPREFIX foaf:   <http://xmlns.com/foaf/0.1/>\n\n:alice schema:name           \"Alice\" ;            # %* Passes{:User} *)\n       schema:gender         schema:Female ;\n       schema:knows          :bob .\n\n:bob   schema:gender         schema:Male ;        # %* Passes{:User} *)\n       schema:name           \"Robert\";\n       schema:birthDate      \"1980-03-10\"^^xsd:date .\n\n:carol schema:name           \"Carol\" ;            # %* Passes{:User} *)\n       schema:gender         \"unspecified\" ;\n       foaf:name             \"Carol\" .\n\n:dave  schema:name           \"Dave\";         # %* Fails{:User} *)\n       schema:gender         \"XYY\";          #\n       schema:birthDate      1980 .          # %* 1980 is not an xsd:date *)\n\n:emily schema:name \"Emily\", \"Emilee\" ;       # %* Fails{:User} *)\n       schema:gender         schema:Female . # %* too many schema:names *)\n\n:frank foaf:name             \"Frank\" ;       # %* Fails{:User} *)\n       schema:gender:        schema:Male .   # %* missing schema:name *)\n\n:grace schema:name           \"Grace\" ;       # %* Fails{:User} *)\n       schema:gender         schema:Male ;   #\n       schema:knows          _:x .           # %* _:x is not an IRI *)\n\n:harold schema:name         \"Harold\" ;    # %* Fails{:User} *)\n        schema:gender       schema:Male ;\n        schema:knows        :grace .      # %* :grace does not conform to :User *)",
    "source": "byText",
    "format": "turtle",
    "inference": "NONE"
  },
  "schema": {
    "content": "\nPREFIX :       <http://example.org/>\nPREFIX schema: <http://schema.org/>\nPREFIX xsd:  <http://www.w3.org/2001/XMLSchema#>\n\n:User {\n  schema:name          xsd:string  ;\n  schema:birthDate     xsd:date?  ;\n  schema:gender        [ schema:Male schema:Female ] OR xsd:string ;\n  schema:knows         IRI @:User*\n}\n  ",
    "source": "byText",
    "format": "ShExC",
    "engine": "ShEx"
  },
  "triggerMode": {
    "type": "ShapeMap",
    "shape-map": {
      "content": ":alice@:User,:bob@:User,:carol@:User,:emily@:User,:frank@:User,:grace@:User,:harold@:User",
      "source": "byText",
      "format": "Compact"
    }
  }
}
"#;

        // wasm_bindgen_futures::spawn_local(async move {
        //     let validation_endpoint = format!(
        //         "https://api.rdfshape.weso.es/api/schema/validate"
        //     );
        // let validation_result = Request::post(&validation_endpoint).body(request_body).send().await.unwrap().text().await.unwrap();

        // console::log_1(&JsString::from(validation_result));

        // });
    }
    // async fn loadFile(){
    //     let resp = Request::get("../static/example.json").send().await.unwrap();
    //     print!("{}", resp.status());
    // }

    fn render_result(&self) -> Html {
        info!("Show result: {}", self.state.show_result);
        if self.state.show_result {
            let search_text = self.state.search_text.clone();
            html! {
                <div class="result" id="result">
                    <table>
                        <tr>
                            <th>{"Node"}</th>
                            <th>{"Shape"}</th>
                            <th>{"Status"}</th>
                        </tr>
                        { self.render_rows(&search_text) }
                    </table>
                    <div class="result-options">
                        <input type="text" class="search" placeholder="Buscar..." oninput=self.link.callback(|e: InputData| Msg::UpdateSearch(e.value)) />
                        <button>{"CSV"}</button>
                    </div>
                </div>
            }
        } else {
            html! {
                <></>
            }
        }
    }

    fn render_rows(&self, search_text: &str) -> Html {
        if let Some(result) = &self.state.validation_result {
            let mut entries: Vec<_> = result.result.shape_map.iter()
                .filter(|entry| entry.node.contains(search_text))
                .collect();

            entries.sort_by(|a, b| b.status.cmp(&a.status));

            entries.into_iter().map(|entry| {
                html! {
                    <tr class={ if entry.status == "Valid" { "valid" } else { "invalid" } }>
                        <td>{ &entry.node }</td>
                        <td>{ &entry.shape }</td>
                        <td>{ &entry.status }</td>
                    </tr>
                }
            }).collect()
        } else {
            html! {}
        }
    }


    fn view_parameters(&self,filter: Filter) -> Html {
        match filter {
            Filter::RDF => self.view_select(&self.rdf_parameters),
            Filter::ShEx => self.view_select(&self.shex_parameters),
            Filter::ShapeMap => self.view_select(&self.shapemap_parameters),
        }
    }

    fn view_select(&self, options: &Vec<String>) -> Html {
        html! {
            <select class="select parameters">
                {for options.iter().map(|opcion| {
                    html! {
                        <option class="option-parameters" value={opcion}>{opcion}</option>
                    }
                })}
            </select>
        }
    }

}

#[derive(EnumIter, ToString, Clone, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    RDF,
    ShEx,
    ShapeMap,
}

impl State {

}
