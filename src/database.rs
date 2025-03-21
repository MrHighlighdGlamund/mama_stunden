use std::sync::Arc;

use dioxus::{prelude::Asset, signals::{Readable, Signal, Writable}};
use rusqlite::{params, types::Type, Connection, Result};
// #[derive(Debug, Clone)]
#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub entry_id: i32,
    pub date: String,
    pub text: String,
    pub hours: f32,
}

#[derive(Debug, Clone)]
pub struct State {
    db_connection: Arc<Connection>,
    pub active_profile: Signal<String>,
    pub active_profile_id: Signal<i32>,
    pub active_month: Signal<i32>,
    pub active_month_text: Signal<String>,
    pub active_year: Signal<i32>,
}

impl State {
    pub fn new(database: Asset) -> State {
        State {
            db_connection: Arc::new(Connection::open("/data/data/com.example.DxTest/databases/database.db").unwrap()),
            active_profile: Signal::new("Gaby".to_string()),
            active_profile_id: Signal::new(1),
            active_month: Signal::new(2),
            active_month_text: Signal::new("".to_string()),
            active_year: Signal::new(2025),
        }
    }
    pub fn set_profile(&mut self, profile: String) {
        match profile.as_str() {
            "Gaby" => self.active_profile_id.with_mut(|p| *p = 1),
            "Hermann" => self.active_profile_id.with_mut(|p| *p = 2),
            _ => self.active_profile_id.with_mut(|p| *p = 0),
        }
        self.active_profile.with_mut(|p| *p = profile);
    }
    pub fn get_all_profiles(&self) -> Vec<String> {
        let mut stmt = self
            .db_connection
            .prepare("SELECT name FROM persons")
            .unwrap();
        let mut rows = stmt.query([]).unwrap();
        let mut profiles: Vec<String> = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            let name: String = row.get(0).unwrap();
            profiles.push(name);
        }
        profiles
    }
    pub fn set_month(&mut self, month: String, year: i32) {
        let mut stmt = self
            .db_connection
            .prepare("SELECT id FROM months WHERE person_id = ?1 AND month = ?2 AND year = ?3")
            .unwrap();
        let active_profile_id: i32 = self.active_profile_id.read().clone();
        let month_id: i32 = stmt.query_row(params![active_profile_id, month, year], |row| {
            Ok(row.get(0).unwrap())
        }).unwrap();
        self.active_month.with_mut(|m| *m = month_id);
        self.active_month_text.with_mut(|m| *m = month);
        self.active_year.with_mut(|y| *y = year);
    }
    pub fn get_last_month_with_entries(&mut self) {
        // get the last month id which has entries
        let mut stmt = self
        .db_connection
        .prepare(
            "SELECT m.id, m.month, m.year
            FROM months m
            JOIN entries e ON m.id = e.month_id
            WHERE m.person_id = ?
            GROUP BY m.id
            ORDER BY m.year DESC, 
            CASE m.month
                WHEN 'Januar' THEN 1
                WHEN 'Februar' THEN 2
                WHEN 'März' THEN 3
                WHEN 'April' THEN 4
                WHEN 'Mai' THEN 5
                WHEN 'Juni' THEN 6
                WHEN 'Juli' THEN 7
                WHEN 'August' THEN 8
                WHEN 'September' THEN 9
                WHEN 'Oktober' THEN 10
                WHEN 'November' THEN 11
                WHEN 'Dezember' THEN 12
                ELSE 13  -- For any invalid month name
            END DESC
            LIMIT 1;",
        )
        .unwrap();
        let active_profile_id = self.active_profile_id.read().clone();
        let mut rows = stmt.query(params![active_profile_id]).unwrap();
        match rows.next() {
            Ok(row) => match row {
                Some(row) => {
                    let month_id: i32 = row.get(0).unwrap();
                    let month: String = row.get(1).unwrap();
                    let year: i32 = row.get(2).unwrap();
                    self.active_month.with_mut(|m| *m = month_id);
                    self.active_month_text.with_mut(|m| *m = month);
                    self.active_year.with_mut(|m| *m = year);
                    return;
                }
                None => {
                    println!("No rows found");
                }
            },
            Err(_) => {
                println!("No rows found");
            }
        }
        println!("Person ID: {}", active_profile_id);
        let mut stmt = self
        .db_connection
        .prepare(
            "SELECT id, month, year 
            FROM months 
            WHERE person_id = ? 
            ORDER BY year ASC, 
            CASE month
                WHEN 'Januar' THEN 1
                WHEN 'Februar' THEN 2
                WHEN 'März' THEN 3
                WHEN 'April' THEN 4
                WHEN 'Mai' THEN 5
                WHEN 'Juni' THEN 6
                WHEN 'Juli' THEN 7
                WHEN 'August' THEN 8
                WHEN 'September' THEN 9
                WHEN 'Oktober' THEN 10
                WHEN 'November' THEN 11
                WHEN 'Dezember' THEN 12
                ELSE 13
            END ASC
            LIMIT 1;",
        )
        .unwrap();        let mut rows = stmt.query(params![active_profile_id]).unwrap();

        if let Some(row) = rows.next().unwrap() {
            let month_id: i32 = row.get(0).unwrap();
            let month: String = row.get(1).unwrap();
            let year: i32 = row.get(2).unwrap();
            self.active_month.with_mut(|m| *m = month_id);
            self.active_month_text.with_mut(|m| *m = month);
            self.active_year.with_mut(|m| *m = year);
        }
    }
    pub fn save_entry(&self, entry: Entry) {
        let active_month: i32 = *self.active_month.read();
        let mut stmt = self
            .db_connection
            .prepare("INSERT INTO entries (month_id, date, text, hours) VALUES (?1, ?2, ?3, ?4)")
            .unwrap();
        stmt.execute(params![active_month, entry.date, entry.text, entry.hours])
            .unwrap();
    }
    pub fn delete_entry(&self, entry_id: i32) {
        let mut stmt = self
            .db_connection
            .prepare("DELETE FROM entries WHERE id = ?1")
            .unwrap();
        stmt.execute(params![entry_id]).unwrap();
    }
    pub fn get_entries(&self) -> Vec<Entry> {
        let active_month: i32 = *self.active_month.read();
        let mut stmt = self
            .db_connection
            .prepare("SELECT * FROM entries WHERE month_id = ?1 ORDER BY date ASC")
            .unwrap();
        let mut rows = stmt.query(params![active_month]).unwrap();
        let mut entries: Vec<Entry> = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            let entry_id: i32 = row.get(0).unwrap();
            let date: String = row.get(2).unwrap();
            let text: String = row.get(3).unwrap();
            let hours: f32 = row.get(4).unwrap();
            entries.push(Entry {
                entry_id,
                date,
                text,
                hours,
            });
        }
        entries
    }
}
