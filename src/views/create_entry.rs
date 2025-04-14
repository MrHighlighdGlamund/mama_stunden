use chrono::{Datelike, NaiveDate};
use dioxus::prelude::*;
use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{database, View};

const PREDEFINED_YEAR: i32 = 2025;
const PREDEFINED_MONTH: u32 = 6; // June

#[component]
pub fn create_entry(view: Signal<View>) -> Element {
    let state = use_context::<database::State>();
    let mut selected_date = use_signal(|| "".to_string());
    let mut number = use_signal(|| 0);
    let mut minutes = use_signal(|| 0); // Default to 0
    let mut formatted_date = use_signal(|| "".to_string());
    let mut text = use_signal(|| "".to_string());

    let mut hours = use_signal(|| "".to_string());
    let mut hours_float = use_signal(|| 0.0);
    let mut plusorminus = use_signal(|| 1);
    let current_month = state.active_month_number.read().clone();
    let current_year = state.active_year.read().clone();
    let options = vec![
        "".to_string(),
        "15".to_string(),
        "30".to_string(),
        "45".to_string(),
    ];

    use_effect(move || {
        let entry = state.get_last_entry();
        text.set(entry.text.clone());

        if let Ok(parsed_date) = NaiveDate::parse_from_str(&entry.date, "%d-%m-%Y") {
            let mut next_day = parsed_date.succ();
            if next_day.day() == 1 {

             next_day = next_day.pred();
                
            }
            // if day is one go back a day
            //
            formatted_date.set(next_day.format("%d-%m-%Y").to_string());
            let formatted_dat = next_day.format("%Y-%m-%d").to_string();
            selected_date.set(formatted_dat);
        } else {
            let next_day = NaiveDate::from_ymd_opt(current_year, current_month as u32, 1).unwrap();
            formatted_date.set(next_day.format("%d-%m-%Y").to_string());
            let formatted_dat = next_day.format("%Y-%m-%d").to_string();
            selected_date.set(formatted_dat);
        }

        // selected_date.set(entry.date.clone());
        println!("Entry hours: {}", entry.hours);
        let hours_as_int = entry.hours as i32;
        // let min = hours_as_int % 100;
        number.set(hours_as_int);
        let min = (entry.hours.fract() * 100.0) as i32;
        match min {
            25 => minutes.set(15),
            50 => minutes.set(30),
            75 => minutes.set(45),
            _ => minutes.set(0),
        }
    });

    let min_date = format!("{}-{:02}-01", current_year, current_month);
    let last_day = NaiveDate::from_ymd_opt(current_year, current_month as u32 + 1, 1)
        .unwrap()
        .pred()
        .day();
    let max_date = format!("{}-{:02}-{:02}", current_year, current_month, last_day);

    rsx! {
                            div {
                                style: "display: flex; overflow-y: none; flex-direction: column; align-items: center; height: 100vh; background-color: #E0A0B0; gap: 20px;",
                                div {
                                    style: "width: 100%;; display: flex; flex-direction: column; align-items: center; gap: 10px; padding: 10px;",

                                    input {
                                        r#type: "date",
                                        min: "{min_date}",
                                        max: "{max_date}",
                                        value: "{selected_date}",
                                        style: "background-color: #7A4D6B; min-height: 50px; min-width: 80%; color: white; font-size: 40px; border-radius: 100px; border: none; min-width: 60%; width: auto; text-align: center; display: inline-block;",
                                        onchange: move |event| {
                                            let raw_date = event.value();
                                            selected_date.set(raw_date.clone());

                                            if let Ok(parsed_date) = NaiveDate::parse_from_str(&raw_date, "%Y-%m-%d") {
                                                let formatted_dat = parsed_date.format("%d-%m-%Y").to_string();
                                                formatted_date.set(formatted_dat);
                                            }
                                        }
                                    }
                                }
            div {
        style: "width: 100%; display: flex; flex-direction: column; justify-content: center; align-items: center; background-color: #E0A0B0; gap: 20px;",
        // Wrapper to keep buttons & input side by side
        div {
            style: "display: flex; align-items: center; gap: 20px;", // FLEX for side-by-side layout

            button {
                style: "background-color: #7A4D6B; color: white; font-size: 80px; border-radius: 30px; border: none; width: 80px; height: 70px; text-align: center; display: flex; align-items: center; justify-content: center;",
                onclick: move |_| {
                    number.set(number() - 1);
                },
                "-"
            }

            input {
                style: "background-color: #7A4D6B; color: white; font-size: 60px; border-radius: 30px; border: none; width: 200px; height: 70px; text-align: center;",
                r#type: "number",
                value: "{number}",
                step: "1",
                oninput: move |e| {
                    if let Ok(val) = e.value().parse::<i32>() {
                        number.set(val); // Allows negative values
                    }
                }
            }

            button {
                style: "background-color: #7A4D6B; color: white; font-size: 80px; border-radius: 30px; border: none; width:80px; height: 70px; text-align: center; display: flex; align-items: center; justify-content: center;",
                onclick: move |_| {
                    number.set(number() + 1);
                },
                "+"
            }
        }
    }

    div {
        style: "display: flex; align-items: center; gap: 20px;", // FLEX for side-by-side layout
        button {
            style: "background-color: #7A4D6B; color: white; font-size: 80px; border-radius: 30px; border: none; width: 80px; height: 70px; text-align: center; display: flex; align-items: center; justify-content: center;",
            onclick: move |_| {
                let current_value = minutes();
                if current_value - 15 >= 0 {
                    minutes.set(current_value - 15); // Decreases by 15 with min 0
                }
            },
            "-"
        }

        input {
            style: "background-color: #7A4D6B; color: white; font-size: 60px; border-radius: 30px; border: none; width: 200px; height: 70px; text-align: center;",
            r#type: "number",
            value: "{minutes}",
            step: "15",
            min: "0",
            max: "45",
            oninput: move |e| {
                if let Ok(val) = e.value().parse::<i32>() {
                    // Ensure value is between 0 and 45 and is a multiple of 15
                    let clamped_val = val.clamp(0, 45 / 15) * 15;
                    minutes.set(clamped_val);
                }
            }
        }

        button {
            style: "background-color: #7A4D6B; color: white; font-size: 80px; border-radius: 30px; border: none; width:80px; height: 70px; text-align: center; display: flex; align-items: center; justify-content: center;",
            onclick: move |_| {
                let current_value = minutes();
                if current_value + 15 <= 45 {
                    minutes.set(current_value + 15); // Increases by 15 with max 45
                }
            },
            "+"
        }
    }

    div {
                                    style: "display: flex; gap: 20px; background-color: #E0A0B0;", // Buttons
                                    button {
                                        style: "flex: 1; max-width: 300px; border-radius: 100px; font-size: 28px; background-color: #7A4D6B; color: white; border: 5px solid #E0A0B0; padding: 10px; text-align: center;",
                                        onclick: move |_| {
                                            view.set(View::MainView);
                                        },
                                        "Zurück"
                                    }
                                    button {
                                        style: "flex: 1; max-width: 300px; border-radius: 100px; font-size: 28px; background-color: #7A4D6B; color: white; border: 5px solid #E0A0B0; padding: 10px; text-align: center;",
                                        onclick: move |_| {
                                            let n = number.read().clone();
                                            let stunden = number.read().clone();
                                            let mut stunden_as_float: f32 = stunden as f32;
                                            let mut minuten: f32 = 0.0;
                                            let minutes = minutes.read().clone();
                                           match minutes {
        15 => {
            if stunden_as_float > 0.0 {
                stunden_as_float += 0.25;
            } else {
                stunden_as_float -= 0.25;
            }
        }
        30 => {
            if stunden_as_float > 0.0 {
                stunden_as_float += 0.5;
            } else {
                stunden_as_float -= 0.5;
            }
        }
        45 => {
            if stunden_as_float > 0.0 {
                stunden_as_float += 0.75;
            } else {
                stunden_as_float -= 0.75;
            }
        }
        _ => {},
    }

                                            let entry = database::Entry {
                                                entry_id: 0,
                                                date: formatted_date.read().clone(),
                                                text: text.read().clone(),
                                                hours: stunden_as_float,
                                            };
                                            let state = use_context::<database::State>();
                                            state.save_entry(entry);
                                            view.set(View::MainView);
                                        },
                                        "Speichern"
                                    }
                                }



                                div {
                                    style: "width: 100%; display: flex; flex-direction: column; align-items: center; gap: 10px;",

                                    textarea {
                                        value: "{text}",
                                        rows: "6",
                                        placeholder: "Beschreibung eingeben...",
                                        style: "width: 80%; max-width: 400px; padding: 10px; text-align: center; border-radius: 50px; font-size: 20px; background-color: #7A4D6B; color: white; border: 5px solid #E0A0B0;",
                                        oninput: move |event| {
                                            text.set(event.value());
                                        }



                                    }
                                }

                                div {
                        style: "width: 100%; display: flex; flex-direction: column; align-items: center; gap: 10px;",

                        // Plus/Minus Selector Positioned Above


                        // Container for Sliders and Labels
                        // div {
                        //     style: "display: flex; flex-direction: column; gap: 10px; width: 100%; max-width: 400px;",

                        //     // Stunden Slider
                        //     div {
                        //         style: "display: flex; align-items: center; gap: 10px; width: 100%;",

                        //         label {
                        //             style: "font-weight: bold; font-size: 20px; color: white; min-width: 80px;",
                        //             "Stunden"
                        //         }
                        //         input {
                        //             style: "
                        //     width: 100%;
                        //     padding: 10px;
                        //     text-align: center;
                        //     border-radius: 50px;
                        //     font-size: 80px;
                        //     background-color: #7A4D6B;
                        //     color: white;
                        //     border: 5px solid #E0A0B0;
                        //     appearance: none;
                        //     -webkit-appearance: none;
                        //     -moz-appearance: none;
                        //     box-sizing: border-box;
                        // ",
                        //             r#type: "range",
                        //             min: "0",
                        //             max: if *plusorminus.read() == 1 { "24" } else { "100" },
                        //             value: "{number}",
                        //             oninput: {
                        //                 let mut number = number.clone();
                        //                 move |e: Event<FormData>| {
                        //                     let value: i32 = e.value().parse().unwrap_or(0);
                        //                     number.set(value);
                        //                 }
                        //             }
                        //         }
                        //     }

                        //     // Minuten Slider
                        //     div {
                        //         style: "display: flex; align-items: center; gap: 10px; width: 100%;",

                        //         label {
                        //             style: "font-weight: bold; font-size: 20px; color: white; min-width: 80px;",
                        //             "Minuten"
                        //         }
                        //         input {
                        //             style: "
                        //     width: 100%;
                        //     padding: 10px;
                        //     text-align: center;
                        //     border-radius: 50px;
                        //     font-size: 80px;
                        //     background-color: #7A4D6B;
                        //     color: white;
                        //     border: 5px solid #E0A0B0;
                        //     appearance: none;
                        //     -webkit-appearance: none;
                        //     -moz-appearance: none;
                        //     box-sizing: border-box;
                        // ",
                        //             r#type: "range",
                        //             min: "0",
                        //             max: "45",
                        //             step: "15",
                        //             value: "{minutes}",
                        //             oninput: {
                        //                 let mut minutes = minutes.clone();
                        //                 move |e: Event<FormData>| {
                        //                     if let Ok(value) = e.value().parse::<i32>() {
                        //                         minutes.set(value);
                        //                     }
                        //                 }
                        //             }
                        //         }

                        //     }
                        //     div {
                        //         style: "display: flex; justify-content: center; align-items: center; gap: 10px; width: 100%;",
                // // select {
                // // style: "width: 80px; padding: 10px; text-align: center;
                // // color: white; border-radius: 50px; font-size: 35px;
                // // background-color: #7A4D6B; border: 5px solid #E0A0B0;",
                // // value: "+",
                // // onchange: move |event| {
                // //     number.set(0);
                // //     minutes.set(0);
                // //     plusorminus.set(event.value().parse().unwrap_or(1));
                // // },
                // // option { value: "1", "+" }
                // // option { value: "0", "-" }
                // // }
                        // }
                    // }
                                }



                            }
                        }
}
