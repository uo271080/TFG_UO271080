pub(crate) mod api;
mod examples_manager;
mod rdf_properties;

use crate::components::{editors::Editor, header::Header, modal::Modal, result_table::ResultTable, search_bar::SearchBar};
use api::{ShapeMap, ShapeMapEntry};
use log::*;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, ToString};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    state: State,
    rdf_parameters: Vec<String>,
    shex_parameters: Vec<String>,
    shapemap_parameters: Vec<String>,
}

#[wasm_bindgen(inline_js = r#"
export function exportCsv(csvContent, fileName) {
    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
    const link = document.createElement('a');
    const url = URL.createObjectURL(blob);
    link.setAttribute('href', url);
    link.setAttribute('download', fileName);
    link.style.visibility = 'hidden';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
}
"#)]
extern "C" {
    pub fn exportCsv(csvContent: &str, fileName: &str);
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

#[wasm_bindgen(inline_js = "
export function scrollToElement(id) {
    const element = document.getElementById(id);
    if (element) {
        const elementRect = element.getBoundingClientRect();
        const viewportHeight = window.innerHeight || document.documentElement.clientHeight;
        const scrollY = elementRect.top + window.pageYOffset - (viewportHeight / 2);
        element.scrollIntoView({
            behavior: 'smooth',
            block: 'center',
        });
    }
}
")]
extern "C" {
    fn scrollToElement(id: &str);
}

#[derive(Serialize, Deserialize)]
pub struct State {
    filter: Filter,
    show_result: bool,
    scroll_needed: bool,
    shapemap_value: String,
    edit_value: String,
    search_text: String,
    validation_result: Option<api::ValidationResult>,
    api_error: String,
    show_reason: bool,
    selected_shape: ShapeMapEntry,
}

#[derive(Serialize, Deserialize)]
pub struct ExampleData {
    rdf: String,
    shex: String,
    shapemap: String,
}

pub enum Msg {
    Validate,
    ValidationResult(api::ValidationResult, String),
    UpdateSearch(String),
    UpdateShapeMapValue(String),
    LoadExample,
    CloseAlert,
    OpenModal(String, String, String),
    CloseModal,
    ExportToCsv,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state: State = State {
            filter: Filter::RDF,
            show_result: false,
            show_reason: false,
            scroll_needed: false,
            edit_value: "".into(),
            shapemap_value: "".into(),
            search_text: "".into(),
            validation_result: None,
            api_error: "".into(),
            selected_shape: Default::default(),
        };
        App {
            link,
            state,
            rdf_parameters: vec![
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
            shex_parameters: vec![
                "ShExC".to_string(),
                "ShExJ".to_string(),
            ],
            shapemap_parameters: vec![
                "Compact".to_string(),
                "JSON".to_string(),
            ],
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Validate => {
                self.state.api_error = "".to_string();
                self.state.validation_result = Default::default();
                self.state.show_result = true;
                self.state.scroll_needed = true;
                let rdf_content = getYate();
                let shex_content = getYashe();
                let shapemap_content = self.state.shapemap_value.clone();
                let link = self.link.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let result = api::call_validation_api(rdf_content, shex_content, shapemap_content).await;
                    link.send_message(Msg::ValidationResult(result.0, result.1));
                });
            },
            Msg::CloseAlert => {
                self.state.api_error = "".to_string();
            },
            Msg::OpenModal(node, shape, reason) => {
                print!("LLEGO A OPEN MODAL");
                print!("{}", node);
                print!("{}", shape);
                print!("{}", reason);
                self.state.show_reason = true;
                self.state.selected_shape = ShapeMapEntry { node, shape, status: "".to_string(), reason };
            },
            Msg::CloseModal => {
                self.state.show_reason = false;
            },
            Msg::ExportToCsv => {
                let csv_data = App::format_csv_data(&self);
                exportCsv(&csv_data, "export.csv");
            },
            Msg::UpdateShapeMapValue(new_value) => {
                self.state.shapemap_value = new_value;
            },
            Msg::ValidationResult(result, error) => {
                if !error.is_empty() {
                    self.state.api_error = error;
                } else {
                    self.state.validation_result = Some(result);
                }
            },
            Msg::UpdateSearch(text) => {
                self.state.search_text = text.to_lowercase();
            },
            Msg::LoadExample => {
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
            }
        }
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if self.state.scroll_needed && !first_render {
            scrollToElement("result");
            self.state.scroll_needed = false;
        }
        // if first_render {
        //     initializeYate();
        //     initializeYashe();
        // }
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <div class="todomvc-wrapper">
                <section class="app">
                    <Header on_load_example=self.link.callback(|_| Msg::LoadExample) />
                    <div class="content">
                        <Editor
                            shapemap_value=self.state.shapemap_value.clone()
                            on_update_shapemap_value=self.link.callback(Msg::UpdateShapeMapValue)
                            on_validate=self.link.callback(|_| Msg::Validate)
                            rdf_parameters=self.rdf_parameters.clone()
                            shex_parameters=self.shex_parameters.clone()
                            shapemap_parameters=self.shapemap_parameters.clone()
                        />
                        <div class="footer-options">
                        </div>
                        <div class="result-container">
                            {self.render_result()}
                            <div class="result-options">
                            <SearchBar on_search=self.link.callback(Msg::UpdateSearch) />
                            <button onclick=self.link.callback(|_| Msg::ExportToCsv)>{ "Export to CSV" }</button>
                            </div>
                        </div>
                    </div>
                </section>
            </div>
        }
    }
}

impl App {
    fn format_csv_data(&self) -> String {
        if let Some(result) = &self.state.validation_result {
            let header = "Node;Shape;Status;Reason\n".to_string();
            let entries: Vec<_> = result.result.shape_map.iter()
                .map(|entry| {
                    format!("{};{};{};{}\n", entry.node, entry.shape, entry.status, entry.reason.replace('\n', ""))
                })
                .collect();

            let csv_data = entries.into_iter().fold(header, |acc, line| acc + &line);
            csv_data
        } else {
            "".to_string()
        }
    }

    
    fn render_result(&self) -> Html {
        html! {
            <>
                { if self.state.show_reason {
                    html! {
                        <>
                            <div class="reason-modal-overlay" onclick=self.link.callback(|_| Msg::CloseModal)></div>
                            <Modal
                                node=self.state.selected_shape.node.clone()
                                reason=self.state.selected_shape.reason.clone()
                                on_close=self.link.callback(|_| Msg::CloseModal)
                            />
                        </>
                    }
                } else {
                    html! { <></> }
                }}
                
                { if self.state.show_result && self.state.validation_result.is_some() {
                    let entries = self.state.validation_result.as_ref().unwrap().result.shape_map.clone();
                    html! {
                        <ResultTable
                            entries=entries
                            search_text=self.state.search_text.clone()
                            on_open_modal=self.link.callback(|(node, shape, reason)| Msg::OpenModal(node, shape, reason))
                        />
                    }
                } else if !self.state.api_error.is_empty() {
                    html! {
                        <div class="alert-error">
                            {"Error en la validación. Revise las entradas."}
                            <button class={"close-btn "} onclick=self.link.callback(|_| Msg::CloseAlert)>{ "X" }</button>
                        </div>
                    }
                } else {
                    html! { <></> }
                }}
            </>
        }
    }
}

#[derive(EnumIter, ToString, Clone, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    RDF,
    ShEx,
    ShapeMap,
}
