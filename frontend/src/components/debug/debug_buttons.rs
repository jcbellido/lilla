use serde_derive::{Deserialize, Serialize};

use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};

use crate::event_bus::{EventBus, Request};

use ost::context_remote_async::AsyncRemoteMonolith;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArgFakeCount {
    pub count: u32,
}

pub enum MsgDebugControls {
    AppendRandomEventCount(u32),
    AppendRandomPerson,
    AppendFeedingsForToday,
    CallFinished,
}

pub struct DebugControls {
    event_bus: Dispatcher<EventBus>,
}

impl Component for DebugControls {
    type Message = MsgDebugControls;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            event_bus: EventBus::dispatcher(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgDebugControls::AppendRandomEventCount(count) => {
                ctx.link().send_future(async move {
                    let remote = AsyncRemoteMonolith {};
                    let _ = remote.add_fake_events(count).await;
                    MsgDebugControls::CallFinished
                });
            }
            MsgDebugControls::AppendRandomPerson => {
                ctx.link().send_future(async {
                    let remote = AsyncRemoteMonolith {};
                    let _ = remote.add_fake_persons(1).await;
                    MsgDebugControls::CallFinished
                });
            }
            MsgDebugControls::AppendFeedingsForToday => {
                ctx.link().send_future(async {
                    let remote = AsyncRemoteMonolith {};
                    let mut persons = remote.persons().await;
                    for _ in 0..persons.len() {
                        let p = persons.pop().unwrap();
                        let _ = remote.add_feeding(&p, 100, 100, 100).await;
                    }
                    MsgDebugControls::CallFinished
                });
            }

            MsgDebugControls::CallFinished => {
                self.event_bus
                    .send(Request::EventBusMsg("DebugControls inserted".to_owned()));
                return false;
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // The following number is picked at random
        html! {
            <>
            <div class="block">
                <article class="message is-warning">
                    <div class="message-header">
                        <p>{"Testing features"}</p>
                    </div>
                    <div class="message-body">
                        <div class="columns">
                            <div class="column">
                                <button class="button" onclick={ctx.link().callback(|_| MsgDebugControls::AppendRandomPerson  )} >{ "Add Person" }</button>
                            </div>
                            <div class="column">
                                <button class="button" onclick={ctx.link().callback(|_| MsgDebugControls::AppendRandomEventCount(1) )} >{ "Add event" }</button>
                            </div>
                            <div class="column">
                                <button class="button" onclick={ctx.link().callback(|_| MsgDebugControls::AppendRandomEventCount(150) )} >{ "Add rand. 150" }</button>
                            </div>
                            <div class="column">
                                <button class="button" onclick={ctx.link().callback(|_| MsgDebugControls::AppendRandomEventCount(1500) )} >{ "Add rand. 1500" }</button>
                            </div>
                            <div class="column">
                                <button class="button" onclick={ctx.link().callback(|_| MsgDebugControls::AppendRandomEventCount(6000) )} >{ "Add rand. 6000" }</button>
                            </div>
                            <div class="column">
                                <button class="button" onclick={ctx.link().callback(|_| MsgDebugControls::AppendFeedingsForToday )} >{ "Feedings Today" }</button>
                            </div>
                        </div>
                    </div>
                </article>
            </div>
            <div class="block"/>
            </>
        }
    }
}
