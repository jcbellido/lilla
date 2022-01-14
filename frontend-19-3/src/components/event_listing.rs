use chrono::Local;

use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use yew_router::prelude::*;

use crate::event_bus::EventBus;
use crate::pages::routes::LillaOstRoutes;

use ost::context_remote_async::AsyncRemoteMonolith;
use ost::event_base::EventBase as ost_EventBase;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub page: u64,
}

pub enum MsgEventListing {
    FirstPage,
    NextPage,
    PreviousPage,
    RequestLoadPage(u64),
    DataForPageReceived {
        page: u64,
        max_pages: u64,
        data: Vec<Box<dyn ost_EventBase>>,
    },
    StorageChanged(String),
}

pub struct EventListing {
    events_per_page: u16,
    page: u64,
    max_page: Option<u64>,
    _producer: Box<dyn Bridge<EventBus>>,
    slice_to_display: Vec<Box<dyn ost_EventBase>>,
}

/// Id
/// Date as in YYYY-MM-DD
/// Time as in HH:MM
/// Summary
impl EventListing {
    #[allow(clippy::borrowed_box)]
    fn event_entry(&self, event: &Box<dyn ost_EventBase>) -> Html {
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
}

impl Component for EventListing {
    type Message = MsgEventListing;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ost_get_slice(ctx, ctx.props().page, 15);
        Self {
            events_per_page: 15,
            page: ctx.props().page,
            _producer: EventBus::bridge(ctx.link().callback(MsgEventListing::StorageChanged)),
            max_page: None,
            slice_to_display: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgEventListing::FirstPage => {
                self.page = 0;
                ctx.link()
                    .send_message(MsgEventListing::RequestLoadPage(self.page));
            }
            MsgEventListing::NextPage => {
                if let Some(max_page) = self.max_page {
                    if (self.page + 1) <= max_page {
                        self.page += 1;
                        ctx.link()
                            .send_message(MsgEventListing::RequestLoadPage(self.page));
                    }
                } else {
                    // don't have max page (??)
                    self.page += 1;
                    ctx.link()
                        .send_message(MsgEventListing::RequestLoadPage(self.page));
                }
            }
            MsgEventListing::PreviousPage => {
                if self.page > 0 {
                    self.page -= 1;
                    ctx.link()
                        .send_message(MsgEventListing::RequestLoadPage(self.page));
                }
            }
            MsgEventListing::StorageChanged(_) => {
                ctx.link()
                    .send_message(MsgEventListing::RequestLoadPage(self.page));
            }
            MsgEventListing::RequestLoadPage(page) => {
                ost_get_slice(ctx, page, self.events_per_page as u64);
            }
            MsgEventListing::DataForPageReceived {
                page,
                max_pages,
                data,
            } => {
                self.page = page;
                self.max_page = Some(max_pages);
                self.slice_to_display = data;
                return true;
            }
        }
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let pagination_position = format!("{} / {}", self.page + 1, self.max_page.unwrap_or(0));

        if self.slice_to_display.is_empty() {
            return html! {
                <div>
                    {format!("No events found for page: {}", pagination_position)}
                </div>
            };
        } else {
            return html! {
                <>
                    <div class="table-container">
                        <table class="table is-fullwidth is-striped is-narrow">
                            <thead>
                                <tr>
                                    <th>{"Date"}</th>
                                    <th>{"Time"}</th>
                                    <th>{"Person"}</th>
                                    <th>{"Summary"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {
                                    for self.slice_to_display.iter().map(|e| self.event_entry(e))
                                }
                            </tbody>
                        </table>
                    </div>
                    <div class="container">
                        <nav class="pagination is-centered" role="navigation" aria-label="pagination">
                            <div>
                                { pagination_position }
                            </div>
                            <a class="pagination-previous" onclick={ctx.link().callback(|_| MsgEventListing::FirstPage)} >{"First"}</a>
                            <a class="pagination-previous" onclick={ctx.link().callback(|_| MsgEventListing::PreviousPage)} >{"Previous"}</a>
                            <a class="pagination-next" onclick={ctx.link().callback(|_| MsgEventListing::NextPage)}>{"Next"}</a>
                        </nav>
                    </div>
                </>
            };
        }
    }
}

fn ost_get_slice(ctx: &Context<EventListing>, page: u64, page_size: u64) {
    ctx.link().send_future(async move {
        let remote_monolith = AsyncRemoteMonolith {};

        let from: usize = page as usize * page_size as usize;
        let u_page_size = page_size as usize;

        let feeds = remote_monolith.feedings();
        let events = remote_monolith.events();
        let expulsions = remote_monolith.expulsions();

        let mut feeds = feeds.await;
        feeds.retain(|f| f.is_person_active());
        let mut feeds_page: Vec<Box<dyn ost_EventBase>> = feeds
            .drain(..)
            .map(|f| f as Box<dyn ost_EventBase>)
            .collect();

        let mut expulsions = expulsions.await;
        expulsions.retain(|f| f.is_person_active());
        let mut expulsions_page: Vec<Box<dyn ost_EventBase>> = expulsions
            .drain(..)
            .map(|exp| exp as Box<dyn ost_EventBase>)
            .collect();

        let mut events = events.await;
        events.retain(|f| f.is_person_active());
        let mut events_page: Vec<Box<dyn ost_EventBase>> = events
            .drain(..)
            .map(|eve| eve as Box<dyn ost_EventBase>)
            .collect();

        let mut result: Vec<Box<dyn ost_EventBase>> = vec![];

        let total_entries =
            feeds_page.len() as u64 + expulsions_page.len() as u64 + events_page.len() as u64;
        let total_pages = total_entries / page_size;

        result.append(&mut feeds_page);
        result.append(&mut expulsions_page);
        result.append(&mut events_page);

        if from > result.len() {
            return MsgEventListing::DataForPageReceived {
                page,
                max_pages: total_pages,
                data: vec![],
            };
        }

        result.sort_by(|a, b| b.time_stamp().cmp(a.time_stamp()));

        if from > 0 {
            result.drain(0..from);
        }

        if result.len() > u_page_size {
            // result.drain(0..u_page_size);
            result.drain(u_page_size..);
        }

        MsgEventListing::DataForPageReceived {
            page,
            max_pages: total_pages,
            data: result,
        }
    });
}
