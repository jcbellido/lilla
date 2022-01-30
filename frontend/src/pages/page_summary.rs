use std::rc::Rc;

use yew::prelude::*;

use crate::components::graphs::graph_feedings_individual::GraphFeedingsIndividual;
use crate::components::graphs::graph_feedings_individual_all_time::GraphFeedingsIndividualAllTime;
use crate::components::summary_on_date::SummaryOnDate;

use ost::context_remote_async::AsyncRemoteMonolith;
use ost::person::Person as ost_Person;

#[derive(Debug)]
pub enum MsgPageSummary {
    MsgPersonsLoaded(Vec<Rc<Box<dyn ost_Person>>>),
}

pub struct PageSummary {
    active_persons: Vec<Rc<Box<dyn ost_Person>>>,
    is_loading: bool,
}

impl Component for PageSummary {
    type Message = MsgPageSummary;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            let remote = AsyncRemoteMonolith {};
            let persons = remote
                .persons()
                .await
                .drain(..)
                .filter(|p| p.is_active())
                .map(Rc::new)
                .collect();
            MsgPageSummary::MsgPersonsLoaded(persons)
        });

        Self {
            active_persons: vec![],
            is_loading: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgPageSummary::MsgPersonsLoaded(p) => {
                self.is_loading = false;
                self.active_persons = p;
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.is_loading {
            return html! {
                <div>
                    {"Loading persons."}
                </div>
            };
        }

        if self.active_persons.is_empty() {
            return html! {
                <div>
                    {"No persons found."}
                </div>
            };
        }

        let mut id: u32 = 0;
        let mut summaries: Vec<Html> = vec![];
        self.active_persons.iter().for_each(|p| {
            summaries.push(self.summarize_individual(p.clone(), id));
            id += 1;
        });

        html! {
            <div>
                <div class="block"></div>
                {summaries}
            </div>
        }
    }
}

impl PageSummary {
    fn summarize_individual(&self, person: Rc<Box<dyn ost_Person>>, id: u32) -> Html {
        let person_name = person.name().to_string();
        html! {
            <div class="block">
                <SummaryOnDate person_key={ person.key() } person_name={ person_name } />
                <GraphFeedingsIndividual person={person.clone()} id={id} />
                <GraphFeedingsIndividualAllTime person={person.clone()} id={id} />
            </div>
        }
    }
}
