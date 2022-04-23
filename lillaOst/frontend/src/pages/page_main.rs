use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::event_listing::EventListing;
use crate::components::quick_insert::QuickInsert;
use crate::components::summary_expulsions::SummaryExpulsions;
use crate::components::summary_feeding::SummaryFeeding;
use crate::pages::routes::LillaOstRoutes;

use ost::context_remote_async::AsyncRemoteMonolith;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct PropsPageMain {
    pub page: u64,
}

pub enum MsgPageMain {
    MsgFirstLoad {
        remote_has_persons: bool,
        remote_has_active_persons: bool,
    },
}

pub struct PageMain {
    page: u64,
    first_load_in_progress: bool,
    remote_has_persons: bool,
    remote_has_active_persons: bool,
}

impl Component for PageMain {
    type Message = MsgPageMain;
    type Properties = PropsPageMain;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            let remote = AsyncRemoteMonolith {};
            let persons = remote.persons().await;

            let remote_has_persons = !persons.is_empty();
            let remote_has_active_persons = persons.iter().filter(|p| p.is_active()).count() > 0;

            MsgPageMain::MsgFirstLoad {
                remote_has_persons,
                remote_has_active_persons,
            }
        });

        Self {
            page: ctx.props().page,
            first_load_in_progress: true,
            remote_has_persons: false,
            remote_has_active_persons: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgPageMain::MsgFirstLoad {
                remote_has_persons,
                remote_has_active_persons,
            } => {
                self.first_load_in_progress = false;
                self.remote_has_active_persons = remote_has_active_persons;
                self.remote_has_persons = remote_has_persons;
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.first_load_in_progress {
            let first_start_modal = html! {

                <div class="modal is-active">
                <div class="modal-background">
                </div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{"Loading remote data"}</p>
                    </header>
                    <section class="modal-card-body">
                        <p>{"This might take a moment."}</p>
                    </section>
                </div>
            </div>
            };
            return html! {
                <div>
                    { first_start_modal }
                </div>
            };
        }

        if !self.remote_has_persons {
            let first_start_modal = html! {

                <div class="modal is-active">
                <div class="modal-background">
                </div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{"First time using LillaOst?"}</p>
                    </header>
                    <section class="modal-card-body">
                        <p>{"No previous data found."}</p>
                        <p>{"Start by adding a tracked person in the `Settings` Section."}</p>
                    </section>
                    <footer class="modal-card-foot">
                        <Link<LillaOstRoutes> classes="button is-success" to={LillaOstRoutes::Settings} >
                            {"Go to settings"}
                        </Link<LillaOstRoutes>>
                    </footer>
                </div>
            </div>
            };
            return html! {
                <div>
                    { first_start_modal }
                    <p></p>
                </div>
            };
        }

        if !self.remote_has_active_persons {
            let no_active_individuals = html! {
                <div class="modal is-active">
                <div class="modal-background">
                </div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{"No active individuals found?"}</p>
                    </header>
                    <section class="modal-card-body">
                        <p>{"Can't find any active individual."}</p>
                        <p>{"Please, check in the `Settings` section if every individual is deactivated."}</p>
                    </section>
                    <footer class="modal-card-foot">
                        <Link<LillaOstRoutes> classes="button is-success" to={LillaOstRoutes::Settings} >
                            {"Go to settings"}
                        </Link<LillaOstRoutes>>
                    </footer>
                </div>
            </div>
            };
            return html! {
                <div>
                    { no_active_individuals }
                    <p></p>
                </div>
            };
        }
        html! {
            <div>
                <div class="block"></div>
                <QuickInsert />
                <SummaryFeeding />
                <SummaryExpulsions />
                <div class="block">
                    <EventListing page={self.page} />
                </div>
                <p></p>
            </div>
        }
    }
}
