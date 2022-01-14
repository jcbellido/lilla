use std::rc::Rc;

use web_sys::InputEvent;
use yew::prelude::*;

use ost::context_remote_async::AsyncRemoteMonolith;
use ost::person::Person as ost_Person;

use ost::person_key::OstPersonKey;

use crate::web_sys_utils::input_get_value_from_input_event;

#[derive(Clone)]
pub enum MsgSettingsIndividuals {
    CreateNewPerson,
    DataModified,
    DeactivatePerson { p_key: OstPersonKey },
    PersonsLoaded(Vec<Rc<Box<dyn ost_Person>>>),
    ReactivatePerson { p_key: OstPersonKey },
    UpdateInputName { name: String, p_key: OstPersonKey },
    UpdateNewName { name: String },
}

pub struct SettingsIndividuals {
    new_person_name: String,
    persons: Vec<Rc<Box<dyn ost_Person>>>,
    is_loaded: bool,
}

impl Component for SettingsIndividuals {
    type Message = MsgSettingsIndividuals;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        load_persons(ctx);
        Self {
            is_loaded: false,
            new_person_name: "".to_string(),
            persons: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgSettingsIndividuals::UpdateNewName { name } => {
                self.new_person_name = name;
            }
            MsgSettingsIndividuals::CreateNewPerson => {
                if self.new_person_name.is_empty() || self.new_person_name.trim().is_empty() {
                    self.new_person_name.clear();
                    return false;
                }

                let new_name = self.new_person_name.trim().to_string();

                ctx.link().send_future(async move {
                    let remote = AsyncRemoteMonolith {};
                    let _ = remote.add_person(&new_name).await;
                    MsgSettingsIndividuals::DataModified
                });

                self.new_person_name.clear();
                return false;
            }
            MsgSettingsIndividuals::DeactivatePerson { p_key } => {
                ctx.link().send_future(async move {
                    let remote = AsyncRemoteMonolith {};

                    let p = remote.get_person_by_key(p_key).await;

                    if let Some(mut p) = p {
                        p.set_is_active(false);
                        let _ = remote.modify_person(&p).await;
                    }
                    MsgSettingsIndividuals::DataModified
                });
                return false;
            }
            MsgSettingsIndividuals::ReactivatePerson { p_key } => {
                ctx.link().send_future(async move {
                    let remote = AsyncRemoteMonolith {};

                    let p = remote.get_person_by_key(p_key).await;

                    if let Some(mut p) = p {
                        p.set_is_active(true);
                        let _ = remote.modify_person(&p).await;
                    }
                    MsgSettingsIndividuals::DataModified
                });
                return false;
            }
            MsgSettingsIndividuals::UpdateInputName { p_key, name } => {
                ctx.link().send_future(async move {
                    let remote = AsyncRemoteMonolith {};

                    let p = remote.get_person_by_key(p_key).await;

                    if let Some(mut p) = p {
                        p.set_name(&name);
                        let _ = remote.modify_person(&p).await;
                    }
                    MsgSettingsIndividuals::DataModified
                });
                return false;
            }
            MsgSettingsIndividuals::PersonsLoaded(p) => {
                self.persons = p;
                self.is_loaded = true;
            }
            MsgSettingsIndividuals::DataModified => {
                load_persons(ctx);
                return false;
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.is_loaded {
            return html! {
                <>{"Loading persons"}</>
            };
        }

        let p_active_persons: Vec<_> = self
            .persons
            .iter()
            .filter(|p| p.is_active())
            .cloned()
            .collect();

        let mut active_individuals = html!();
        if !p_active_persons.is_empty() {
            active_individuals = html! {
                <>
                    <label class="label">{"Active tracked persons"}</label>
                    {
                        for p_active_persons.iter().map(|person| self.active_person_entry(person.clone(), ctx))
                    }
                </>
            };
        }

        let p_inactive_persons: Vec<_> = self
            .persons
            .iter()
            .filter(|p| !p.is_active())
            .cloned()
            .collect();

        let mut deactivated_individiuals = html!();
        if !p_inactive_persons.is_empty() {
            deactivated_individiuals = html! {
                <>
                    <label class="label">{"Archived persons"}</label>
                    {
                        for p_inactive_persons.iter().map(|person| self.archived_person_entry(person.clone(), ctx))
                    }
                </>
            };
        }

        html! {
        <div class="block">
            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">
                        {"Tracked persons"}
                    </p>
                </header>
                <div class="card-content">
                    <label class="label">{"Add a new person"}</label>
                    <div class="field is-horizontal">
                        <div class="field-body">
                            <div class="field">
                                <input class="input" placeholder={"New person name"}  value={ self.new_person_name.clone() }
                                    oninput={ ctx.link().callback(move |e: InputEvent| MsgSettingsIndividuals::UpdateNewName { name: input_get_value_from_input_event(e) }) }
                                />
                            </div>
                            <div class="control">
                                <button class="button is-link" onclick={ ctx.link().callback(|_| MsgSettingsIndividuals::CreateNewPerson ) } >{"Add Person"}</button>
                            </div>
                        </div>
                    </div>
                    {active_individuals}
                    {deactivated_individiuals}
                </div>
            </div>
        </div>
        }
    }
}

impl SettingsIndividuals {
    fn active_person_entry(&self, person: Rc<Box<dyn ost_Person>>, ctx: &Context<Self>) -> Html {
        if !person.is_active() {
            return html!();
        }
        let person_name = person.name().to_string();
        let person_to_deactivate = person.clone();
        html! {
            <div class="field is-horizontal">
                <div class="field-body">
                    <div class="field">
                        <input class="input"
                                value={person_name}
                                oninput={ctx.link().callback(move |e: InputEvent| MsgSettingsIndividuals::UpdateInputName{ p_key: person.key(), name: input_get_value_from_input_event(e) } ) }
                                />
                    </div>
                    <div class="control">
                        <button class="button is-warning" onclick={ctx.link().callback( move |_| MsgSettingsIndividuals::DeactivatePerson{ p_key: person_to_deactivate.key() } ) } >{"Deactivate"}</button>
                    </div>
                </div>
            </div>
        }
    }

    fn archived_person_entry(&self, person: Rc<Box<dyn ost_Person>>, ctx: &Context<Self>) -> Html {
        if person.is_active() {
            return html!();
        }

        let person_name = person.name().to_string();
        let person_to_activate = person.clone();

        html! {
            <div class="field is-horizontal">
                <div class="field-body">
                    <div class="field">
                        <input class="input" value={person_name}
                        oninput={ctx.link().callback(move |e: InputEvent| MsgSettingsIndividuals::UpdateInputName{ p_key: person.key(), name: input_get_value_from_input_event(e) } ) }
                        />
                    </div>
                    <div class="control">
                        <button class="button is-link" onclick={ctx.link().callback( move |_| MsgSettingsIndividuals::ReactivatePerson{ p_key: person_to_activate.key() } ) }  >{"Re Activate"}</button>
                    </div>
                </div>
            </div>
        }
    }
}

fn load_persons(ctx: &Context<SettingsIndividuals>) {
    ctx.link().send_future(async {
        let remote = AsyncRemoteMonolith {};
        let persons = remote.persons().await.drain(..).map(Rc::new).collect();
        MsgSettingsIndividuals::PersonsLoaded(persons)
    });
}
