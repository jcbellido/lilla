use chrono::prelude::*;
use yew::prelude::*;

use crate::components::event_lists::list_by_date::ListByDate;
use crate::components::event_lists::list_by_page::ListByPage;
use crate::components::inputs::date_input_box::DateInputBox;

pub enum EventListingMode {
    ByPage,
    ByDate,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub page: u64,
}

pub enum MsgEventListing {
    FirstPage,
    NextPage,
    PreviousPage,
    DateChanged(Date<Utc>),
}

pub struct EventListing {
    page: u64,
    listing_mode: EventListingMode,
    date: Date<Utc>,
}

impl Component for EventListing {
    type Message = MsgEventListing;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            page: ctx.props().page,
            listing_mode: EventListingMode::ByDate,
            date: Utc::today(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgEventListing::FirstPage => {
                self.page = 0;
                self.listing_mode = EventListingMode::ByPage;
            }
            MsgEventListing::NextPage => {
                self.listing_mode = EventListingMode::ByPage;
                self.page += 1;
            }
            MsgEventListing::PreviousPage => {
                self.listing_mode = EventListingMode::ByPage;
                if self.page > 0 {
                    self.page -= 1;
                }
            }
            MsgEventListing::DateChanged(d) => {
                self.listing_mode = EventListingMode::ByDate;
                self.date = d;
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.page = ctx.props().page;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let event_lister = match self.listing_mode {
            EventListingMode::ByPage => html! { <ListByPage page={self.page} page_size={15} /> },
            EventListingMode::ByDate => html! { <ListByDate date={ self.date } /> },
        };

        html! {
            <>
                <div class="container is-fluid">
                    <div class="columns">
                        <div class="column notification is-primary">
                            <nav class="pagination is-centered" role="navigation" aria-label="pagination">
                                <DateInputBox id={0} date={ self.date }
                                    callback={ctx.link().callback( MsgEventListing::DateChanged )}/>
                            </nav>
                        </div>
                        <div class="column">
                            <nav class="pagination is-centered" role="navigation" aria-label="pagination">
                                <a class="pagination-previous" onclick={ctx.link().callback(|_| MsgEventListing::FirstPage)} >{"First"}</a>
                                <a class="pagination-previous" onclick={ctx.link().callback(|_| MsgEventListing::PreviousPage)} >{"Previous"}</a>
                                <a class="pagination-next" onclick={ctx.link().callback(|_| MsgEventListing::NextPage)}>{"Next"}</a>
                            </nav>
                        </div>
                    </div>
                </div>
                { event_lister }
            </>
        }
    }
}
