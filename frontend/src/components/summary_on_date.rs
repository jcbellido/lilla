use chrono::prelude::*;
use yew::prelude::*;

use super::inputs::date_input_box::DateInputBox;
use super::summary_person_expulsions_on_date::SummaryPersonExpulsionsOnDate;
use super::summary_person_feedings_on_date::SummaryPersonFeedingsOnDate;

use ost::person_key::OstPersonKey;

// Starts without a date defined ... or begins with today (option)
#[derive(Clone, Properties, PartialEq)]
pub struct PropsMsgSummaryDate {
    pub person_key: OstPersonKey,
    pub person_name: String,
}

pub enum MsgSummaryDate {
    DateChanged(Date<Utc>),
}

pub struct SummaryOnDate {
    person_key: OstPersonKey,
    pub person_name: String,
    date: Date<Utc>,
}

impl Component for SummaryOnDate {
    type Message = MsgSummaryDate;
    type Properties = PropsMsgSummaryDate;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            person_key: ctx.props().person_key,
            person_name: ctx.props().person_name.clone(),
            date: Utc::today(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgSummaryDate::DateChanged(d) => {
                self.date = d;
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.person_key = ctx.props().person_key;
        self.person_name = ctx.props().person_name.clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="block">
                <div class="notification is-info">
                    { &self.person_name }
                    <DateInputBox id={ self.person_key.id } date={ self.date }
                        callback={ctx.link().callback( MsgSummaryDate::DateChanged )}/>
                    <SummaryPersonFeedingsOnDate person_key={ self.person_key } date={ self.date } />
                    <SummaryPersonExpulsionsOnDate person_key={ self.person_key } date={ self.date } />
                </div>
            </div>
        }
    }
}
