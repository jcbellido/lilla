use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlSelectElement;
use web_sys::InputEvent;
use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};

use crate::event_bus::{EventBus, Request};
use crate::web_sys_utils::text_area_get_value_from_input_event;

use super::f64_input_box::F64InputBox;

use ost::context_remote_async::AsyncRemoteMonolith;
use ost::event::{Event as ost_Event, EventType as ost_EventType};
use ost::person::Person as ost_Person;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum EventType {
    Bath = 0,
    Medicine = 1,
    Sleep = 2,
    Awake = 3,
    Note = 4,
    Temperature = 5,
}

#[derive(Clone, Debug)]
pub enum MsgInsertEvent {
    OstEventTypeChanged { event_type: ost_EventType },
    UpdateNote { note: String },
    UpdateFloat(f64),
    AddEvent,
    CallFinished,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::Bath => write!(f, "{}", r#"Bath"#),
            EventType::Medicine => write!(f, "{}", r#"Medicine"#),
            EventType::Sleep => write!(f, "{}", r#"Sleep"#),
            EventType::Awake => write!(f, "{}", r#"Awake"#),
            EventType::Note => write!(f, "{}", r#"Note"#),
            EventType::Temperature => write!(f, "{}", r#"Temperature"#),
        }
    }
}

#[derive(Clone, Properties)]
pub struct PropsInsertEvent {
    pub ost_event: Option<Rc<RefCell<Box<dyn ost_Event>>>>,
    pub ost_person: Option<Rc<Box<dyn ost_Person>>>,
}

impl PartialEq for PropsInsertEvent {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

pub struct InsertEvent {
    ost_event_type: ost_EventType,
    props: PropsInsertEvent,
    event_bus: Dispatcher<EventBus>,
}

impl Component for InsertEvent {
    type Message = MsgInsertEvent;
    type Properties = PropsInsertEvent;

    fn create(ctx: &Context<Self>) -> Self {
        let mut ost_event_type: ost_EventType = ost_EventType::Note(String::default());

        if ctx.props().ost_event.is_some() {
            ost_event_type = ctx.props().ost_event.as_ref().unwrap().borrow().event();
        }

        Self {
            ost_event_type,
            props: ctx.props().clone(),
            event_bus: EventBus::dispatcher(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgInsertEvent::AddEvent => {
                if let ost_EventType::Temperature(t) = self.ost_event_type {
                    if t <= 30.0 || t >= 45.0 {
                        return false;
                    }
                }
                let p_cloned = self.props.ost_person.as_ref().unwrap().clone();
                let ev_cloned = self.ost_event_type.clone();
                ctx.link().send_future(async move {
                    let remote = AsyncRemoteMonolith {};
                    let _ = remote.add_event(&p_cloned, ev_cloned).await;
                    MsgInsertEvent::CallFinished
                });
                self.ost_event_type = ost_EventType::Note(String::default());
                return true;
            }
            MsgInsertEvent::UpdateNote { note } => {
                match self.ost_event_type {
                    ost_EventType::Medicine(_) => {
                        self.ost_event_type = ost_EventType::Medicine(note)
                    }
                    ost_EventType::Note(_) => self.ost_event_type = ost_EventType::Note(note),
                    _ => {}
                };
                self.update_existing_event();
            }
            MsgInsertEvent::OstEventTypeChanged { event_type } => {
                self.ost_event_type = event_type;
                self.update_existing_event();
            }
            MsgInsertEvent::UpdateFloat(f) => {
                #[allow(clippy::single_match)]
                match self.ost_event_type {
                    ost_EventType::Temperature(_) => {
                        self.ost_event_type = ost_EventType::Temperature(f)
                    }
                    _ => {}
                };
                self.update_existing_event();
            }
            MsgInsertEvent::CallFinished => {
                self.event_bus.send(Request::EventBusMsg(
                    "Insert events, call finished".to_owned(),
                ));
                return false;
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props = ctx.props().clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let per_event_control = match &self.ost_event_type {
            ost_EventType::Bath => html!(),
            ost_EventType::Medicine(s) => html! {
                <div class="field">
                    <div class="control">
                        <textarea class="textarea is-small" placeholder=""
                            maxlength="128"
                            value={ s.clone() }
                            oninput={ctx.link().callback(move |e: InputEvent| MsgInsertEvent::UpdateNote { note: text_area_get_value_from_input_event (e)})}
                            ></textarea>
                    </div>
                </div>
            },
            ost_EventType::Sleep => html!(),
            ost_EventType::Awake => html!(),
            ost_EventType::Note(s) => html! {
                <div class="field">
                    <div class="control">
                        <textarea class="textarea is-small" placeholder=""
                            maxlength="128"
                            value={ s.clone() }
                            oninput={ctx.link().callback(move |e: InputEvent| MsgInsertEvent::UpdateNote { note: text_area_get_value_from_input_event (e)})}
                            ></textarea>
                    </div>
                </div>
            },
            ost_EventType::Temperature(t) => html! {
                <div>
                    <F64InputBox id=0 label="Input Temperature" value={t.clone()} callback={
                        ctx.link().callback( MsgInsertEvent::UpdateFloat )
                    } />
                </div>
            },
        };

        let add_event_button = if self.props.ost_event.is_none() {
            let allow_add_event = match self.ost_event_type {
                ost_EventType::Temperature(t) => t > 30.0 && t < 45.0,
                _ => true,
            };
            html! {
            <div class="field">
                <div class="control">
                    <button class={ if allow_add_event {"button is-link"} else { "button is-dark" } } onclick={ctx.link().callback(|_| MsgInsertEvent::AddEvent)}>{"Add Event"}</button>
                </div>
            </div>}
        } else {
            html!()
        };

        html! {
        <div class={"block"}>
            <div class={"field"}>
                <label class={"label"}>{"Pick your event"}</label>
                { self.select_event(ctx) }
            </div>
            {per_event_control}
            {add_event_button}
        </div>
        }
    }
}

impl InsertEvent {
    fn select_event(&self, ctx: &Context<Self>) -> Html {
        let events_variants = vec![
            ost_EventType::Bath,
            ost_EventType::Medicine(String::default()),
            ost_EventType::Sleep,
            ost_EventType::Awake,
            ost_EventType::Note(String::default()),
            ost_EventType::Temperature(36.5_f64),
        ];

        html! {
            <div class="select">
                <select oninput={ ctx.link().callback( move | input_event: InputEvent |
                    {
                        let event: Event = input_event.dyn_into().unwrap_throw();
                        let event_target = event.target().unwrap_throw();
                        let target: HtmlSelectElement = event_target.dyn_into().unwrap_throw();

                        match target.value().as_str() {
                            "Awake" => MsgInsertEvent::OstEventTypeChanged { event_type: ost_EventType::Awake },
                            "Bath" => MsgInsertEvent::OstEventTypeChanged { event_type: ost_EventType::Bath },
                            "Medicine" => MsgInsertEvent::OstEventTypeChanged { event_type: ost_EventType::Medicine(String::default()) },
                            "Note" => MsgInsertEvent::OstEventTypeChanged { event_type: ost_EventType::Note(String::default()) },
                            "Sleep" => MsgInsertEvent::OstEventTypeChanged { event_type: ost_EventType::Sleep },
                            "Temperature" => MsgInsertEvent::OstEventTypeChanged { event_type: ost_EventType::Temperature(36.5_f64) },
                            _ => MsgInsertEvent::OstEventTypeChanged { event_type: ost_EventType::Bath }
                        }
                    })}
                >
                {
                    for events_variants.iter().map(|e| {
                        if self.ost_event_type == *e {
                            html! { <option selected={true}> {e} </option> }
                        } else {
                            html! { <option selected={false}> {e} </option> }
                        }
                    })
                }
                </select>
            </div>
        }
    }

    fn update_existing_event(&mut self) {
        if self.props.ost_event.is_none() {
            return;
        }

        let c_event = self.ost_event_type.clone();

        if let Some(event) = &self.props.ost_event {
            #[allow(clippy::clone_on_copy)]
            let ts = event.borrow().time_stamp().clone();
            event.borrow_mut().modify_event(ts, c_event);
        }
    }
}
