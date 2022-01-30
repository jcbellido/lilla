use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum LillaOstRoutes {
    #[at("/ost-details/event/:id")]
    OstDetailsEvent { id: u32 },

    #[at("/ost-details/feed/:id")]
    OstDetailsFeed { id: u32 },

    #[at("/ost-details/expulsion/:id")]
    OstDetailsExpulsion { id: u32 },

    #[at("/")]
    Home,

    #[at("/page/:no")]
    Page { no: u64 },

    #[at("/summary")]
    Summary,

    #[at("/settings")]
    Settings,

    #[not_found]
    #[at("/404")]
    NotFound,
}
