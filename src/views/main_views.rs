use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::{database, View};
use dioxus::{html::style, prelude::*};

// use robius_open::Uri;
#[component]
pub fn main_view(view: Signal<View>) -> Element {
    let state: database::State = use_context();
    let entries = state.get_entries();
    let mut entries = use_signal(|| entries);
    let entries_iter = entries.read().clone();
    let mut confirm_delete = use_signal(|| None::<i32>);
    let mut hours = 0.0;
    let month_text = state.active_month_text.read().clone();
    let year = state.active_year.read().clone();
    for e in entries_iter.clone() {
        hours += e.hours;
    }

    rsx! {
                    div {
                    style: "display: flex;margin: 0px; padding: 0px; flex-direction: column; height: 100vh;background-color: #E0A0B0;",

                    // --- Top Section ---
                    div {
                        style: "display: flex; justify-content: space-between; padding: 0px;",
                        button {
                            style: "width: 50vw; background-color: #7A4D6B; color: white; border: 5px solid #E0A0B0; padding: 10px;",
                            onclick: move |_| {
                                view.set(View::ChooseProfile);
                            },
                            "{state.active_profile}"
                        }
                        button {
                            style: "width: 50vw; background-color: #7A4D6B; font-size: 20px; color: white; border: 5px solid #E0A0B0; padding: 10px;",
                            onclick: move |_| {
                                view.set(View::ChooseMonth);
                            },
                            "{month_text} {year}"
                        }
                    }

                    // --- Middle Section (Entries, Scrollable) ---
                     div {
            style: "flex-grow: 1; overflow-y: auto; padding: 0px;",

            // for e in entries_iter {

            //     div {

            //         style: "display: flex; align-items: center; justify-content: space-between; width: 95vw; margin: 6px; padding: 8px; border: 2px solid #E0A0B0; background-color: #A3B8B6; color: white;",


            //         // Clickable Entry Button
            //         button {
            //             style: "display: flex; flex-direction: column; align-items: center; justify-content: center; text-align: center; flex-grow: 1; background: none; border: none; color: white; padding: 0px;",
            //             onclick: move |_| {

            //             },

            //             // Date & Hours row
            //             div {
            //                 style: "display: flex; justify-content: space-between; align-items: center; width: 100%; padding-bottom: 4px;",

            //                 span {
            //                     style: "font-weight: bold;",
            //                     "{e.date}"
            //                 }
            //                 span {
            //                     style: "font-weight: bold; margin-right: 35px;",
            //                     "{e.hours} Stunden"
            //                 }
            //             }

            //             // Centered text
            //             span {
            //                 "{e.text}"
            //             }
            //         }

            //         // Delete button
            //         button {
            //             style: "width: 10vw; height: 10vh; background-color: red; color: white; border: none; padding: 0px;",
            //             onclick: move |_| {
            //                 confirm_delete.set(Some(e.entry_id));
            //             },
            //             span { "🗑️" }
            //         }
            //     }
            // }
        }


                    // --- Bottom Section ---
                    div {
                        style: "position: sticky; bottom: 0; background-color: #E0A0B0; padding: 10px; display: flex; justify-content: space-between;",

                        button {
                            style: "background-color: #7A4D6B; color: white; border: none; padding: 10px;",
                            onclick: move |_| {
                                state.sender.send("reload_entries".to_string()).unwrap();
                            },
                            "Gesamtstunden: {hours}"
                        }
                        button {
                            style: "background-color: #7A4D6B; color: white; border: none; padding: 10px;",
                            onclick: move |_| {
                                view.set(View::CreateEntry);
                            },
                            "Neuer Eintrag"
                        }
                    }
                }
                    if let Some(entry_id) = *confirm_delete.read() {
                            div {
                                style: "position: fixed; top: 30%; left: 50%; transform: translate(-50%, -50%); background: #E0A0B0; padding: 20px; border-radius: 5px; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);",
                                "Sind Sie sicher, dass Sie diesen Eintrag löschen möchten?"
                                div {
                                    button {
                                        style: "margin: 10px; padding: 10px; background: red; color: white; border: none;",
                                        onclick: move |_| {
                                            let mut state = use_context::<database::State>();
                                            state.delete_entry(entry_id);
                                            entries.set(state.get_entries());
                                            confirm_delete.set(None);
                                        },
                                        "Ja, löschen"
                                    }
                                    button {
                                        style: "margin: 10px; padding: 10px; background: gray; color: white; border: none;",
                                        onclick: move |_| {
                                            confirm_delete.set(None);
                                        },
                                        "Abbrechen"
                                    }
                                }
                            }
                        }


            }
}
