use std::rc::Rc;

use yew::{html, Component, Context, Html, Properties};

use ost::context_remote_async::AsyncRemoteMonolith;

pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
}

pub enum Msg {
    SetFetchState(FetchState<String>),
    CallGet,
    CallPurgeEvents,
}

#[derive(Properties, Clone, PartialEq)]
pub struct PropsFetcher {
    pub shim_remote: Rc<AsyncRemoteMonolith>,
}

pub struct Fetcher {
    state: FetchState<String>,
    shim_remote: Rc<AsyncRemoteMonolith>,
}

impl Component for Fetcher {
    type Message = Msg;
    type Properties = PropsFetcher;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            state: FetchState::NotFetching,
            shim_remote: ctx.props().shim_remote.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetFetchState(fetch_state) => {
                self.state = fetch_state;
                true
            }
            Msg::CallGet => {
                let shim = self.shim_remote.clone();

                ctx.link().send_future(async move {
                    let answer = shim.feedings().await;

                    Msg::SetFetchState(FetchState::Success(answer.len().to_string()))
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }

            Msg::CallPurgeEvents => {
                let shim = self.shim_remote.clone();

                ctx.link().send_future(async move {
                    let answer = shim.purge_all_events().await;
                    match answer {
                        Ok(_) => Msg::SetFetchState(FetchState::Success(
                            "purge events done!".to_string(),
                        )),
                        Err(e) => Msg::SetFetchState(FetchState::Success(e)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.state {
            FetchState::NotFetching => html! {
                <>
                    <button onclick={ctx.link().callback(|_| Msg::CallGet)}>
                        { "Get Feedings" }
                    </button>
                    <button onclick={ctx.link().callback(|_| Msg::CallPurgeEvents)}>
                        { "Purge all events" }
                    </button>
                </>
            },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(data) => html! { data },
        }
    }
}
