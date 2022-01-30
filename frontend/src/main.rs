#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all, rust_2018_idioms)]
#![feature(trait_upcasting)]
#![allow(incomplete_features)]

mod components;
mod event_bus;
mod pages;
mod root_spa;
use root_spa::RootSpa;
mod web_sys_utils;

fn main() {
    yew::start_app::<RootSpa>();
}
