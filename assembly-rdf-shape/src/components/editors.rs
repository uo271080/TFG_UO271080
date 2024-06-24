use crate::app::api::{self, InfoRdfResponse, InfoShexResponse};
use log::info;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

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

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub shapemap_value: String,
    pub on_update_shapemap_value: Callback<String>,
    pub on_validate: Callback<()>,
    pub on_open_modal: Callback<(String, String)>,
    pub rdf_parameters: Vec<String>,
    pub shex_parameters: Vec<String>,
    pub shapemap_parameters: Vec<String>,
}

pub struct Editor {
    link: ComponentLink<Self>,
    props: Props,
    analyzer_error: bool,
}

pub enum Msg {
    UpdateShapeMapValue(String),
    Validate,
    AnalyzeRDF,
    AnalyzeShex,
    ReceiveShexAnalysis((InfoShexResponse, String)),
    OpenModal(String, String),
    ReceiveRDFAnalysis((InfoRdfResponse, String)),
}

impl Component for Editor {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            analyzer_error: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateShapeMapValue(value) => {
                self.props.on_update_shapemap_value.emit(value);
                true
            }
            Msg::Validate => {
                self.props.on_validate.emit(());
                true
            }
            Msg::AnalyzeRDF => {
                let link = self.link.clone(); // Clonar el enlace del componente para usar en el contexto async
                wasm_bindgen_futures::spawn_local(async move {
                    let content = api::call_rdf_info_api(getYate()).await;
                    link.send_message(Msg::ReceiveRDFAnalysis(content));
                });
                true
            }
            Msg::ReceiveRDFAnalysis(result) => {
                if !result.1.is_empty() {
                    let content =
                        format!("Status: RDF is not well-formed. Errors have been identified.",);
                    self.props
                        .on_open_modal
                        .emit(("RDF PROPERTIES".to_string(), content));
                } else {
                    let number_statements = result.0.result.number_of_statements;
                    let content = format!(
                        "Status: {}\nNumber of statements: {}",
                        result.0.message, number_statements
                    );
                    self.props
                        .on_open_modal
                        .emit(("RDF PROPERTIES".to_string(), content));
                }
                true
            }
            Msg::AnalyzeShex => {
                let link = self.link.clone(); // Clonar el enlace del componente para usar en el contexto async
                wasm_bindgen_futures::spawn_local(async move {
                    let content: (InfoShexResponse, String) =
                        api::call_shex_info_api(getYashe()).await;
                    link.send_message(Msg::ReceiveShexAnalysis(content));
                });
                true
            }
            Msg::ReceiveShexAnalysis(result) => {
                if !result.1.is_empty() {
                    self.analyzer_error = true;
                    let content =
                        format!("Status: Schema is not well-formed. Errors have been identified.",);
                    self.props
                        .on_open_modal
                        .emit(("SHEX PROPERTIES".to_string(), content));
                } else {
                    let number_shapes = result.0.result.shapes.len();
                    let number_prefixes = result.0.result.prefix_map.len();

                    let content = format!(
                        "Status: {}\nNumber of shapes: {}\nNumber of prefixes: {}",
                        result.0.message, number_shapes, number_prefixes
                    );
                    self.props
                        .on_open_modal
                        .emit(("SHEX PROPERTIES".to_string(), content));
                }
                true
            }
            Msg::OpenModal(title, content) => {
                self.props
                    .on_open_modal
                    .emit(("titulo mdal".to_string(), "subtitulo".to_string()));
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            initializeYate();
            initializeYashe();
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="editors-container">
                <div class="yashe-container">
                    <h3 class="title-editor">{"RDF"}</h3>
                    <textarea id="editor-yate"></textarea>
                    <div class="editor-tools">
                        { self.view_parameters(&self.props.rdf_parameters, "rdf") }
                        <button class="analyze-btn" onclick=self.link.callback(|_| Msg::AnalyzeRDF)>{"Analyze"}</button>
                    </div>
                    <div class="shapemap-container">
                        <h3 class="title-editor">{"ShapeMap"}</h3>
                        <textarea
                            class="shapemap-editor"
                            value=&self.props.shapemap_value
                            oninput=self.link.callback(|e: InputData| Msg::UpdateShapeMapValue(e.value))
                        />
                        { self.view_parameters(&self.props.shapemap_parameters, "shapemap") }
                    </div>
                </div>
                <div class="yate-container">
                    <h3 class="title-editor">{"ShEx"}</h3>
                    <textarea id="editor-yashe"></textarea>
                    <div class="editor-tools">
                    { self.view_parameters(&self.props.shex_parameters, "shex") }
                    <button class="analyze-btn" onclick=self.link.callback(|_| Msg::AnalyzeShex)>{"Analyze"}</button>
                    </div>
                    <div style="margin-top: auto;">
                        <button class="button-27" onclick=self.link.callback(|_| Msg::Validate)>
                            { "VALIDATE" }
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}

impl Editor {
    fn view_parameters(&self, options: &Vec<String>, filter: &str) -> Html {
        let select_class = format!("select parameters param-{}", filter);
        let id = format!("select-{}", filter);
        html! {
        <select class={select_class} id={id}>
              { for options.iter().map(|option| html! { <option class="option-parameters" value={option}>{option}</option> }) }
        </select>
        }
    }
}
