use std::collections::HashMap;
use std::rc::Rc;

use chrono::prelude::*;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::event_bus::EventBus;

use ost::context_remote_async::AsyncRemoteMonolith;
use ost::person::Person as ost_Person;
use ost::person_key::OstPersonKey;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct CompFeeding {
    pub breast_milk: u64,
    pub formula: u64,
    pub solids: u64,
}

#[derive(Debug)]
pub enum MsgDailySummary {
    PersonsLoaded(Vec<Rc<Box<dyn ost_Person>>>),
    FeedingsForPerson {
        person_key: OstPersonKey,
        feeding_summary: CompFeeding,
    },
    RemoteDataChanged(String),
}

pub struct DailySummary {
    active_persons: Vec<Rc<Box<dyn ost_Person>>>,
    feedings: HashMap<OstPersonKey, CompFeeding>,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for DailySummary {
    type Message = MsgDailySummary;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        refresh_data(ctx);
        Self {
            active_persons: vec![],
            feedings: HashMap::<OstPersonKey, CompFeeding>::new(),
            _producer: EventBus::bridge(ctx.link().callback(MsgDailySummary::RemoteDataChanged)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgDailySummary::PersonsLoaded(mut p) => {
                self.active_persons = p.clone();
                p.drain(..).for_each(|p| {
                    let cloned_p = p.clone();
                    ctx.link().send_future(async move {
                        let remote = AsyncRemoteMonolith {};
                        let feedings = remote.feedings_by(&cloned_p).await;
                        let today = Utc::today();
                        let mut composed_feeding = CompFeeding::default();
                        feedings
                            .iter()
                            .filter(|f| f.time_stamp().date() == today)
                            .for_each(|feed| {
                                composed_feeding.breast_milk += feed.breast_milk() as u64;
                                composed_feeding.formula += feed.formula() as u64;
                                composed_feeding.solids += feed.solids() as u64;
                            });
                        MsgDailySummary::FeedingsForPerson {
                            person_key: p.key(),
                            feeding_summary: composed_feeding,
                        }
                    });
                });
                true
            }
            MsgDailySummary::FeedingsForPerson {
                person_key,
                feeding_summary,
            } => {
                let _ = self.feedings.insert(person_key, feeding_summary);
                true
            }
            MsgDailySummary::RemoteDataChanged(_) => {
                refresh_data(ctx);
                false
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if !self.active_persons.is_empty() {
            return html! {
                <div class="block">
                    <div class="notification is-primary">
                        <div class="table-container">
                            <table class="table is-fullwidth is-striped is-narrow">
                                <thead>
                                    <tr>
                                        <th>{"Person"}</th>
                                        <th>{"Breast"}</th>
                                        <th>{"Formula"}</th>
                                        <th>{"Solids"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {
                                        for self.active_persons
                                            .iter()
                                            .map(|person| self.construct_tably_daily_summary(person.clone()))
                                    }
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            };
        } else {
            html! { <div>{"No active individuals found"} </div>}
        }
    }
}

impl DailySummary {
    fn construct_tably_daily_summary(&self, person: Rc<Box<dyn ost_Person>>) -> Html {
        // look for the person in the internal hash table
        // if not found let's assume it's being loaded in the background

        if let Some(composed_feeding) = self.feedings.get(&person.key()) {
            html! {
                <tr>
                    <td>
                        { person.name() }
                    </td>
                    <td>
                        { composed_feeding.breast_milk }
                    </td>
                    <td>
                        { composed_feeding.formula }
                    </td>
                    <td>
                        { composed_feeding.solids }
                    </td>
                </tr>
            }
        } else {
            html! {
                <tr>
                    <td>
                        { person.name() }
                    </td>
                    <td>
                        { "loading" }
                    </td>
                    <td>
                        { "loading"  }
                    </td>
                    <td>
                        { "loading"  }
                    </td>
                </tr>
            }
        }
    }
}

fn refresh_data(ctx: &Context<DailySummary>) {
    ctx.link().send_future(async {
        let remote = AsyncRemoteMonolith {};
        let persons = remote
            .persons()
            .await
            .drain(..)
            .filter(|p| p.is_active())
            .map(Rc::new)
            .collect();
        MsgDailySummary::PersonsLoaded(persons)
    });
}
