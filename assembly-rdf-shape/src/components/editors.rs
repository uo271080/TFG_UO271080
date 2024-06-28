use crate::app::api::{self, InfoRdfResponse, InfoShexResponse};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

/// Obtiene el valor actual del editor YATE.
#[wasm_bindgen(inline_js = "
export function getYate() {
    return window.yateInstance.getValue();
}
")]
extern "C" {
    fn getYate() -> String;
}

/// Obtiene el valor actual del editor YASHE.
#[wasm_bindgen(inline_js = "
export function getYashe() {
    return window.yasheInstance.getValue();
}
")]
extern "C" {
    fn getYashe() -> String;
}

/// Inicializa el editor YATE en la página.
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

/// Inicializa el editor YASHE en la página.
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

/// Propiedades para configurar el componente `Editor`.
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

/// Componente `Editor` que maneja editores de texto para RDF,ShEx y ShapeMap.
pub struct Editor {
    link: ComponentLink<Self>,
    props: Props,
    analyzer_error: bool,
}

/// Mensajes internos del componente para manejar la lógica de la interfaz.
pub enum Msg {
    /// Actualiza valor del editor ShapeMap
    UpdateShapeMapValue(String),
    /// Lanza proceso de validación dado el valor de cada entrada
    Validate,
    /// Lanza proceso de análisis dada la entrada RDF
    AnalyzeRDF,
    /// Lanza proceso de análisis dada la entrada ShEx
    AnalyzeShex,
    /// Recibe respuesta del proceso de análisis ShEx
    ReceiveShexAnalysis((InfoShexResponse, String)),
    /// Recibe respuesta del proceso de análisis RDF
    ReceiveRDFAnalysis((InfoRdfResponse, String)),
}

impl Component for Editor {
    type Message = Msg;
    type Properties = Props;

    /// Crea una nueva instancia del componente `Editor`.
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            analyzer_error: false,
        }
    }

    /// Actualiza el estado del componente basado en mensajes recibidos.
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
                    let content = format!("Status: RDF is not well-formed.",);
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
                    let content = format!("Status: Schema is not well-formed.");
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
        }
    }

    /// Gestiona cambios en las propiedades del componente.
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    /// Se llama después de que el componente es renderizado.
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            initializeYate();
            initializeYashe();
        }
    }

    /// Renderiza el HTML del componente.
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
    /// Renderiza las opciones de parámetros para cada editor.
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
