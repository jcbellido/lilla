use std::cell::RefCell;
use std::rc::Rc;

use std::fmt;

use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlSelectElement;
use web_sys::InputEvent;
use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};

use crate::event_bus::{EventBus, Request};

use ost::context_remote_async::AsyncRemoteMonolith;
use ost::expulsion::{Expulsion as ost_Expulsion, ExpulsionDegree};
use ost::person::Person as ost_Person;

#[derive(Debug, PartialEq, Clone)]
pub enum ExpulsionAmount {
    Clean = 0,
    Pee = 1,
    Shart = 2,
    Poop = 3,
    Pooplosion = 4,
}

#[derive(Clone, Debug)]
pub enum MsgInsertExpulsions {
    SelectChanged { e: ExpulsionAmount },
    AddExpulsion,
    CallFinished,
}

impl fmt::Display for ExpulsionAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpulsionAmount::Clean => write!(f, "{}", r#"Clean"#),
            ExpulsionAmount::Pee => write!(f, "{}", r#"Pee"#),
            ExpulsionAmount::Shart => write!(f, "{}", r#"Shart"#),
            ExpulsionAmount::Poop => write!(f, "{}", r#"Poop"#),
            ExpulsionAmount::Pooplosion => write!(f, "{}", r#"Pooplosion"#),
        }
    }
}

#[derive(Clone, Properties)]
pub struct PropsInsertExpulsions {
    pub ost_expulsion: Option<Rc<RefCell<Box<dyn ost_Expulsion>>>>,
    pub ost_person: Option<Rc<Box<dyn ost_Person>>>,
}

impl PartialEq for PropsInsertExpulsions {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

pub struct InsertExpulsions {
    currently_selected: ExpulsionAmount,
    props: PropsInsertExpulsions,
    event_bus: Dispatcher<EventBus>,
}

impl Component for InsertExpulsions {
    type Message = MsgInsertExpulsions;
    type Properties = PropsInsertExpulsions;

    fn create(ctx: &Context<Self>) -> Self {
        let mut currently_selected = ExpulsionAmount::Pee;
        if let Some(existing_expulsion) = ctx.props().ost_expulsion.as_ref() {
            currently_selected = match existing_expulsion.borrow().degree() {
                ExpulsionDegree::Clean => ExpulsionAmount::Clean,
                ExpulsionDegree::Pee => ExpulsionAmount::Pee,
                ExpulsionDegree::Shart => ExpulsionAmount::Shart,
                ExpulsionDegree::Poopies => ExpulsionAmount::Poop,
                ExpulsionDegree::Pooplosion => ExpulsionAmount::Pooplosion,
            }
        }

        Self {
            currently_selected,
            props: ctx.props().clone(),
            event_bus: EventBus::dispatcher(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgInsertExpulsions::SelectChanged { e } => {
                self.currently_selected = e;
                self.update_existing_expulsion();
            }
            MsgInsertExpulsions::AddExpulsion => {
                let p_cloned = self.props.ost_person.as_ref().unwrap().clone();
                let expulsion_cloned = self.as_expulsion_degree();
                ctx.link().send_future(async move {
                    let remote = AsyncRemoteMonolith {};
                    let _ = remote.add_expulsion(&p_cloned, expulsion_cloned).await;
                    MsgInsertExpulsions::CallFinished
                });
                self.currently_selected = ExpulsionAmount::Pee;
            }
            MsgInsertExpulsions::CallFinished => {
                self.event_bus.send(Request::EventBusMsg(
                    "Insert expulsions, call finished".to_owned(),
                ));
                self.currently_selected = ExpulsionAmount::Pee;
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props = ctx.props().clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let add_new_expulsion = if self.props.ost_expulsion.is_none() {
            html! {
                <div class="field is-grouped">
                    <div class="control">
                        <button class="button is-link" onclick={ctx.link().callback(|_| MsgInsertExpulsions::AddExpulsion)} >{"Add Nappy Change"}</button>
                    </div>
                </div>
            }
        } else {
            html! {}
        };

        html! {
            <div class="block">
                <div class="field">
                    <label class="label">{"How things went?"}</label>
                    { self.select_expulsion(ctx) }
                </div>
                { add_new_expulsion }
            </div>
        }
    }
}

impl InsertExpulsions {
    fn update_existing_expulsion(&self) {
        if self.props.ost_expulsion.is_none() {
            return;
        }
        let degree = self.as_expulsion_degree();

        if let Some(expulsion) = &self.props.ost_expulsion {
            #[allow(clippy::clone_on_copy)]
            let ts = expulsion.borrow().time_stamp().clone();
            expulsion.borrow_mut().modify_expulsion(degree, ts);
        }
    }

    fn as_expulsion_degree(&self) -> ExpulsionDegree {
        match self.currently_selected {
            ExpulsionAmount::Clean => ExpulsionDegree::Clean,
            ExpulsionAmount::Pee => ExpulsionDegree::Pee,
            ExpulsionAmount::Shart => ExpulsionDegree::Shart,
            ExpulsionAmount::Poop => ExpulsionDegree::Poopies,
            ExpulsionAmount::Pooplosion => ExpulsionDegree::Pooplosion,
        }
    }

    fn select_expulsion(&self, ctx: &Context<Self>) -> Html {
        let expulsion_variants = vec![
            ExpulsionAmount::Clean,
            ExpulsionAmount::Pee,
            ExpulsionAmount::Shart,
            ExpulsionAmount::Poop,
            ExpulsionAmount::Pooplosion,
        ];

        html! {
            <div class="select">
                <select oninput={ ctx.link().callback( move | input_event: InputEvent |
                    {
                        let event: Event = input_event.dyn_into().unwrap_throw();
                        let event_target = event.target().unwrap_throw();
                        let target: HtmlSelectElement = event_target.dyn_into().unwrap_throw();

                        match target.value().as_str() {
                            "Clean" => MsgInsertExpulsions::SelectChanged { e: ExpulsionAmount::Clean },
                            "Pee" => MsgInsertExpulsions::SelectChanged { e: ExpulsionAmount::Pee },
                            "Shart" => MsgInsertExpulsions::SelectChanged { e: ExpulsionAmount::Shart },
                            "Poop" => MsgInsertExpulsions::SelectChanged { e: ExpulsionAmount::Poop },
                            "Pooplosion" => MsgInsertExpulsions::SelectChanged { e: ExpulsionAmount::Pooplosion },
                            _ => {
                                MsgInsertExpulsions::SelectChanged { e: ExpulsionAmount::Pee }
                            }
                        }
                    })}
                >
                {
                    for expulsion_variants.iter().map(|e| {
                        if self.currently_selected == *e {
                            html! { <option selected={true} value={format!("{}", e)} > {e} </option> }
                        } else {
                            html! { <option value={format!("{}", e)} > {e} </option> }
                        }
                    })
                }
                </select>
            </div>
        }
    }
}
