use std::cell::RefCell;
use std::rc::Rc;

use chrono::{prelude::*, Local, Utc};

use gloo_console::{error, warn};
use ost::context_remote_async::AsyncRemoteMonolith;
use web_sys::InputEvent;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::web_sys_utils::input_get_value_from_input_event;

use super::routes::LillaOstRoutes;
use crate::components::{
    insert_event::InsertEvent, insert_expulsions::InsertExpulsions, insert_feedings::InsertFeedings,
};

use ost::event::Event as ost_Event;
use ost::event_key::{EventType, OstEventKey};
use ost::expulsion::Expulsion as ost_Expulsion;
use ost::feed::Feed as ost_Feed;

pub enum MsgPageDetails {
    // Deletion block
    RequestDeletionConfirmation,
    PerformEventDeletion,
    CancelDeletion,
    // Date and time update
    UpdateDate { date: String },
    UpdateTime { time: String },
    // async load events
    EventLoaded(Option<Box<dyn ost_Event>>),
    ExpulsionLoaded(Option<Box<dyn ost_Expulsion>>),
    FeedingLoaded(Option<Box<dyn ost_Feed>>),
    // async update events
    UpdatedAndThenNavigateHome,
    NavigateHome,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub ost_event_key: OstEventKey,
}

pub struct PageDetails {
    event_found_in_db: bool,
    is_loading: bool,
    ost_existing_event: Vec<Rc<RefCell<Box<dyn ost_Event>>>>,
    ost_existing_expulsion: Vec<Rc<RefCell<Box<dyn ost_Expulsion>>>>,
    ost_existing_feeding: Vec<Rc<RefCell<Box<dyn ost_Feed>>>>,
    props: Props,
    show_delete_dialog: bool,
}

impl Component for PageDetails {
    type Message = MsgPageDetails;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        #[allow(clippy::clone_on_copy)]
        let event_key = ctx.props().ost_event_key.clone();

        match ctx.props().ost_event_key.t {
            EventType::Event => {
                ctx.link().send_future(async move {
                    let remote = AsyncRemoteMonolith {};
                    let event = remote.get_event_by_key(&event_key).await;
                    MsgPageDetails::EventLoaded(event)
                });
            }
            EventType::Expulsion => {
                ctx.link().send_future(async move {
                    let remote = AsyncRemoteMonolith {};
                    let expulsion = remote.get_expulsion_by_key(&event_key).await;
                    MsgPageDetails::ExpulsionLoaded(expulsion)
                });
            }
            EventType::Feed => {
                ctx.link().send_future(async move {
                    let remote = AsyncRemoteMonolith {};
                    let feeding = remote.get_feeding_by_key(&event_key).await;
                    MsgPageDetails::FeedingLoaded(feeding)
                });
            }
        }
        Self {
            props: ctx.props().clone(),
            event_found_in_db: false,
            show_delete_dialog: false,
            ost_existing_event: vec![],
            ost_existing_expulsion: vec![],
            ost_existing_feeding: vec![],
            is_loading: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgPageDetails::RequestDeletionConfirmation => self.show_delete_dialog = true,
            MsgPageDetails::PerformEventDeletion => {
                if !self.event_found_in_db {
                    return false;
                }

                #[allow(clippy::clone_on_copy)]
                let k = self.props.ost_event_key.clone();

                match &self.props.ost_event_key.t {
                    EventType::Event => {
                        ctx.link().send_future(async move {
                            let remote = AsyncRemoteMonolith {};
                            let ev = remote.get_event_by_key(&k).await;
                            if let Some(event) = ev {
                                let _ = remote.remove_event(event).await;
                            }
                            MsgPageDetails::UpdatedAndThenNavigateHome
                        });
                    }
                    EventType::Expulsion => {
                        ctx.link().send_future(async move {
                            let remote = AsyncRemoteMonolith {};
                            let ex = remote.get_expulsion_by_key(&k).await;
                            if let Some(event) = ex {
                                let _ = remote.remove_expulsion(event).await;
                            }
                            MsgPageDetails::UpdatedAndThenNavigateHome
                        });
                    }
                    EventType::Feed => {
                        ctx.link().send_future(async move {
                            let remote = AsyncRemoteMonolith {};
                            let fe = remote.get_feeding_by_key(&k).await;
                            if let Some(event) = fe {
                                let _ = remote.remove_feeding(event).await;
                            }
                            MsgPageDetails::UpdatedAndThenNavigateHome
                        });
                    }
                }
                self.event_found_in_db = false; // Mainly to block double deletion ...
                return false;
            }
            MsgPageDetails::CancelDeletion => self.show_delete_dialog = false,
            MsgPageDetails::UpdateDate { date } => {
                let local_date_time = self.utc_date_time();
                let utc_parsed_date_time = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                    Ok(parsed_date) => {
                        let local_parsed_date = Local::ymd(
                            &Local,
                            parsed_date.year(),
                            parsed_date.month(),
                            parsed_date.day(),
                        );
                        let time = local_date_time.time();
                        let local_date_time = local_parsed_date.and_time(time).unwrap();
                        local_date_time.with_timezone(&Utc)
                    }
                    Err(e) => {
                        error!(e.to_string());
                        return false;
                    }
                };
                self.update_existing_event_time_stamp(utc_parsed_date_time);
            }
            MsgPageDetails::UpdateTime { time } => {
                let local_date_time = self.utc_date_time();

                let utc_parsed_date_time = match NaiveTime::parse_from_str(&time, "%H:%M") {
                    Ok(parsed_time) => {
                        let local_date = Local::ymd(
                            &Local,
                            local_date_time.year(),
                            local_date_time.month(),
                            local_date_time.day(),
                        );
                        let local_date_time = local_date.and_time(parsed_time).unwrap();
                        local_date_time.with_timezone(&Utc)
                    }
                    Err(_) => {
                        warn!(format!("variant_event :: Error parsing time {:#?}", time));
                        return false;
                    }
                };
                self.update_existing_event_time_stamp(utc_parsed_date_time);
            }
            MsgPageDetails::EventLoaded(ev) => {
                self.is_loading = false;

                if let Some(ev) = ev {
                    self.event_found_in_db = true;
                    self.ost_existing_event.push(Rc::new(RefCell::new(ev)));
                }
            }
            MsgPageDetails::ExpulsionLoaded(ex) => {
                self.is_loading = false;
                if let Some(ex) = ex {
                    self.event_found_in_db = true;
                    self.ost_existing_expulsion.push(Rc::new(RefCell::new(ex)));
                }
            }
            MsgPageDetails::FeedingLoaded(fe) => {
                self.is_loading = false;
                if let Some(fe) = fe {
                    self.event_found_in_db = true;
                    self.ost_existing_feeding.push(Rc::new(RefCell::new(fe)));
                }
            }
            MsgPageDetails::UpdatedAndThenNavigateHome => {
                #[allow(clippy::clone_on_copy)]
                let k = self.props.ost_event_key.clone();
                #[allow(clippy::single_match)]
                match self.props.ost_event_key.t {
                    EventType::Event => {
                        let n_event = self.ost_existing_event.get(0).unwrap().clone();
                        ctx.link().send_future(async move {
                            let remote = AsyncRemoteMonolith {};
                            let ev = remote.get_event_by_key(&k).await;
                            if ev.is_some() {
                                let _ = remote.modify_event(&n_event.borrow()).await;
                            }
                            MsgPageDetails::NavigateHome
                        });
                    }
                    EventType::Expulsion => {
                        let n_expulsion = self.ost_existing_expulsion.get(0).unwrap().clone();

                        ctx.link().send_future(async move {
                            let remote = AsyncRemoteMonolith {};
                            let ex = remote.get_expulsion_by_key(&k).await;
                            if ex.is_some() {
                                let _ = remote.modify_expulsion(&n_expulsion.borrow()).await;
                            }
                            MsgPageDetails::NavigateHome
                        });
                    }
                    EventType::Feed => {
                        let n_feed = self.ost_existing_feeding.get(0).unwrap().clone();

                        ctx.link().send_future(async move {
                            let remote = AsyncRemoteMonolith {};
                            let fe = remote.get_feeding_by_key(&k).await;
                            if fe.is_some() {
                                let _ = remote.modify_feeding(&n_feed.borrow()).await;
                            }
                            MsgPageDetails::NavigateHome
                        });
                    }
                }
                return false;
            }
            MsgPageDetails::NavigateHome => {
                match ctx.link().history() {
                    Some(h) => {
                        h.push(LillaOstRoutes::Home);
                    }
                    None => warn!("No history found"),
                }
                return false;
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.is_loading {
            return html! {
                <div>
                {"Loading requested event, please wait"}
                </div>
            };
        }

        if !self.event_found_in_db {
            return self.display_no_event();
        }
        let person_name = self.person_name();
        let are_you_sure_delete_modal = self.construct_are_you_sure_delete_modal(ctx);
        let local_date_time = self.local_date_time();

        let details = self.render_details();
        html! {
             <>
                 {are_you_sure_delete_modal}
                 <div>
                     <div class="field">
                         <label class="label">{ person_name }</label>
                     </div>
                    <div class="field">
                        <label class="label">{"Date"}</label>
                        <div class="control">
                            <input class="input"
                                    type="date"
                                    id="input_date"
                                    value={ local_date_time.format("%Y-%m-%d").to_string() }
                                    oninput={ctx.link().callback(move |e: InputEvent| MsgPageDetails::UpdateDate { date: input_get_value_from_input_event(e) } ) }
                                    />
                        </div>
                    </div>
                    <div class="field">
                        <label class="label">{"Time"}</label>
                        <div class="control">
                            <input class="input"
                                    type="time"
                                    id="input_time"
                                    value={ local_date_time.format("%H:%M").to_string() }
                                    oninput={ctx.link().callback(move |e: InputEvent| MsgPageDetails::UpdateTime { time: input_get_value_from_input_event(e) } ) }
                            />
                        </div>
                    </div>
                </div>
                <div class="block">
                        { details }
                </div>
                <div class="block">
                    <div style="display:flex; justify-content:space-between; padding:0; align-items: baseline;">
                        <button class="button is-danger" onclick={ctx.link().callback(|_| MsgPageDetails::RequestDeletionConfirmation)} >
                            { "Delete" }
                        </button>

                        <Link<LillaOstRoutes> classes={classes!(vec!["button", "is-primary"])} to={LillaOstRoutes::Home} >
                            { "Back" }
                        </Link<LillaOstRoutes>>

                        <button class="button is-warning" onclick={ctx.link().callback(|_| MsgPageDetails::UpdatedAndThenNavigateHome)} >
                            { "Update" }
                        </button>
                    </div>
                 </div>
                 <p></p>
             </>
        }
    }
}

impl PageDetails {
    fn update_existing_event_time_stamp(&mut self, new_time_stamp: DateTime<Utc>) {
        #[allow(clippy::single_match)]
        match self.props.ost_event_key.t {
            EventType::Event => {
                let event = self.ost_existing_event.get_mut(0).unwrap();
                let event_payload = event.borrow().event();
                event
                    .borrow_mut()
                    .modify_event(new_time_stamp, event_payload);
            }
            EventType::Expulsion => {
                let expulsion = self.ost_existing_expulsion.get_mut(0).unwrap();
                let degree = expulsion.borrow().degree();
                expulsion
                    .borrow_mut()
                    .modify_expulsion(degree, new_time_stamp);
            }
            EventType::Feed => {
                let feeding = self.ost_existing_feeding.get_mut(0).unwrap();
                let bm = feeding.borrow().breast_milk();
                let fo = feeding.borrow().formula();
                let so = feeding.borrow().solids();
                feeding.borrow_mut().modify_feed(bm, fo, so, new_time_stamp);
            }
        }
    }

    #[allow(clippy::clone_on_copy)]
    fn local_date_time(&self) -> DateTime<Local> {
        let ts = match &self.props.ost_event_key.t {
            EventType::Event => self
                .ost_existing_event
                .get(0)
                .as_ref()
                .unwrap()
                .borrow()
                .time_stamp()
                .clone(),
            EventType::Expulsion => self
                .ost_existing_expulsion
                .get(0)
                .as_ref()
                .unwrap()
                .borrow()
                .time_stamp()
                .clone(),
            EventType::Feed => self
                .ost_existing_feeding
                .get(0)
                .as_ref()
                .unwrap()
                .borrow()
                .time_stamp()
                .clone(),
        };
        ts.with_timezone(&Local)
    }

    #[allow(clippy::clone_on_copy)]
    fn utc_date_time(&self) -> DateTime<Utc> {
        match &self.props.ost_event_key.t {
            EventType::Event => self
                .ost_existing_event
                .get(0)
                .as_ref()
                .unwrap()
                .borrow()
                .time_stamp()
                .clone(),
            EventType::Expulsion => self
                .ost_existing_expulsion
                .get(0)
                .as_ref()
                .unwrap()
                .borrow()
                .time_stamp()
                .clone(),
            EventType::Feed => self
                .ost_existing_feeding
                .get(0)
                .as_ref()
                .unwrap()
                .borrow()
                .time_stamp()
                .clone(),
        }
    }

    fn person_name(&self) -> String {
        match &self.props.ost_event_key.t {
            EventType::Event => self
                .ost_existing_event
                .get(0)
                .as_ref()
                .unwrap()
                .borrow()
                .person_name(),
            EventType::Expulsion => self
                .ost_existing_expulsion
                .get(0)
                .as_ref()
                .unwrap()
                .borrow()
                .person_name(),
            EventType::Feed => self
                .ost_existing_feeding
                .get(0)
                .as_ref()
                .unwrap()
                .borrow()
                .person_name(),
        }
    }

    fn render_details(&self) -> Html {
        if !self.event_found_in_db {
            return html!();
        }

        match &self.props.ost_event_key.t {
            EventType::Feed => {
                let rc_existing_feeding = Some(self.ost_existing_feeding.get(0).unwrap().clone());
                html! {
                    <InsertFeedings ost_feeding={ rc_existing_feeding } />
                }
            }
            EventType::Expulsion => {
                let rc_existing_expulsion =
                    Some(self.ost_existing_expulsion.get(0).unwrap().clone());
                html! {
                    <InsertExpulsions ost_expulsion={ rc_existing_expulsion } />
                }
            }
            EventType::Event => {
                let rc_event = Some(self.ost_existing_event.get(0).unwrap().clone());
                html! {
                    <InsertEvent ost_event={ rc_event } />
                }
            }
        }
    }

    fn display_no_event(&self) -> Html {
        let message = match self.props.ost_event_key.t {
            EventType::Event => format!("No Event found with ID {}", self.props.ost_event_key.id),
            EventType::Expulsion => {
                format!("No Expulsion found with ID {}", self.props.ost_event_key.id)
            }
            EventType::Feed => format!("No Feeding found with ID {}", self.props.ost_event_key.id),
        };

        html! {
            <div>
                <p>
                    { message }
                </p>
                <div>
                    <Link<LillaOstRoutes> classes={ classes!(vec!["button", "is-primary"]) } to={ LillaOstRoutes::Home } >
                        { "Back" }
                    </Link<LillaOstRoutes>>
                </div>
            </div>
        }
    }

    fn construct_are_you_sure_delete_modal(&self, ctx: &Context<Self>) -> Html {
        if self.show_delete_dialog {
            html! {
                <div class="modal is-active">
                    <div class="modal-background">
                </div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{"Please confirm"}</p>
                        <button class="delete" aria-label="close" onclick={ctx.link().callback(|_| MsgPageDetails::CancelDeletion ) }></button>
                    </header>
                    <section class="modal-card-body">
                        <p>{"You're about to delete this event."}</p>
                        <p>{"Are you sure?"}</p>
                    </section>
                    <footer class="modal-card-foot">
                        <button class="button is-danger" onclick={ctx.link().callback(|_| MsgPageDetails::PerformEventDeletion) }>{"Delete"}</button>
                        <button class="button is-success" onclick={ctx.link().callback(|_| MsgPageDetails::CancelDeletion) }>{"Cancel"}</button>
                    </footer>
                </div>
            </div>
            }
        } else {
            html! {}
        }
    }
}
