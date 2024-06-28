use log::info;
use yew::prelude::*;

/// Componente `Modal` que proporciona una interfaz para mostrar información en un modal.
///
/// Este componente es útil para mostrar detalles o mensajes en una ventana modal que puede ser cerrada. Utiliza propiedades para recibir el contenido y título del modal y un `Callback` para manejar el evento de cierre.
#[derive(Properties, Clone)]
pub struct Props {
    /// Título que se mostrará en la cabecera del modal.
    pub title: String,
    /// Contenido del modal que se mostrará como cuerpo del mismo. Puede incluir texto con saltos de línea para mejor formato.
    pub content: String,
    /// `Callback` que se activa cuando se cierra el modal. No devuelve ningún valor.
    pub on_close: Callback<()>,
}

/// Estructura interna del componente `Modal`.
pub struct Modal {
    link: ComponentLink<Self>,
    props: Props,
}

/// Mensajes utilizados por `Modal` para manejar la interacción del usuario.
pub enum Msg {
    /// Mensaje que indica que se debe cerrar el modal.
    Close,
}

impl Component for Modal {
    type Message = Msg;
    type Properties = Props;

    /// Método para inicializar el componente `Modal`. Configura el estado inicial con las propiedades y enlace proporcionados.
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    /// Actualiza el estado del componente en respuesta a acciones del usuario.
    /// Maneja el cierre del modal cuando se recibe el mensaje `Close`.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Close => {
                self.props.on_close.emit(());
                true
            }
        }
    }

    /// Maneja los cambios en las propiedades pasadas al componente.
    /// Si las propiedades cambian, actualiza el estado interno y devuelve true para re-renderizar el componente.
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    /// Renderiza el componente como HTML.
    /// Devuelve el marcado HTML para el modal, incluyendo el título, cuerpo y un botón de cierre.
    fn view(&self) -> Html {
        info!("desde modal!!!!!!!!");
        info!("{}", &self.props.content);

        let reason_lines: Vec<&str> = self.props.content.split('\n').collect();

        html! {
            <div class="reason-modal">
                <h2>{ &self.props.title }</h2>
                <div class="reason-modal-body">
                    { for reason_lines.iter().map(|line| html! { <p>{ line }</p> }) }
                </div>
                <button class="reason-modal-button" onclick=self.link.callback(|_| Msg::Close)>{ "Cerrar" }</button>
            </div>
        }
    }
}
