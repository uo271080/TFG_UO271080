use yew::prelude::*;
use crate::app::api::ShapeMapEntry;

#[derive(Properties, Clone)]
pub struct Props {
    pub entries: Vec<ShapeMapEntry>,
    pub search_text: String,
    pub on_open_modal: Callback<(String, String, String)>,
}

pub struct ResultTable {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    OpenModal(String, String, String),
}

impl Component for ResultTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenModal(node, shape, _status) => {
                let reason = self.props.entries.iter()
                    .find(|entry| entry.node == node && entry.shape == shape)
                    .map(|entry| entry.reason.clone())
                    .unwrap_or_default();
                self.props.on_open_modal.emit((node, shape, reason));
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="result" id="result">
                <table>
                    <tr>
                        <th>{"Node"}</th>
                        <th>{"Shape"}</th>
                        <th>{"Status"}</th>
                        <th class="details-col">{"Details"}</th>
                    </tr>
                    { self.props.entries.iter()
                        .filter(|entry| entry.node.contains(&self.props.search_text))
                        .map(|entry| self.view_entry(entry))
                        .collect::<Html>() 
                    }
                </table>
            </div>
        }
    }
}

impl ResultTable {
    fn view_entry(&self, entry: &ShapeMapEntry) -> Html {
        let cloned_entry = entry.clone();
        html! {
            <tr class={ if entry.status == "Valid" { "valid" } else { "invalid" } }>
                <td>{ &entry.node }</td>
                <td>{ &entry.shape }</td>
                <td class="details-row">{ &entry.status }</td>
                <td>
                    <button type="button" class="btn" onclick=self.link.callback(move |_| Msg::OpenModal(cloned_entry.node.clone(), cloned_entry.shape.clone(), cloned_entry.status.clone()))>
                        { "Show" }
                    </button>
                </td>
            </tr>
        }
    }
}
