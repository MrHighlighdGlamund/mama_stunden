mod views {
    pub mod choose_month;
    pub mod create_entry;
    pub mod main_view;
    pub mod profile_selection;
}
pub mod database;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use database::State;
use dioxus::prelude::*;
use rusqlite::{params, Connection};
use views::{choose_month, create_entry, main_view, profile_selection};

#[derive(Clone, PartialEq)]
pub enum View {
    MainView,
    ProfileSelection,
    ChooseMonth,
    CreateEntry,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const DATABASE: Asset = asset!("/assets/database.db");
fn create_empty_database() {
    // Path for the SQLite database in Android's internal storage
    // Create folder path

    std::fs::create_dir_all("/data/data/com.example.DxTest/databases").unwrap();

    let db_path = "/data/data/com.example.DxTest/databases/database.db";

    // Create or open the SQLite database
    let conn = Connection::open(db_path).unwrap();

    // Optionally create a table
    conn.execute_batch(
        "
CREATE TABLE persons (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE  
);

CREATE TABLE months (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    person_id INTEGER NOT NULL,
    month TEXT NOT NULL,
    year INTEGER NOT NULL,
    UNIQUE(person_id, month, year),  
    FOREIGN KEY (person_id) REFERENCES persons(id) ON DELETE CASCADE
);

CREATE TABLE entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    month_id INTEGER NOT NULL,
    date TEXT NOT NULL,  
    text TEXT NOT NULL,
    hours REAL NOT NULL,
    FOREIGN KEY (month_id) REFERENCES months(id) ON DELETE CASCADE
);
",
    )
    .unwrap();

    println!("Database created and table initialized.");
}
fn main() {
    dioxus::launch(App);
}
// #[component]
// fn App() -> Element {

//     create_empty_database();

//     let view = use_signal(|| View::MainView);
//     let mut state = State::new(DATABASE);

//     state.get_last_month_with_entries();
//     use_context_provider(|| state);

//     rsx! {
//         document::Link { rel: "icon", href: FAVICON }
//         document::Link { rel: "stylesheet", href: MAIN_CSS }
//         match *view.read() {
//             View::ProfileSelection => rsx! { profile_selection::profile_selection { view} },
//             View::ChooseMonth => rsx! { choose_month::choose_month { view } },
//             View::CreateEntry => rsx! { create_entry::create_entry { view} },
//             View::MainView => rsx! { main_view::main_view { view} },
//         }
//     }
// }
#[component]
fn App() -> Element {
    // create_empty_database();
    insert();

    let view = use_signal(|| View::MainView);
    let mut state = State::new(DATABASE);

    state.get_last_month_with_entries();
    use_context_provider(|| state);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        match *view.read() {
            View::ProfileSelection => rsx! { profile_selection::profile_selection { view} },
            View::ChooseMonth => rsx! { choose_month::choose_month { view } },
            View::CreateEntry => rsx! { create_entry::create_entry { view} },
            View::MainView => rsx! { main_view::main_view { view} },
        }
    }
}
fn insert() {
    // delete old database
    let db_path = "/data/data/com.example.DxTest/databases/database.db";
    // check if file exists
    if std::path::Path::new(db_path).exists() {
        return;
        std::fs::remove_file(db_path).unwrap();
    }


std::fs::create_dir_all("/data/data/com.example.DxTest/databases").unwrap();


    // Create or open the SQLite database
    let conn = Connection::open(db_path).unwrap();

    // Optionally create a table
    conn.execute_batch(
        "
CREATE TABLE persons (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE  
);

CREATE TABLE months (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    person_id INTEGER NOT NULL,
    month TEXT NOT NULL,
    year INTEGER NOT NULL,
    UNIQUE(person_id, month, year),  
    FOREIGN KEY (person_id) REFERENCES persons(id) ON DELETE CASCADE
);

CREATE TABLE entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    month_id INTEGER NOT NULL,
    date TEXT NOT NULL,  
    text TEXT NOT NULL,
    hours REAL NOT NULL,
    FOREIGN KEY (month_id) REFERENCES months(id) ON DELETE CASCADE
);
",
    )
    .unwrap();
    // Insert persons
    let persons = ["Gaby", "Hermann"];
    let mut person_ids = Vec::new();

    for name in persons.iter() {
        conn.execute("INSERT INTO persons (name) VALUES (?1)", params![name])
            .unwrap();
        person_ids.push(conn.last_insert_rowid());
    }
    let mut months: Vec<String> = Vec::new();
    months.push(String::from("Januar"));
    months.push(String::from("Februar"));
    months.push(String::from("März"));
    months.push(String::from("April"));
    months.push(String::from("Mai"));
    months.push(String::from("Juni"));
    months.push(String::from("Juli"));
    months.push(String::from("August"));
    months.push(String::from("September"));
    months.push(String::from("Oktober"));
    months.push(String::from("November"));
    months.push(String::from("Dezember"));
   
    for person_id in person_ids.iter() {
        for i in 2025..2050 {
            for month in months.iter() {
                conn.execute(
                    "INSERT INTO months (person_id, month, year) VALUES (?1, ?2, ?3)",
                    params![person_id, month, i],
                )
                .unwrap();
            }
        }
    }

    // Insert months for each person
    // let months = [("March", 2025), ("April", 2025)];
    // let mut month_ids = Vec::new();

    // for (i, &(month, year)) in months.iter().enumerate() {
    //     conn.execute(
    //         "INSERT INTO months (person_id, month, year) VALUES (?1, ?2, ?3)",
    //         params![person_ids[i], month, year],
    //     ).unwrap();
    //     month_ids.push(conn.last_insert_rowid());
    // }

    // // Insert entries for each month
    // let entries = [
    //     ("2025-03-15", "Worked on project A", 4.0),
    //     ("2025-04-10", "Completed task B", 6.5),
    // ];

    // for (i, &(date, text, hours)) in entries.iter().enumerate() {
    //     conn.execute(
    //         "INSERT INTO entries (month_id, date, text, hours) VALUES (?1, ?2, ?3, ?4)",
    //         params![month_ids[i], date, text, hours],
    //     ).unwrap();
    // }

    println!("Data inserted successfully!");
}
