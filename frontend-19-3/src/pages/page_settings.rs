use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_agent::{Dispatched, Dispatcher};
use yew_router::prelude::*;

use crate::event_bus::{EventBus, Request};

use ost::context_remote_async::AsyncRemoteMonolith;

use crate::components::settings_individuals::SettingsIndividuals;

#[cfg(debug_assertions)]
use crate::components::debug_buttons::DebugControls;

use super::routes::LillaOstRoutes;

pub enum MsgPageSettings {
    RequestDisplayDeleteData,
    RequestHideDeleteData,
    ActuallyDeleteAll,
    RequestDisplayDeleteEvents,
    RequestHideDeleteEvents,
    ActuallyDeleteEvents,
    CallDone,
}

#[allow(dead_code)]
pub struct PageSettings {
    show_delete_dialog: bool,
    show_delete_events_dialog: bool,
    event_bus: Dispatcher<EventBus>,
}

impl Component for PageSettings {
    type Message = MsgPageSettings;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            show_delete_dialog: false,
            show_delete_events_dialog: false,
            event_bus: EventBus::dispatcher(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgPageSettings::RequestDisplayDeleteData => {
                self.show_delete_dialog = true;
            }
            MsgPageSettings::RequestHideDeleteData => {
                self.show_delete_dialog = false;
            }
            MsgPageSettings::ActuallyDeleteAll => {
                ctx.link().send_future(async {
                    let remote = AsyncRemoteMonolith {};
                    let _ = remote.purge_all_data().await;
                    MsgPageSettings::CallDone
                });
                return false;
            }

            MsgPageSettings::RequestDisplayDeleteEvents => {
                self.show_delete_events_dialog = true;
            }
            MsgPageSettings::RequestHideDeleteEvents => {
                self.show_delete_events_dialog = false;
            }
            MsgPageSettings::ActuallyDeleteEvents => {
                ctx.link().send_future(async {
                    let remote = AsyncRemoteMonolith {};
                    let _ = remote.purge_all_events().await;
                    MsgPageSettings::CallDone
                });
            }
            MsgPageSettings::CallDone => {
                self.show_delete_events_dialog = false;
                self.show_delete_dialog = false;
                self.event_bus.send(Request::EventBusMsg(
                    "Page settings, call finished".to_owned(),
                ));
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let are_you_sure_about_delete = self.construct_are_you_sure_delete_modal(ctx);
        let are_you_sure_about_delete_events = self.construct_are_you_sure_delete_events_modal(ctx);

        #[cfg(not(debug_assertions))]
        let debug_controls = html!();

        #[cfg(debug_assertions)]
        let debug_controls = html!(<DebugControls />);

        html! {
             <>
                 <div class="block"></div>
                 <SettingsIndividuals />

                 <div class="block">
                     <div class="columns">
                         <div class="column is-one-third">
                             <Link<LillaOstRoutes> classes={ classes!(vec!["button", "is-primary"]) } to={LillaOstRoutes::Home} >
                                 { "Home" }
                             </Link<LillaOstRoutes>>
                         </div>
                     </div>
                 </div>
                { debug_controls }
                <article class="message is-danger">
                    <div class="message-header">
                        <p>{"The danger zone"}</p>
                    </div>
                    <div class="message-body">
                        { are_you_sure_about_delete }
                        { are_you_sure_about_delete_events}
                        <div style="display:flex; justify-content:space-between; padding:0; align-items: baseline;">
                            <button class="button is-danger" onclick={ctx.link().callback(|_| MsgPageSettings::RequestDisplayDeleteEvents)} >{ "Purge all events" }</button>
                            <button class="button is-danger" onclick={ctx.link().callback(|_| MsgPageSettings::RequestDisplayDeleteData)} >{ "Purge all data" }</button>
                        </div>
                    </div>
                </article>
             </>
        }
    }
}

impl PageSettings {
    fn construct_are_you_sure_delete_modal(&self, ctx: &Context<Self>) -> VNode {
        if self.show_delete_dialog {
            html! {
                <div class="modal is-active">
                    <div class="modal-background">
                </div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{"You'll lose all data!"}</p>
                        <button class="delete" aria-label="close" onclick={ctx.link().callback(|_| MsgPageSettings::RequestHideDeleteData)}></button>
                    </header>
                    <section class="modal-card-body">
                        <p>{"If you click `Delete all`, you'll lose all data contained in LillaOst."}</p>
                        <p>{"Are you sure?"}</p>
                    </section>
                    <footer class="modal-card-foot">
                        <button class="button is-danger" onclick={ctx.link().callback(|_| MsgPageSettings::ActuallyDeleteAll)} >{"Delete all"}</button>
                        <button class="button is-success" onclick={ctx.link().callback(|_| MsgPageSettings::RequestHideDeleteData)} >{"Cancel"}</button>
                    </footer>
                </div>
            </div>}
        } else {
            html! {}
        }
    }

    fn construct_are_you_sure_delete_events_modal(&self, ctx: &Context<Self>) -> VNode {
        if self.show_delete_events_dialog {
            html! {
                <div class="modal is-active">
                    <div class="modal-background">
                </div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{"You'll lose every event!"}</p>
                        <button class="delete" aria-label="close" onclick={ctx.link().callback(|_| MsgPageSettings::RequestHideDeleteEvents)}></button>
                    </header>
                    <section class="modal-card-body">
                        <p>{"If you click `Delete events`, you'll lose every event in LillaOst."}</p>
                        <p>{"Are you sure?"}</p>
                    </section>
                    <footer class="modal-card-foot">
                        <button class="button is-danger" onclick={ctx.link().callback(|_| MsgPageSettings::ActuallyDeleteEvents)}>{"Delete events"}</button>
                        <button class="button is-success" onclick={ctx.link().callback(|_| MsgPageSettings::RequestHideDeleteEvents)}>{"Cancel"}</button>
                    </footer>
                </div>
            </div>}
        } else {
            html! {}
        }
    }
}
