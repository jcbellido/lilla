use chrono::prelude::*;
use yew::prelude::*;

use ost::context_remote_async::AsyncRemoteMonolith;
use ost::person_key::OstPersonKey;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct CompExpulsion {
    pub total: u16,
    pub solid_expulsions: u16,
}

pub enum MsgSummaryPersonExpulsionsOnDate {
    Expulsions(Option<CompExpulsion>),
}

#[derive(Clone, Properties, PartialEq)]
pub struct PropsSummaryPersonExpulsionsOnDate {
    pub person_key: OstPersonKey,
    pub date: Date<Utc>,
}

pub struct SummaryPersonExpulsionsOnDate {
    person_key: OstPersonKey,
    expulsions: Option<CompExpulsion>,
    date: Date<Utc>,
}

impl Component for SummaryPersonExpulsionsOnDate {
    type Message = MsgSummaryPersonExpulsionsOnDate;
    type Properties = PropsSummaryPersonExpulsionsOnDate;

    fn create(ctx: &Context<Self>) -> Self {
        fetch_expulsion_data_for(ctx, ctx.props().person_key, ctx.props().date);
        SummaryPersonExpulsionsOnDate {
            person_key: ctx.props().person_key,
            expulsions: None,
            date: ctx.props().date,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgSummaryPersonExpulsionsOnDate::Expulsions(f) => self.expulsions = f,
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.person_key = ctx.props().person_key;
        self.date = ctx.props().date;
        self.expulsions = None;
        fetch_expulsion_data_for(ctx, self.person_key, self.date);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.expulsions.is_none() {
            return html! {
                { "Fetching expulsion data" }
            };
        }

        html! {
            <div>
                { format!("Expulsions: solids {} / total {}", self.expulsions.as_ref().unwrap().solid_expulsions, self.expulsions.as_ref().unwrap().total) }
            </div>
        }
    }
}

fn fetch_expulsion_data_for(
    ctx: &Context<SummaryPersonExpulsionsOnDate>,
    person_key: OstPersonKey,
    date: Date<Utc>,
) {
    ctx.link().send_future(async move {
        let remote = AsyncRemoteMonolith {};
        let person = remote.get_person_by_key(person_key).await;
        if person.is_none() {
            return MsgSummaryPersonExpulsionsOnDate::Expulsions(None);
        }
        let feedings = remote.expulsions_by(&person.unwrap()).await;
        let mut composed_expulsion = CompExpulsion::default();
        feedings
            .iter()
            .filter(|f| f.time_stamp().date() == date)
            .for_each(|exp| {
                composed_expulsion.total += 1;
                match exp.degree() {
                    ost::expulsion::ExpulsionDegree::Shart => {
                        composed_expulsion.solid_expulsions += 1
                    }
                    ost::expulsion::ExpulsionDegree::Poopies => {
                        composed_expulsion.solid_expulsions += 1
                    }
                    ost::expulsion::ExpulsionDegree::Pooplosion => {
                        composed_expulsion.solid_expulsions += 1
                    }
                    _ => {}
                }
            });
        MsgSummaryPersonExpulsionsOnDate::Expulsions(Some(composed_expulsion))
    })
}
