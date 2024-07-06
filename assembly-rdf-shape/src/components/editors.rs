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
    pub rdf_format: String,
    pub shex_format: String,
    pub shapemap_format: String,
    pub on_update_shapemap_value: Callback<String>,
    pub on_validate: Callback<(String, String, String)>,
    pub on_open_modal: Callback<(String, Html)>,
    pub rdf_parameters: Vec<String>,
    pub shex_parameters: Vec<String>,
    pub shapemap_parameters: Vec<String>,
    pub example_loaded: bool,
    pub reset_example_loaded: Callback<()>,
}

/// Componente `Editor` que maneja editores de texto para RDF, ShEx y ShapeMap.
pub struct Editor {
    link: ComponentLink<Self>,
    props: Props,
    analyzer_error: bool,
    rdf_param_selected: String,
    shex_param_selected: String,
    shapemap_param_selected: String,
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
    /// Actualiza formato seleccionado para RDF
    UpdateRdfParamSelected(String),
    /// Actualiza formato seleccionado para ShEx
    UpdateShexParamSelected(String),
    /// Actualiza formato seleccionado para ShapeMap
    UpdateShapeMapParamSelected(String),
}

impl Component for Editor {
    type Message = Msg;
    type Properties = Props;

    /// Crea una nueva instancia del componente `Editor`.
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let rdf_param_selected = props.rdf_format.clone();
        let shex_param_selected = props.shex_format.clone();
        let shapemap_param_selected = props.shapemap_format.clone();

        Self {
            link,
            props,
            analyzer_error: false,
            rdf_param_selected,
            shex_param_selected,
            shapemap_param_selected,
        }
    }

    /// Actualiza el estado del componente basado en mensajes recibidos.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateShapeMapValue(value) => {
                self.props.on_update_shapemap_value.emit(value);
                false
            }
            Msg::Validate => {
                let rdf_param_selected = self.rdf_param_selected.clone();
                let shex_param_selected = self.shex_param_selected.clone();
                let shapemap_param_selected = self.shapemap_param_selected.clone();

                self.props.on_validate.emit((
                    rdf_param_selected,
                    shex_param_selected,
                    shapemap_param_selected,
                ));
                false
            }
            Msg::AnalyzeRDF => {
                let link = self.link.clone();
                let rdf_param_selected = self.rdf_param_selected.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let content = api::call_rdf_info_api(getYate(), rdf_param_selected).await;
                    link.send_message(Msg::ReceiveRDFAnalysis(content));
                });
                false
            }
            Msg::ReceiveRDFAnalysis(result) => {
                let content = if !result.1.is_empty() {
                    html! {
                        <>
                            <p>{"Status: RDF is not well-formed."}</p>
                        </>
                    }
                } else {
                    let number_statements = result.0.result.number_of_statements;
                    html! {
                        <>
                            <p>{format!("Status: {}", result.0.message)}</p>
                            <p>{format!("Number of statements: {}", number_statements)}</p>
                        </>
                    }
                };

                self.props
                    .on_open_modal
                    .emit(("RDF PROPERTIES".to_string(), content));
                false
            }
            Msg::AnalyzeShex => {
                let link = self.link.clone();
                let shex_param_selected = self.shex_param_selected.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let content: (InfoShexResponse, String) =
                        api::call_shex_info_api(getYashe(), shex_param_selected).await;
                    link.send_message(Msg::ReceiveShexAnalysis(content));
                });
                false
            }
            Msg::ReceiveShexAnalysis(result) => {
                let content = if !result.1.is_empty() {
                    self.analyzer_error = true;
                    html! {
                        <>
                            <p>{"Status: Schema is not well-formed."}</p>
                        </>
                    }
                } else {
                    let number_shapes = result.0.result.shapes.len();
                    let number_prefixes = result.0.result.prefix_map.len();
                    html! {
                        <>
                            <p>{format!("Status: {}", result.0.message)}</p>
                            <p>{format!("Number of shapes: {}", number_shapes)}</p>
                            <p>{format!("Number of prefixes: {}", number_prefixes)}</p>
                        </>
                    }
                };

                self.props
                    .on_open_modal
                    .emit(("SHEX PROPERTIES".to_string(), content));
                false
            }
            Msg::UpdateRdfParamSelected(value) => {
                self.rdf_param_selected = value;
                true
            }
            Msg::UpdateShexParamSelected(value) => {
                self.shex_param_selected = value;
                true
            }
            Msg::UpdateShapeMapParamSelected(value) => {
                self.shapemap_param_selected = value;
                true
            }
        }
    }

    /// Gestiona cambios en las propiedades del componente.
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.example_loaded {
            self.rdf_param_selected = props.rdf_format.clone();
            self.shex_param_selected = props.shex_format.clone();
            self.shapemap_param_selected = props.shapemap_format.clone();
            self.props.reset_example_loaded.emit(());
        }
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
            <div id="editors-container" class="editors-container">
                <div class="yashe-container">
                    <h3 class="title-editor">{"RDF"}</h3>
                    <textarea id="editor-yate"></textarea>
                    <div class="editor-tools">
                        { self.view_parameters(&self.props.rdf_parameters, "rdf") }
                        <button id="analyze-rdf" class="analyze-btn" onclick=self.link.callback(|_| Msg::AnalyzeRDF)>{"Analyze"}</button>
                    </div>
                    <div class="shapemap-container">
                        <h3 class="title-editor">{"ShapeMap"}</h3>
                        <textarea
                            id="shapemap-editor"
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
                        <button id="analyze-shex" class="analyze-btn" onclick=self.link.callback(|_| Msg::AnalyzeShex)>{"Analyze"}</button>
                    </div>
                    <div style="margin-top: auto;">
                        <button id="validate-btn" class="button-27" onclick=self.link.callback(|_| Msg::Validate)>
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
        let filter = filter.to_string();
        let selected_value = match filter.as_str() {
            "rdf" => &self.rdf_param_selected,
            "shex" => &self.shex_param_selected,
            "shapemap" => &self.shapemap_param_selected,
            _ => &String::new(),
        };

        html! {
            <select
                title={"Format"}
                class={select_class}
                id={id}
                onchange=self.link.callback(move |e: ChangeData| {
                    if let ChangeData::Select(select) = e {
                        match filter.as_str() {
                            "rdf" => Msg::UpdateRdfParamSelected(select.value()),
                            "shex" => Msg::UpdateShexParamSelected(select.value()),
                            "shapemap" => Msg::UpdateShapeMapParamSelected(select.value()),
                            _ => unreachable!(),
                        }
                    } else {
                        match filter.as_str() {
                            "rdf" => Msg::UpdateRdfParamSelected(String::new()),
                            "shex" => Msg::UpdateShexParamSelected(String::new()),
                            "shapemap" => Msg::UpdateShapeMapParamSelected(String::new()),
                            _ => unreachable!(),
                        }
                    }
                })
            >
                { for options.iter().map(|option| {
                    html! {
                        <option
                            class="option-parameters"
                            value={option}
                            selected={selected_value == option}
                        >
                            {option}
                        </option>
                    }
                }) }
            </select>
        }
    }
}
