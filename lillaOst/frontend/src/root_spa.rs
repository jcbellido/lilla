//! `root_spa` has two functions:
//! 1. Mounts the rest of the page
//! 1. Implements the storage service

use yew::prelude::*;
use yew_router::prelude::*;

use ost::event_key::{EventType, OstEventKey};

use crate::pages::{
    page_details::PageDetails, page_main::PageMain, page_not_found::PageNotFound,
    page_settings::PageSettings, page_summary::PageSummary, routes::LillaOstRoutes,
};

pub enum MsgRootSpa {
    ToggleNavbar,
}

pub struct RootSpa {
    navbar_active: bool,
}

impl RootSpa {
    fn view_nav(&self, ctx: &Context<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "LillaOst" }</h1>
                    <button
                        class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={ctx.link().callback(|_| MsgRootSpa::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>

                <div class={classes!("navbar-menu", active_class)} >
                    <div class="navbar-start">
                        <Link<LillaOstRoutes> classes={classes!("navbar-item")} to={LillaOstRoutes::Home}>
                            { "Home" }

                        </Link<LillaOstRoutes>>
                        <Link<LillaOstRoutes> classes={classes!("navbar-item")} to={LillaOstRoutes::Summary}>
                            { "Summary" }
                        </Link<LillaOstRoutes>>
                        <Link<LillaOstRoutes> classes={classes!("navbar-item")} to={LillaOstRoutes::Settings}>
                            { "Settings" }
                        </Link<LillaOstRoutes>>
                        <a class={classes!("navbar-item")} target="_blank" onclick={ctx.link().callback(|_| MsgRootSpa::ToggleNavbar)} href="https://github.com/jcbellido/LillaOst-Feedback/issues">{ "Report an issue" }</a>
                        <a class={classes!("navbar-item")} target="_blank" onclick={ctx.link().callback(|_| MsgRootSpa::ToggleNavbar)} href="https://jcbellido.netlify.app/tags/lillaost">{ "Articles about LillaOst" }</a>
                    </div>
                </div>
            </nav>
        }
    }
}

fn switch(routes: &LillaOstRoutes) -> Html {
    match routes {
        LillaOstRoutes::Home => html! { <PageMain page=0 />},
        LillaOstRoutes::Page { no } => html! { <PageMain page={ *no } />},
        LillaOstRoutes::Summary => html! { <PageSummary />},
        LillaOstRoutes::Settings => html! { <PageSettings /> },
        // // Details block
        LillaOstRoutes::OstDetailsEvent { id } => {
            html! { <PageDetails ost_event_key= {OstEventKey { t: EventType::Event, id: *id }}/>}
        }
        LillaOstRoutes::OstDetailsFeed { id } => {
            html! { <PageDetails ost_event_key= {OstEventKey { t: EventType::Feed, id: *id }}/>}
        }
        LillaOstRoutes::OstDetailsExpulsion { id } => {
            html! { <PageDetails ost_event_key= {OstEventKey { t: EventType::Expulsion, id: *id }}/>}
        }
        _ => html! { <PageNotFound /> },
    }
}

impl Component for RootSpa {
    type Message = MsgRootSpa;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgRootSpa::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx) }
                <div class="container">
                    <main>
                        <Switch<LillaOstRoutes> render={Switch::render(switch)} />
                    </main>
                </div>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Powered by " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                        { " using " }
                        <a href="https://bulma.io">{ "Bulma" }</a>
                        { " by "}
                        <a href="https://twitter.com/jc_bellido">{ "jcbellido" }</a>
                    </div>
                </footer>
            </BrowserRouter>
        }
    }
}
