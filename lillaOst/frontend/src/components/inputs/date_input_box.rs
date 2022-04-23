use chrono::prelude::*;
use yew::prelude::*;

use gloo_console::error;

use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;

pub enum MsgDateInputBox {
    ChangeEvent(String),
}

#[derive(Clone, Properties, PartialEq)]
pub struct PropsDateInputBox {
    pub id: u32,
    pub date: Date<Utc>,
    pub callback: Callback<Date<Utc>>,
}

pub struct DateInputBox {
    id: u32,
    pub date: Date<Utc>,
    callback: Callback<Date<Utc>>,
}

impl Component for DateInputBox {
    type Message = MsgDateInputBox;
    type Properties = PropsDateInputBox;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: ctx.props().id,
            date: ctx.props().date,
            callback: ctx.props().callback.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgDateInputBox::ChangeEvent(s) => {
                match NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
                    Ok(o) => {
                        let p = Utc::from_local_date(&Utc, &o);
                        if let chrono::LocalResult::Single(s) = p {
                            self.callback.emit(s);
                            return true;
                        }
                    }
                    Err(e) => error!(format!("Can't parse `{}`, error {:#?}", s, e)),
                };
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.callback = ctx.props().callback.clone();
        self.id = ctx.props().id;
        self.date = ctx.props().date;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let input_element = html! {
            <input
                class="input"
                type="date"
                id={format!("{}_date_input_field", self.id)}
                value={ self.date.format("%Y-%m-%d").to_string() }
                onchange={ ctx.link().callback( | e: web_sys::Event | {
                        let event_target = e.target().unwrap_throw();
                        let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
                        MsgDateInputBox::ChangeEvent( target.value() )
                    })
                }
            />
        };

        html! {
            <div class="field">
                <div class="control">
                    {input_element}
                </div>
            </div>
        }
    }
}
