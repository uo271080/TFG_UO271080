/// Proporciona funciones y estructuras necesarias para realizar solicitudes HTTP, manejar las respuestas y
/// procesar los datos recibidos.
pub(crate) mod api;
/// Gestiona los ejemplos de datos utilizados dentro de la aplicación.
mod examples_manager;

use crate::components::{editors::Editor, header::Header, modal::Modal, result_table::ResultTable};
use examples_manager::{load_example, ExampleData};
use log::*;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, ToString};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

/// Definición del componente principal App
pub struct App {
    link: ComponentLink<Self>,
    state: State,
    rdf_parameters: Vec<String>,
    shex_parameters: Vec<String>,
    shapemap_parameters: Vec<String>,
}

/// Asignación editor Yate a elemento del DOM
#[wasm_bindgen(inline_js = "
    import YATE from 'perfectkb-yate';
    export function setYate(input) {
        return window.yateInstance.setValue(input);
    }
    ")]
extern "C" {
    fn setYate(input: &str);
}

/// Asignación editor Yashe a elemento del DOM
#[wasm_bindgen(inline_js = "
    export function setYashe(input) {
        return window.yasheInstance.setValue(input);
    }
    ")]
extern "C" {
    fn setYashe(input: &str);
}

/// Recuperación valor editor Yate
#[wasm_bindgen(inline_js = "
    export function getYate() {
        return window.yateInstance.getValue();
    }
    ")]
extern "C" {
    fn getYate() -> String;
}

/// Recuperación valor editor Yashe
#[wasm_bindgen(inline_js = "
    export function getYashe() {
        return window.yasheInstance.getValue();
    }
    ")]
extern "C" {
    fn getYashe() -> String;
}

/// ScroLL automático a elemento del DOM
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

/// Estructura para la información del modal
#[derive(Serialize, Deserialize, Default)]
pub struct ModalInfo {
    title: String,
    content: String,
}

/// Estructura para mantener el estado de la aplicación
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
    show_modal: bool,
    modal_info: ModalInfo,
    is_loading: bool,
}

/// Enum para los mensajes que se pueden enviar al componente
pub enum Msg {
    /// Solicita la validación de los datos actualmente cargados en los editores.
    Validate,
    /// Resultado de una solicitud de validación.
    /// Contiene `api::ValidationResult` con el resultado de la validación y un `String` que puede contener un mensaje de error.
    ValidationResult(api::ValidationResult, String),
    /// Actualiza el valor actual del ShapeMap con el nuevo valor proporcionado.
    /// `String` contiene el nuevo valor de ShapeMap.
    UpdateShapeMapValue(String),
    /// Carga un ejemplo específico.
    /// `String` contiene el identificador del ejemplo a cargar.
    LoadExample(String),
    /// Notifica que un ejemplo ha sido cargado.
    /// `Result<ExampleData, String>` contiene los datos del ejemplo o un mensaje de error en caso de fallo.
    ExampleLoaded(Result<ExampleData, String>),
    /// Cierra la alerta actualmente mostrada, por ejemplo, un mensaje de error.
    CloseAlert,
    /// Abre un modal con información detallada.
    /// `String, String` contiene el título y el contenido del modal, respectivamente.
    OpenModal(String, String),
    /// Cierra el modal actualmente abierto.
    CloseModal,
}

/// Implementación del componente App
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state: State = State {
            filter: Filter::RDF,
            show_result: false,
            show_modal: false,
            scroll_needed: false,
            edit_value: "".into(),
            shapemap_value: "".into(),
            search_text: "".into(),
            validation_result: None,
            api_error: "".into(),
            modal_info: Default::default(),
            is_loading: false,
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
            shex_parameters: vec!["ShExC".to_string(), "ShExJ".to_string()],
            shapemap_parameters: vec!["Compact".to_string(), "JSON".to_string()],
        }
    }

    /// Método para detectar cambios en las propiedades
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    /// Método para manejar los mensajes enviados al componente
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Validate => {
                self.state.api_error = "".to_string();
                self.state.validation_result = Default::default();
                self.state.show_result = true;
                self.state.is_loading = true;
                self.state.scroll_needed = true;
                let rdf_content = getYate();
                let shex_content = getYashe();
                let shapemap_content = self.state.shapemap_value.clone();
                let link = self.link.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let result =
                        api::call_validation_api(rdf_content, shex_content, shapemap_content).await;
                    link.send_message(Msg::ValidationResult(result.0, result.1));
                });
            }
            Msg::CloseAlert => {
                self.state.api_error = "".to_string();
            }
            Msg::OpenModal(title, content) => {
                print!("LLEGO A OPEN MODAL");
                print!("{}", title);
                print!("{}", content);
                self.state.show_modal = true;
                self.state.modal_info = ModalInfo {
                    title: title,
                    content: content,
                };
            }
            Msg::CloseModal => {
                self.state.show_modal = false;
            }
            Msg::UpdateShapeMapValue(new_value) => {
                self.state.shapemap_value = new_value;
                false;
            }
            Msg::ValidationResult(result, error) => {
                self.state.is_loading = false;
                if !error.is_empty() {
                    self.state.api_error = error;
                } else {
                    self.state.validation_result = Some(result);
                    self.state.scroll_needed = true;
                    scrollToElement("result-table");
                }
            }
            Msg::LoadExample(file) => {
                let link = self.link.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let result = load_example(file).await;
                    link.send_message(Msg::ExampleLoaded(result));
                });
            }
            Msg::ExampleLoaded(result) => match result {
                Ok(data) => {
                    setYate(&data.rdf);
                    setYashe(&data.shex);
                    self.state.shapemap_value = data.shapemap;
                }
                Err(error) => {
                    self.state.api_error = error;
                }
            },
        }
        true
    }

    /// Método que se llama después de que el componente se haya renderizado
    fn rendered(&mut self, first_render: bool) {
        if self.state.scroll_needed && !first_render {
            scrollToElement("result-table");
            self.state.scroll_needed = false;
        }
    }

    /// Método para renderizar la vista del componente
    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <div class="todomvc-wrapper">
                <section class="app">
                    <Header on_load_example=self.link.callback(Msg::LoadExample) />
                    <div class="content">
                        <Editor
                            shapemap_value=self.state.shapemap_value.clone()
                            on_update_shapemap_value=self.link.callback(Msg::UpdateShapeMapValue)
                            on_validate=self.link.callback(|_| Msg::Validate)
                            on_open_modal=self.link.callback(|(title, content)| Msg::OpenModal(title, content))
                            rdf_parameters=self.rdf_parameters.clone()
                            shex_parameters=self.shex_parameters.clone()
                            shapemap_parameters=self.shapemap_parameters.clone()
                        />
                        <div class="footer-options">
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
    /// Método para renderizar el resultado de la validación
    fn render_result(&self) -> Html {
        html! {
            <>
                { if self.state.is_loading {
                    html! {
                        <div class="spinner-container">
                            <div class="spinner"></div>
                        </div>
                    }
                } else {
                    html! {
                        <>
                            { if self.state.show_modal {
                                html! {
                                    <>
                                        <div class="reason-modal-overlay" onclick=self.link.callback(|_| Msg::CloseModal)></div>
                                        <Modal
                                            title=self.state.modal_info.title.clone()
                                            content=self.state.modal_info.content.clone()
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
                                        entries={entries.clone()}
                                        search_text={self.state.search_text.clone()}
                                        on_open_modal={self.link.callback(|(title, content)| Msg::OpenModal(title, content))}
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
                }}
            </>
        }
    }
}

/// Enum que contiene los distintos tipos de entrada
#[derive(EnumIter, ToString, Clone, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    RDF,
    ShEx,
    ShapeMap,
}
