use chrono::prelude::*;
use yew::prelude::*;

use ost::context_remote_async::AsyncRemoteMonolith;
use ost::person_key::OstPersonKey;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct CompFeeding {
    pub breast_milk: u64,
    pub formula: u64,
    pub solids: u64,
}

pub enum MsgSummaryPersonFeedingsOnDate {
    Feedings(Option<CompFeeding>),
}

#[derive(Clone, Properties, PartialEq)]
pub struct PropsSummaryPersonFeedingsOnDate {
    pub person_key: OstPersonKey,
    pub date: Date<Utc>,
}

pub struct SummaryPersonFeedingsOnDate {
    person_key: OstPersonKey,
    feedings: Option<CompFeeding>,
    date: Date<Utc>,
}

impl Component for SummaryPersonFeedingsOnDate {
    type Message = MsgSummaryPersonFeedingsOnDate;
    type Properties = PropsSummaryPersonFeedingsOnDate;

    fn create(ctx: &Context<Self>) -> Self {
        fetch_feeding_data_for(ctx, ctx.props().person_key, ctx.props().date);
        SummaryPersonFeedingsOnDate {
            person_key: ctx.props().person_key,
            feedings: None,
            date: ctx.props().date,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgSummaryPersonFeedingsOnDate::Feedings(f) => self.feedings = f,
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.person_key = ctx.props().person_key;
        self.date = ctx.props().date;
        self.feedings = None;
        fetch_feeding_data_for(ctx, self.person_key, self.date);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.feedings.is_none() {
            return html! {
                { "Fetching feeding data" }
            };
        }

        html! {
            <div>
            { format!("Breast Milk {}, Formula {}, Solids {}", self.feedings.as_ref().unwrap().breast_milk, self.feedings.as_ref().unwrap().formula, self.feedings.as_ref().unwrap().solids,) }
            </div>
        }
    }
}

fn fetch_feeding_data_for(
    ctx: &Context<SummaryPersonFeedingsOnDate>,
    person_key: OstPersonKey,
    date: Date<Utc>,
) {
    ctx.link().send_future(async move {
        let remote = AsyncRemoteMonolith {};
        let person = remote.get_person_by_key(person_key).await;
        if person.is_none() {
            return MsgSummaryPersonFeedingsOnDate::Feedings(None);
        }
        let feedings = remote.feedings_by(&person.unwrap()).await;
        let mut composed_feeding = CompFeeding::default();
        feedings
            .iter()
            .filter(|f| f.time_stamp().date() == date)
            .for_each(|feed| {
                composed_feeding.breast_milk += feed.breast_milk() as u64;
                composed_feeding.formula += feed.formula() as u64;
                composed_feeding.solids += feed.solids() as u64;
            });
        MsgSummaryPersonFeedingsOnDate::Feedings(Some(composed_feeding))
    })
}
