use std::collections::HashMap;
use std::rc::Rc;

use chrono::prelude::*;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::event_bus::EventBus;
use ost::context_remote_async::AsyncRemoteMonolith;
use ost::person::Person;
use ost::person_key::OstPersonKey;

#[derive(Default)]
pub struct CompExpulsion {
    pub total: u16,
    pub solid_expulsion: u16,
}

pub enum MsgSummaryExpulsions {
    ExpulsionForPerson {
        person_key: OstPersonKey,
        feeding_summary: CompExpulsion,
    },
    PersonsLoaded(Vec<Rc<Box<dyn Person>>>),
    RemoteDataChanged(String),
}

pub struct SummaryExpulsions {
    active_persons: Vec<Rc<Box<dyn Person>>>,
    expulsions: HashMap<OstPersonKey, CompExpulsion>,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for SummaryExpulsions {
    type Message = MsgSummaryExpulsions;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        request_active_persons(ctx);
        Self {
            active_persons: vec![],
            expulsions: HashMap::<OstPersonKey, CompExpulsion>::new(),
            _producer: EventBus::bridge(
                ctx.link().callback(MsgSummaryExpulsions::RemoteDataChanged),
            ),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgSummaryExpulsions::RemoteDataChanged(_) => {
                request_active_persons(ctx);
                return false;
            }
            MsgSummaryExpulsions::PersonsLoaded(mut persons) => {
                self.active_persons = persons.clone();
                persons.drain(..).for_each(|p| {
                    let cloned_p = p.clone();
                    ctx.link().send_future(async move {
                        let remote = AsyncRemoteMonolith {};
                        let expulsions = remote.expulsions_by(&cloned_p).await;
                        let today = Utc::today();

                        let mut summary_expulsions = CompExpulsion::default();
                        expulsions
                            .iter()
                            .filter(|f| f.time_stamp().date() == today)
                            .for_each(|exp| {
                                summary_expulsions.total += 1;
                                match exp.degree() {
                                    ost::expulsion::ExpulsionDegree::Shart => {
                                        summary_expulsions.solid_expulsion += 1
                                    }
                                    ost::expulsion::ExpulsionDegree::Poopies => {
                                        summary_expulsions.solid_expulsion += 1
                                    }
                                    ost::expulsion::ExpulsionDegree::Pooplosion => {
                                        summary_expulsions.solid_expulsion += 1
                                    }
                                    _ => {}
                                }
                            });
                        MsgSummaryExpulsions::ExpulsionForPerson {
                            person_key: p.key(),
                            feeding_summary: summary_expulsions,
                        }
                    });
                });
            }
            MsgSummaryExpulsions::ExpulsionForPerson {
                person_key,
                feeding_summary,
            } => {
                let _ = self.expulsions.insert(person_key, feeding_summary);
            }
        }
        true
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.active_persons.is_empty() {
            return html! {"No expulsions summary available: No active individuals found"};
        }
        html! {
            <div class="block">
                <div class="notification is-info">
                    <div class="table-container">
                        <table class="table is-fullwidth is-striped is-narrow">
                            <thead>
                                <tr>
                                    <th>{"Person"}</th>
                                    <th>{"Poops today"}</th>
                                    <th>{"Total Nappies"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {
                                    for self.active_persons
                                        .iter()
                                        .map(|person| self.construct_daily_summary_row_for(person.clone()))
                                }
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }
}

impl SummaryExpulsions {
    fn construct_daily_summary_row_for(&self, person: Rc<Box<dyn Person>>) -> Html {
        if let Some(summary_expulsion) = self.expulsions.get(&person.key()) {
            html! {
                <tr>
                    <td>
                        { person.name() }
                    </td>
                    <td>
                        { summary_expulsion.solid_expulsion }
                    </td>
                    <td>
                        { summary_expulsion.total }
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
                </tr>
            }
        }
    }
}

fn request_active_persons(ctx: &Context<SummaryExpulsions>) {
    ctx.link().send_future(async {
        let remote = AsyncRemoteMonolith {};
        let persons = remote
            .persons()
            .await
            .drain(..)
            .filter(|p| p.is_active())
            .map(Rc::new)
            .collect();
        MsgSummaryExpulsions::PersonsLoaded(persons)
    });
}
