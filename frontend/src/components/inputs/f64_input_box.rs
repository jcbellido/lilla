use yew::prelude::*;

use crate::web_sys_utils::input_get_value_from_input_event;

pub enum MsgF64InputBox {
    InputChanged(String),
}

#[derive(Clone, Properties, PartialEq)]
pub struct PropsF64InputBox {
    pub id: u32,
    pub label: String,
    pub value: f64,
    pub callback: Callback<f64>,
}

pub struct F64InputBox {
    id: u32,
    label: String,
    value: f64,
    callback: Callback<f64>,
}

impl Component for F64InputBox {
    type Message = MsgF64InputBox;
    type Properties = PropsF64InputBox;

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
            MsgF64InputBox::InputChanged(s) => {
                if s.is_empty() {
                    self.callback.emit(0_f64);
                } else {
                    match str::parse::<f64>(&s) {
                        Ok(b) => {
                            self.callback.emit(b);
                        }
                        Err(_) => {
                            self.callback.emit(0_f64);
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
                id={format!("{}_f64_input_field", self.id)}
                name="quantity"
                placeholder={"insert a number here"}
                value={ self.value.to_string() }
                min="30.0" max="45.0"
                oninput={ctx.link().callback(|e: InputEvent| MsgF64InputBox::InputChanged( input_get_value_from_input_event (e) ) )}
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
