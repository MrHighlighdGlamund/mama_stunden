use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::{database, View, MAIN_CSS};
use dioxus::{html::style, prelude::*};

// use robius_open::Uri;
#[component]
pub fn main_view(view: Signal<View>) -> Element {
    let state: database::State = use_context();

    let entries = state.get_entries();
    let mut entries = use_signal(|| entries);
    let entries_iter = entries.read().clone();
    let mut confirm_delete = use_signal(|| None::<i32>);
    let mut choose_profile = use_signal(|| None::<i32>);
    let mut choose_month = use_signal(|| None::<i32>);
    let mut hours = 0.0;
    let month_text = state.active_month_text.read().clone();
    let year = state.active_year.read().clone();
    let overtime = state.get_overtime();
    for e in entries_iter.clone() {

        hours += e.hours;
    }
    hours = hours + overtime;

    let mut reload_profile = move || {
        let mut state = use_context::<database::State>();
        state.get_last_month_with_entries();
        let new_entries = state.get_entries(); // Fetch new entries based on the updated month/year
        entries.set(new_entries); // Update the entries signal
    };
    let mut reload_entries = move || {
        let state = use_context::<database::State>();
        let new_entries = state.get_entries(); // Fetch new entries based on the updated month/year
        entries.set(new_entries); // Update the entries signal
    };

    rsx! {


                        div {
                        style: "display: flex;margin: 0px; padding: 0px; flex-direction: column; height: 100vh;background-color: #E0A0B0;",

                        // --- Top Section ---
                        div {
                            style: "display: flex; justify-content: space-between; padding: 0px;",
                            button {
                                style: "width: 50vw; border-radius: 100px; font-size: 28px; background-color: #7A4D6B; color: white; border: 5px solid #E0A0B0; padding: 10px;",
                                onclick: move |_| {
                                    choose_profile.set(Some(1));
                                },
                                "{state.active_profile}"
                            }
                            button {
                                style: "width: 50vw; border-radius: 100px;font-size: 28px;background-color: #7A4D6B; color: white; border: 5px solid #E0A0B0; padding: 10px;",
                                onclick: move |_| {
                                    choose_month.set(Some(1));
                                },
                                "{month_text} {year}"
                            }
                        }

                        // --- Middle Section (Entries, Scrollable) ---
                         div {
                style: "flex-grow: 1; overflow-y: auto; padding: 0px;",
    div {

                        style: "display: flex; align-items: center;  border-radius: 90px; justify-content: space-between; width: 95vw; margin: 5px; padding: 5px; border: 1px solid #7A4D6B; background-color: #A3B8B6; color: white;",


                        // Clickable Entry Button
                        button {
                            style: "display: flex;;  font-size: 22px; border-radius: 100px; flex-direction: column; align-items: center; justify-content: center; text-align: center; flex-grow: 1; background: none; border: none ; color: white; padding: 0px;",
                            onclick: move |_| {
                            },

                            // Date & Hours row
                            div {
                                style: "display: flex; justify-content: space-between; align-items: center; width: 100%; padding-bottom: 4px;",

                                span {
                                    style: "font-weight: bold; margin-left: 15px;",
                                    "Letzter Monat"
                                }
                                span {
                                    style: "font-weight: bold; margin-right: 30px;",
                                    "{overtime} Std."
                                }
                            }

                            // Centered text
                            span {
                                "Übertragene Stunden"
                            }
                        }

                        // Delete button
                        
                    }



                for e in entries_iter {
                    // create link to www.google.de


                    div {

                        style: "display: flex; align-items: center;  border-radius: 90px; justify-content: space-between; width: 95vw; margin: 5px; padding: 5px; border: 1px solid #7A4D6B; background-color: #A3B8B6; color: white;",


                        // Clickable Entry Button
                        button {
                            style: "display: flex;;  font-size: 22px; border-radius: 100px; flex-direction: column; align-items: center; justify-content: center; text-align: center; flex-grow: 1; background: none; border: none ; color: white; padding: 0px;",
                            onclick: move |_| {

        // Uri::new("file:///storage/emulated/0/Download/test.pdf")
        //     .open()
        //     .expect("failed to open URL");


                            },

                            // Date & Hours row
                            div {
                                style: "display: flex; justify-content: space-between; align-items: center; width: 100%; padding-bottom: 4px;",

                                span {
                                    style: "font-weight: bold; margin-left: 15px;",
                                    "{e.date}"
                                }
                                span {
                                    style: "font-weight: bold; margin-right: 30px;",
                                    "{e.hours} Std."
                                }
                            }

                            // Centered text
                            span {
                                "{e.text}"
                            }
                        }

                        // Delete button
                        button {
                            style: "width: 18vw; height: 10vh; font-size: 28px; border-radius: 80px; background-color: red; color: white; border: none; padding: 0px;",
                            onclick: move |_| {
                                let state = use_context::<database::State>();
                                confirm_delete.set(Some(e.entry_id));
                            },
                            span { "🗑️" }
                        }
                    }
                }
            }


                        // --- Bottom Section ---
                        div {
                            style: "position: sticky; bottom: 0; background-color: #E0A0B0; padding: 10px; display: flex; justify-content: space-between;",

                            button {
                                style: "width: 45vw; background-color: #7A4D6B; font-size: 26px; color: white; border-radius: 100px; border: none; padding: 8px;",
                                onclick: move |_| {
                                    let mut state = use_context::<database::State>();
                                    state.print_pdf();
                                },
                                div {
                                    span { "~Gesamt~" }
                                }
                                div {

                                    span { "{month_text}" }
                                }
                                div {

                                    span { "{hours} Std." }
                                }

                            }
                            button {
                                style: "width: 40vw; background-color: #7A4D6B; font-size: 26px; border-radius: 100px; color: white; border: none; padding: 8px;",
                                onclick: move |_| {
                                    view.set(View::CreateEntry);
                                },
                                    div {
                                    span { "+++" }
                                }
                                div {

                                    span { "neuer Eintrag" }
                                }
                            }
                        }
                    }
                        if let Some(entry_id) = *confirm_delete.read() {
                                div {
                                    style: "position: fixed; color: white; text-align: center; font-size: 24px; width: 60%; height: 40%; top: 40%; left: 50%; transform: translate(-50%, -50%); background: #E0A0B0; padding: 20px; border-radius: 5px; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);",
                                    "Sind Sie sicher, dass Sie diesen Eintrag löschen möchten?"
                                    div {
                                        style: "width: 100%; height: 60%;  align-items: center; padding-top: 10px;",
                                        div {

                                        style: "width: 100%; height: 50%; space-between; align-items: center; padding-top: 10px;",

                                        button {
                                            style: "width: 100%; height: 100%; border-radius: 100px; font-size: 30px;  background: red; color: white; border: none;",
                                            onclick: move |_| {
                                                let mut state = use_context::<database::State>();
                                                state.delete_entry(entry_id);
                                                entries.set(state.get_entries());
                                                confirm_delete.set(None);
                                            },
                                            "Ja, löschen"
                                        }
                                        }
                                        div {

                                        style: "width: 100%; height: 50%;  align-items: center; padding-top: 10px;",
                                        button {
                                            style: "width: 100%; border-radius: 100px; font-size: 30px; height: 100%; background: gray; color: white; border: none;",
                                            onclick: move |_| {
                                                confirm_delete.set(None);
                                            },
                                            "Abbrechen"
                                        }
                                        }
                                    }
                                }
                            }
                    if let Some(entry_id) = *choose_month.read() {
                                div {
                                    style: "font-size: 30px; color: white; text-align: center; width: 70%; height: 50%; position: fixed; top: 43%; left: 50%; transform: translate(-50%, -50%); background: #E0A0B0; padding: 20px; border-radius: 5px; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);",
                                   "Monat auswaehlen"
                                       div {
                                           style: "width: 100%; height: 60%; padding-bottom: 20px;  align-items: center; padding-top: 10px; justify-content: center;",

                                    div {
                                                style: "width: 100%;height: 30%;padding-bottom: 20px; background-color: #E0A0B0; color: white; border-radius: 100px; border: none;  justify-content: center;",

                                            select {

            style: "background-color: #7A4D6B; color: white; position: relative; font-size: 30px; padding: 10px; border: none;
            border-radius: 100px; appearance: none; text-align: center; text-align-last: center; 
            width: 100%; height: 100%; justify-content: center;",
                                                // style: "margin: 0px;width: 100%;height: 100%; background-color: #E0A0B0; color: black; border: none; padding: 10px;",
                                                value: "{state.active_month_text.read().clone()}",
                                        onchange: move |e| {
                                            let mut state = use_context::<database::State>();
                                            let selected_month = e.value(); // Get the selected month
                                            let active_year = state.active_year.read().clone();
                                            state.set_month(selected_month, active_year);
                                            reload_entries();
                                            choose_month.set(None);


                                        },
                                        option { value: "Januar", "Januar" }
                                        option { value: "Februar", "Februar" }
                                        option { value: "Maerz", "März" }
                                        option { value: "April", "April" }
                                        option { value: "Mai", "Mai" }
                                        option { value: "Juni", "Juni" }
                                        option { value: "Juli", "Juli" }
                                        option { value: "August", "August" }
                                        option { value: "September", "September" }
                                        option { value: "Oktober", "Oktober" }
                                        option { value: "November", "November" }
                                        option { value: "Dezember", "Dezember" }
                                    }
                                        }
                                        div {

                                                style: "width: 100%;height: 30%; padding-bottom: 40px; background-color: #E0A0B0; color: white; border-radius: 100px; border: none;  justify-content: center;",
                                            select {

            style: "background-color: #7A4D6B; color: white; position: relative; font-size: 30px; padding: 10px; border: none;
            border-radius: 100px; appearance: none; text-align: center; text-align-last: center; 
            width: 90%; height: 100%; justify-content: center;",
                                                value: "{state.active_year.read().clone()}",
                                            onchange: move |e| {
                                                let selected_year = e.value().parse::<i32>().unwrap(); // Get the selected year
                                                let mut state = use_context::<database::State>();
                                                let active_month = state.active_month_text.read().clone();
                                                state.set_month(active_month, selected_year);
                                                reload_entries();

                                            choose_month.set(None);
                                            },
                                            option { value: "2025", "2025" }
                                            option { value: "2026", "2026" }
                                            option { value: "2027", "2027" }
                                            option { value: "2028", "2028" }
                                            option { value: "2029", "2029" }
                                            option { value: "2030", "2030" }
                                            option { value: "2031", "2031" }
                                            option { value: "2032", "2032" }
                                            option { value: "2033", "2033" }
                                            option { value: "2034", "2034" }
                                            option { value: "2035", "2035" }
                                            option { value: "2036", "2036" }
                                            option { value: "2037", "2037" }
                                            option { value: "2038", "2038" }
                                            option { value: "2039", "2039" }
                                            option { value: "2040", "2040" }
                                        }
                                        }
                                        div {


                                                style: "width: 100%;height: 30%;  background-color: #E0A0B0; color: white; border-radius: 100px; border: none;  justify-content: center;",
                                        button {

            style: "background-color: #7A4D6B; color: white; position: relative; font-size: 30px; padding: 10px; border: none;
            border-radius: 100px; appearance: none; text-align: center; text-align-last: center; 
            width: 70%; height: 100%; justify-content: center;",
                                            onclick: move |_| {
                                                choose_month.set(None);

                                            },
                                            "Zurück"
                                    }
                                        }
                                    }
                                }
                            }
                if let Some(entry_id) = *choose_profile.read() {
                div {
                    style: "position: fixed; width: 70vw; height: 50vh; top: 42%; left: 50%; transform: translate(-50%, -50%);
                background: black; padding: 0px; border-radius: 10px; 
                box-shadow: 0 12px 12px rgba(0, 0, 0, 0.1); display: flex; flex-direction: column; 
                align-items: center; font-size: 30px; text-align: center; margin-bottom: 20px; background-color: #E0A0B0; color: white;",
                    "Profil auswählen"

                    // --- Gaby Button Container ---
                    div {
                        style: "width: 89%; height: 30%; display: flex; align-items: center;padding-top: 20px; padding-bottom: 20px; justify-content: center; margin: 5px; background: #E0A0B0;",

                        button {
                            style: "width: 100%; height: 100%; background: #7A4D6B; padding: 10px; margin: 10px; border-radius: 100px; font-size: 35px; color: white; border: none; display: flex; align-items: center; justify-content: center; ",
                            onclick: move |_| {
                                let mut state = use_context::<database::State>();
                                state.set_profile("Gaby".to_string());
                                choose_profile.set(None);
                                reload_profile();
                            },
                            "Gaby"
                        }
                    }

                    // --- Hermann Button Container ---
                    div {
                        style: "width: 89%; height: 30%; display: flex; align-items: center; padding-top: 20px; justify-content: center; margin: 5px; padding: 0px; background: #E0A0B0;",

                        button {

                            style: "width: 100%; height: 100%; background: #7A4D6B; padding: 10px; margin: 10px; border-radius: 100px; font-size: 35px; color: white; border: none; display: flex; align-items: center; justify-content: center;",
                            onclick: move |_| {
                                let mut state = use_context::<database::State>();
                                state.set_profile("Hermann".to_string());
                                choose_profile.set(None);
                                reload_profile();
                            },
                            "Hermann"
                        }
                    }
                }
                }
                }
}
