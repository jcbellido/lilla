use std::rc::Rc;

use yew::prelude::*;

use super::insert_event::InsertEvent;
use super::insert_expulsions::InsertExpulsions;
use super::insert_feedings::InsertFeedings;

use ost::context_remote_async::AsyncRemoteMonolith;
use ost::person::Person as ost_Person;

#[derive(PartialEq)]
pub enum QuickInsertMode {
    Event,
    Expulsion,
    Feeds,
}

pub enum MsgQuickInsert {
    NextPerson,
    PreviousPerson,
    ModeSelection(QuickInsertMode),
    PersonsLoaded(Vec<Rc<Box<dyn ost_Person>>>),
}

pub struct QuickInsert {
    currently_selected: QuickInsertMode,
    active_person: Option<Rc<Box<dyn ost_Person>>>,
    active_person_index: Option<usize>,
    active_persons: Vec<Rc<Box<dyn ost_Person>>>,
}

impl Component for QuickInsert {
    type Message = MsgQuickInsert;
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
            MsgQuickInsert::PersonsLoaded(persons)
        });

        Self {
            currently_selected: QuickInsertMode::Feeds,
            active_person: None,
            active_persons: vec![],
            active_person_index: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgQuickInsert::NextPerson => {
                if self.active_persons.is_empty() {
                    return false;
                }
                let mut next_index = self.active_person_index.unwrap() + 1;
                if next_index >= self.active_persons.len() {
                    next_index = 0;
                }
                self.active_person = Some(self.active_persons.get(next_index).unwrap().clone());
                self.active_person_index = Some(next_index);
            }
            MsgQuickInsert::PreviousPerson => {
                if self.active_persons.is_empty() {
                    return false;
                }
                let mut next_index = self.active_person_index.unwrap();
                if next_index == 0 {
                    next_index = self.active_persons.len() - 1;
                } else {
                    next_index -= 1;
                }

                self.active_person = Some(self.active_persons.get(next_index).unwrap().clone());
                self.active_person_index = Some(next_index);
            }
            MsgQuickInsert::PersonsLoaded(p) => {
                self.active_persons = p;
                if self.active_persons.is_empty() {
                    self.active_person = None;
                    self.active_person_index = None;
                } else {
                    self.active_person = Some(self.active_persons.first().unwrap().clone());
                    self.active_person_index = Some(0);
                }
            }
            MsgQuickInsert::ModeSelection(m) => {
                self.currently_selected = m;
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.active_person.is_none() {
            return html! {
                <div>
                    { "no active person found" }
                </div>
            };
        }

        let feeds_class: Option<&str> = if self.currently_selected == QuickInsertMode::Feeds {
            Some("is-active")
        } else {
            None
        };

        let expulsion_class: Option<&str> = if self.currently_selected == QuickInsertMode::Expulsion
        {
            Some("is-active")
        } else {
            None
        };

        let event_class: Option<&str> = if self.currently_selected == QuickInsertMode::Event {
            Some("is-active")
        } else {
            None
        };

        let to_prop_active_person = self.active_person.as_ref().unwrap().clone();

        let active_form = match self.currently_selected {
            QuickInsertMode::Expulsion => {
                html! { < InsertExpulsions ost_person={Some(to_prop_active_person)}  /> }
            }
            QuickInsertMode::Feeds => {
                html! { < InsertFeedings ost_person={Some(to_prop_active_person)} /> }
            }
            QuickInsertMode::Event => {
                html! { < InsertEvent  ost_person={Some(to_prop_active_person)} /> }
            }
        };

        html! {
            <>
                <div class="block">
                    { self.person_selector(ctx) }
                </div>
                <div class="tabs is-toggle is-fullwidth ">
                    <ul>
                        <li class={classes!(feeds_class)}><a onclick={ctx.link().callback(|_| MsgQuickInsert::ModeSelection(QuickInsertMode::Feeds))} >{"Feeds"}</a></li>
                        <li class={classes!(expulsion_class)}><a onclick={ctx.link().callback(|_| MsgQuickInsert::ModeSelection(QuickInsertMode::Expulsion))}>{"Nappy"}</a></li>
                        <li class={classes!(event_class)}><a onclick={ctx.link().callback(|_| MsgQuickInsert::ModeSelection(QuickInsertMode::Event))}>{"Event"}</a></li>
                    </ul>
                </div>
                { active_form }
            </>
        }
    }
}

impl QuickInsert {
    fn person_selector(&self, ctx: &Context<Self>) -> Html {
        if self.active_person.is_none() {
            return html! {
                <div class="block">
                    {"No persons found. Configure one in settings."}
                </div>
            };
        }

        html! {
            <div style="display:flex; justify-content:space-between; padding:0; align-items: baseline;">
                <button class="button is-link" onclick={ctx.link().callback(|_| MsgQuickInsert::PreviousPerson )} >{ "Previous" }</button>
                { self.active_person.as_ref().unwrap().name() }
                <button class="button is-link"  onclick={ctx.link().callback(|_| MsgQuickInsert::NextPerson ) }>{ "Next" }</button>
            </div>
        }
    }
}
