use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::event_bus::EventBus;

use ost::context_remote_async::AsyncRemoteMonolith;
use ost::event_base::EventBase as ost_EventBase;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub page: u64,
    pub page_size: u64,
}

pub enum MsgListByPage {
    RequestLoadPage(u64),
    DataForPageReceived {
        page: u64,
        max_pages: u64,
        data: Vec<Box<dyn ost_EventBase>>,
    },
    StorageChanged(String),
}

pub struct ListByPage {
    events_per_page: u64,
    page: u64,
    max_page: Option<u64>,
    _producer: Box<dyn Bridge<EventBus>>,
    slice_to_display: Vec<Box<dyn ost_EventBase>>,
}

impl Component for ListByPage {
    type Message = MsgListByPage;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ost_get_slice(ctx, ctx.props().page, 15);
        Self {
            events_per_page: ctx.props().page_size,
            page: ctx.props().page,
            _producer: EventBus::bridge(ctx.link().callback(MsgListByPage::StorageChanged)),
            max_page: None,
            slice_to_display: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgListByPage::StorageChanged(_) => {
                ctx.link()
                    .send_message(MsgListByPage::RequestLoadPage(self.page));
            }
            MsgListByPage::RequestLoadPage(page) => {
                ost_get_slice(ctx, page, self.events_per_page as u64);
            }
            MsgListByPage::DataForPageReceived {
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

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.page = ctx.props().page;
        self.events_per_page = ctx.props().page_size;
        ost_get_slice(ctx, self.page, self.events_per_page);
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let pagination_position = format!("{} / {}", self.page + 1, self.max_page.unwrap_or(0));

        if self.slice_to_display.is_empty() {
            return html! {
                <div>
                    {format!("No events found for page: {}", pagination_position)}
                </div>
            };
        } else {
            return html! {
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
                                for self.slice_to_display.iter().map(super::event_table_entry::event_entry)
                            }
                        </tbody>
                    </table>
                </div>
            };
        }
    }
}

fn ost_get_slice(ctx: &Context<ListByPage>, page: u64, page_size: u64) {
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
            return MsgListByPage::DataForPageReceived {
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
            result.drain(u_page_size..);
        }

        MsgListByPage::DataForPageReceived {
            page,
            max_pages: total_pages,
            data: result,
        }
    });
}
