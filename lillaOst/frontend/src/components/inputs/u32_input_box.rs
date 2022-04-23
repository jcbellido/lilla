use yew::prelude::*;

use crate::web_sys_utils::input_get_value_from_input_event;

pub enum MsgU32InputBox {
    InputChanged(String),
}

#[derive(Clone, Properties, PartialEq)]
pub struct PropsU32InputBox {
    pub id: u32,
    pub label: String,
    pub value: u32,
    pub callback: Callback<u32>,
}

pub struct U32InputBox {
    id: u32,
    label: String,
    value: u32,
    callback: Callback<u32>,
}

impl Component for U32InputBox {
    type Message = MsgU32InputBox;
    type Properties = PropsU32InputBox;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: ctx.props().id,
            label: ctx.props().label.clone(),
            value: ctx.props().value,
            callback: ctx.props().callback.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgU32InputBox::InputChanged(s) => {
                if s.is_empty() {
                    self.callback.emit(0);
                } else {
                    match str::parse::<u32>(&s) {
                        Ok(b) => {
                            self.callback.emit(b);
                        }
                        Err(_) => {
                            self.callback.emit(0);
                        }
                    }
                }
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.callback = ctx.props().callback.clone();
        self.id = ctx.props().id;
        self.label = ctx.props().label.clone();
        self.value = ctx.props().value;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let input_element = html! {
            <input
                class="input"
                type="number"
                id={format!("{}_u32_input_field", self.id)}
                name="quantity"
                placeholder={"insert a number here"}
                value={ self.value.to_string() }
                min="0"
                oninput={ctx.link().callback(|e: InputEvent| MsgU32InputBox::InputChanged( input_get_value_from_input_event (e)))}
            />
        };

        html! {
            <div class="field">
                <label class="label">{ &self.label }</label>
                <div class="control">
                    {input_element}
                </div>
            </div>
        }
    }
}
