use log::info;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    pub title: String,
    pub content: String,
    pub on_close: Callback<()>,
}

pub struct Modal {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    Close,
}

impl Component for Modal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Close => {
                self.props.on_close.emit(());
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

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
