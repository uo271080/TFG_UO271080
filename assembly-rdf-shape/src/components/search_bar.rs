use yew::prelude::*;

/// Componente `SearchBar` que proporciona una interfaz de usuario para la entrada de búsquedas.
///
/// Este componente renderiza una barra de búsqueda que captura la entrada del usuario y emite
/// el texto introducido a través de un `Callback`. Se utiliza comúnmente para recibir
/// entradas de texto y realizar acciones de búsqueda o filtrado basadas en esta entrada.
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    /// Callback que se activa cuando se introduce texto en la barra de búsqueda.
    /// El `String` emitido representa el texto que el usuario ha introducido.
    pub on_search: Callback<String>,
}

/// Estructura interna del componente `SearchBar`.
pub struct SearchBar {
    /// Enlace para conectar el componente con el sistema de Yew y permitir la comunicación
    /// entre el componente y su entorno.
    link: ComponentLink<Self>,
    /// Propiedades pasadas al componente. Incluye el `Callback` para manejar la entrada de búsqueda.
    props: Props,
}

/// Enumeración de mensajes que el componente `SearchBar` puede procesar.
pub enum Msg {
    /// Mensaje que se envía cuando el usuario realiza una búsqueda.
    /// Contiene el texto que el usuario ha introducido en la barra de búsqueda.
    Search(String),
}

impl Component for SearchBar {
    type Message = Msg;
    type Properties = Props;

    /// Método para inicializar el componente. Configura el estado inicial del componente
    /// con las propiedades y enlace proporcionados.
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    /// Método que gestiona los mensajes enviados al componente.
    /// Actualiza el estado del componente basado en la acción del usuario y notifica a otros componentes
    /// mediante callbacks si es necesario.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Search(value) => {
                self.props.on_search.emit(value);
                true
            }
        }
    }

    /// Método para manejar cambios en las propiedades pasadas al componente.
    /// Si las propiedades cambian, actualiza el estado interno y devuelve true para re-renderizar el componente.
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    /// Método que renderiza el componente como HTML.
    /// Devuelve el marcado HTML para la interfaz de usuario de la barra de búsqueda.
    fn view(&self) -> Html {
        html! {
        <input type="text"
        class="search"
        placeholder="Buscar..."
        oninput=self.link.callback(|e: InputData| Msg::Search(e.value)) />        }
    }
}
