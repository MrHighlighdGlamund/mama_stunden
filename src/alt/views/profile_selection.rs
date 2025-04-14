use std::{cell::RefCell, rc::Rc, sync::Arc};

use dioxus::prelude::*;

use crate::{
    database::{self},
    View,
};
#[component]
pub fn profile_selection(view: Signal<View>) -> Element {
    // let profiles = database::get_all_profiles();
    let mut state: database::State = use_context();
    let profiles = state.get_all_profiles();

    rsx! {

        for p in profiles {
            div {
            button {
                style: "flex: 1; background-color: #808080; color: white; border: none; padding: 10px; margin: 5px;",
                onclick: move |_| {
                    let mut state = use_context::<database::State>();
                    state.set_profile(p.clone());
                    state.get_last_month_with_entries();
                    println!("Selected profile: {}", state.active_profile);
                    view.set(View::MainView);
                },
                "{p}"
            }
            }

        }
    }
}
