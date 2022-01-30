use chrono::Local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::routes::LillaOstRoutes;

use ost::event_base::EventBase as ost_EventBase;

/// Id
/// Date as in YYYY-MM-DD
/// Time as in HH:MM
/// Summary

#[allow(clippy::borrowed_box)]
pub fn event_entry(event: &Box<dyn ost_EventBase>) -> Html {
    let local_date_time = event.time_stamp().with_timezone(&Local);
    let formatted_time = local_date_time.format("%H:%M").to_string();
    let formatted_date = local_date_time.format("%x").to_string();

    let ost_key = event.key();

    let to_details_route = match ost_key.t {
        ost::event_key::EventType::Event => html! {
            <Link<LillaOstRoutes> to={ LillaOstRoutes::OstDetailsEvent{ id: ost_key.id } } >
                { formatted_date }
            </Link<LillaOstRoutes>>
        },
        ost::event_key::EventType::Expulsion => html! {
            <Link<LillaOstRoutes> to={ LillaOstRoutes::OstDetailsExpulsion { id: ost_key.id } } >
                { formatted_date }
            </Link<LillaOstRoutes>>
        },
        ost::event_key::EventType::Feed => html! {
            <Link<LillaOstRoutes> to={ LillaOstRoutes::OstDetailsFeed{ id: ost_key.id } } >
                { formatted_date }
            </Link<LillaOstRoutes>>
        },
    };

    html! {
    <tr>
        <td>
            { to_details_route }
        </td>
        <td>
            { formatted_time }
        </td>
        <td>
            { event.person_name() }
        </td>
        <td>
            { event.summary() }
        </td>
    </tr>
    }
}
