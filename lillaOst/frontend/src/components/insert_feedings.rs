use std::cell::RefCell;
use std::rc::Rc;

use gloo_console::error;
use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};

use crate::event_bus::{EventBus, Request};

use ost::context_remote_async::AsyncRemoteMonolith;
use ost::feed::Feed as ost_Feed;
use ost::person::Person as ost_Person;

use super::inputs::u32_input_box::U32InputBox;

#[derive(Debug, PartialEq, Eq)]
pub enum MsgInsertFeedings {
    AddFeed,
    BreastMilkChanged(u32),
    FormulaChanged(u32),
    SolidsChanged(u32),
    CallFinished,
}

#[derive(Clone, Properties)]
pub struct PropsInsertFeedings {
    pub ost_person: Option<Rc<Box<dyn ost_Person>>>,
    pub ost_feeding: Option<Rc<RefCell<Box<dyn ost_Feed>>>>,
}

impl PartialEq for PropsInsertFeedings {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

pub struct InsertFeedings {
    formula: u32,
    breast_milk: u32,
    solids: u32,

    props: PropsInsertFeedings,
    is_new_insertion: bool,
    event_bus: Dispatcher<EventBus>,
}

impl InsertFeedings {
    fn reset_internal_state(&mut self) {
        self.formula = 0;
        self.breast_milk = 0;
        self.solids = 0;
    }

    fn has_no_data(&self) -> bool {
        self.formula == 0 && self.breast_milk == 0 && self.solids == 0
    }
}

impl Component for InsertFeedings {
    type Message = MsgInsertFeedings;
    type Properties = PropsInsertFeedings;

    fn create(ctx: &Context<Self>) -> Self {
        let mut is_new_insertion = true;

        let mut formula: u32 = 0;
        let mut breast_milk: u32 = 0;
        let mut solids: u32 = 0;

        if let Some(existing_feeding) = ctx.props().ost_feeding.as_ref() {
            breast_milk = existing_feeding.borrow().breast_milk();
            formula = existing_feeding.borrow().formula();
            solids = existing_feeding.borrow().solids();
            is_new_insertion = false;
        }

        Self {
            formula,
            breast_milk,
            solids,
            props: ctx.props().clone(),
            is_new_insertion,
            event_bus: EventBus::dispatcher(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgInsertFeedings::AddFeed => {
                if self.has_no_data() {
                    return false;
                }
                if self.props.ost_person.is_none() {
                    return false;
                }

                let p = self.props.ost_person.as_ref().unwrap().clone();
                add_feeding(ctx, p, self.breast_milk, self.formula, self.solids);
                self.reset_internal_state();
            }
            MsgInsertFeedings::CallFinished => {
                self.event_bus.send(Request::EventBusMsg(
                    "Insert feedings, call finished".to_owned(),
                ));
                return false;
            }
            MsgInsertFeedings::BreastMilkChanged(b) => {
                self.breast_milk = b;
                self.update_existing_feeding();
            }
            MsgInsertFeedings::FormulaChanged(f) => {
                self.formula = f;
                self.update_existing_feeding();
            }
            MsgInsertFeedings::SolidsChanged(s) => {
                self.solids = s;
                self.update_existing_feeding();
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props = ctx.props().clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let add_new_feeding_button = if self.is_new_insertion {
            html! {
                <div class="field is-grouped">
                <div class="control">
                    <button
                        class={ if self.has_no_data() { "button is-dark" } else { "button is-link" } }
                        onclick={ctx.link().callback(|_| MsgInsertFeedings::AddFeed)} >
                            {"Add Feed"}
                    </button>
                </div>
            </div>
            }
        } else {
            html! {}
        };

        html! {
        <div class="block">

            <U32InputBox id=0 label="Breast Milk" value={self.breast_milk} callback={
                    ctx.link().callback( MsgInsertFeedings::BreastMilkChanged )
            } />

            <U32InputBox id=1 label="Formula" value={self.formula} callback={
                ctx.link().callback( MsgInsertFeedings::FormulaChanged )
            } />

            <U32InputBox id=2 label="Solids" value={self.solids} callback={
                ctx.link().callback( MsgInsertFeedings::SolidsChanged )
            } />

            {add_new_feeding_button}
        </div>
        }
    }
}

impl InsertFeedings {
    fn update_existing_feeding(&mut self) {
        if self.props.ost_feeding.is_none() {
            return;
        }

        let breast_milk = self.breast_milk;
        let formula = self.formula;
        let solids = self.solids;

        if let Some(feeding) = &self.props.ost_feeding {
            #[allow(clippy::clone_on_copy)]
            let ts = feeding.borrow().time_stamp().clone();
            feeding
                .borrow_mut()
                .modify_feed(breast_milk, formula, solids, ts);
        } else {
            error!("Trying to modify a non existent feeding?");
        }
    }
}

fn add_feeding(
    ctx: &Context<InsertFeedings>,
    person: Rc<Box<dyn ost_Person>>,
    breast_milk: u32,
    formula: u32,
    solids: u32,
) {
    let p_cloned = person.clone();
    ctx.link().send_future(async move {
        let remote_context = AsyncRemoteMonolith {};
        let _ = remote_context
            .add_feeding(&p_cloned, breast_milk, formula, solids)
            .await;
        MsgInsertFeedings::CallFinished
    });
}
