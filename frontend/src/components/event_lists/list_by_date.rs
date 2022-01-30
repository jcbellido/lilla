use chrono::prelude::*;

use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::event_bus::EventBus;

use ost::context_remote_async::AsyncRemoteMonolith;
use ost::event_base::EventBase as ost_EventBase;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub date: Date<Utc>,
}

pub enum MsgListByDate {
    RequestLoadPage(Date<Utc>),
    DataForPageReceived { data: Vec<Box<dyn ost_EventBase>> },
    StorageChanged(String),
}

pub struct ListByDate {
    _producer: Box<dyn Bridge<EventBus>>,
    date: Date<Utc>,
    slice_to_display: Vec<Box<dyn ost_EventBase>>,
}

impl Component for ListByDate {
    type Message = MsgListByDate;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ost_get_slice(ctx, ctx.props().date);
        Self {
            _producer: EventBus::bridge(ctx.link().callback(MsgListByDate::StorageChanged)),
            slice_to_display: vec![],
            date: ctx.props().date,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgListByDate::StorageChanged(_) => {
                ctx.link()
                    .send_message(MsgListByDate::RequestLoadPage(self.date));
            }
            MsgListByDate::RequestLoadPage(date) => {
                ost_get_slice(ctx, date);
            }
            MsgListByDate::DataForPageReceived { data } => {
                self.slice_to_display = data;
                return true;
            }
        }
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.date = ctx.props().date;
        ost_get_slice(ctx, self.date);
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
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
        }
    }
}

fn ost_get_slice(ctx: &Context<ListByDate>, date: Date<Utc>) {
    ctx.link().send_future(async move {
        let remote_monolith = AsyncRemoteMonolith {};

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

        result.append(&mut feeds_page);
        result.append(&mut expulsions_page);
        result.append(&mut events_page);
        result.retain(|r| r.time_stamp().date() == date);
        result.sort_by(|a, b| b.time_stamp().cmp(a.time_stamp()));

        MsgListByDate::DataForPageReceived { data: result }
    });
}
