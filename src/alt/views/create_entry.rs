use std::{cell::RefCell, rc::Rc, sync::Arc};

use dioxus::prelude::*;

use crate::{database, View};
#[component]
pub fn create_entry(view: Signal<View>) -> Element {
    let mut selected_date = use_signal(|| "".to_string());
    let mut text = use_signal(|| "".to_string());
    let mut hours = use_signal(|| "".to_string()); // Store as a string to match event value type

    rsx! {
        div {
            // Back button
            button {
                style: "margin: 10px; background-color: #808080; color: white; border: none; padding: 10px;",
                onclick: move |_| {
                    view.set(View::MainView);
                },
                "Zurueck"
            }
        }

        div {
            // Date input
            input {
                r#type: "date",
                value: "{selected_date}",
                style: "padding: 10px; margin: 5px; border: 1px solid #ccc; border-radius: 5px;",
                onchange: move |event| {
                    selected_date.set(event.value());
                }
            }
            p { "Selected Date: {selected_date}" }
        }

        div {
            // Text input
            input {
                r#type: "text",
                value: "{text}",
                placeholder: "Beschreibung",
                style: "padding: 10px; margin: 5px; border: 1px solid #ccc; border-radius: 5px;",
                onchange: move |event| {
                    text.set(event.value());
                }
            }
        }

        div {
            // Number input for hours
            input {
                r#type: "number",
                value: "{hours}",
                placeholder: "Stunden",
                style: "padding: 10px; margin: 5px; border: 1px solid #ccc; border-radius: 5px;",
                onchange: move |event| {
                    hours.set(event.value());
                }
            }
        }
        // Create button
        button {
            style: "margin: 10px; background-color: #808080; color: white; border: none; padding: 10px;",
            onclick: move |_| {
                let entry = database::Entry {
                    entry_id: 0,
                    date: selected_date.read().clone(),
                    text: text.read().clone(),
                    hours: hours.read().parse().unwrap(),
                };

                let state = use_context::<database::State>();
                state.save_entry(entry);

                view.set(View::MainView);
            },
            "Speichern"
        }
    }
}

