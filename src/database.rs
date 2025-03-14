use rusqlite::{params, Connection, Result};
#[derive(Debug)]
pub struct Person {
    id: i32,
    name: String,
    months: Vec<Month>,
}
#[derive(Debug)]
pub struct Month {
    month: String,
    year: i32,
    entries: Vec<Entry>,
}
#[derive(Debug)]
pub struct Entry {
    date: String,
    text: String,
    hours: f32,
}
pub fn get_months(name: String) -> Vec<Month> {
    let conn = Connection::open("database.db").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM months WHERE person_id = (SELECT id FROM persons WHERE name = ?1)").unwrap();
    let month_iter = stmt.query_map(params![name], |row| {
        Ok(Month {
            month: row.get(2)?,
            year: row.get(3)?,
            entries: Vec::new(),
        })
    }).unwrap();
    // get entries for 
    let mut months: Vec<Month> = Vec::new();
    for month in month_iter {
        months.push(month.unwrap());
    }
    months

}
pub fn get_entries(month_id: i32) -> Vec<Entry> {
    let conn = Connection::open("database.db").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM entries WHERE month_id = (SELECT id FROM months WHERE month = ?1 AND year = ?2)").unwrap();
    let entry_iter = stmt.query_map(params![month_id], |row| {
        Ok(Entry {
            date: row.get(2)?,
            text: row.get(3)?,
            hours: row.get(4)?,
        })
    }).unwrap();
    let mut entries: Vec<Entry> = Vec::new();
    for entry in entry_iter {
        entries.push(entry.unwrap());
    }
    entries
}
pub fn save_entry(month_id: i32, entry: Entry) {
    let conn = Connection::open("database.db").unwrap();
    conn.execute(
        "INSERT INTO entries (month_id, date, text, hours) VALUES (?1, ?2, ?3, ?4)",
        params![month_id, entry.date, entry.text, entry.hours],
    ).unwrap();
}
pub fn create_month(person_id: i32, month: Month) {
    let conn = Connection::open("database.db").unwrap();
    conn.execute(
        "INSERT INTO months (person_id, month, year) VALUES (?1, ?2, ?3)",
        params![person_id, month.month, month.year],
    ).unwrap();
}

pub fn save_data() {
    let conn = Connection::open("database.db").unwrap();
    let test_people = Person {
        id: 1,
        name: "Gaby".to_string(),
        months: vec![
            Month {
                month: "January".to_string(),
                year: 2025,
                entries: vec![],
            },
            Month {
                month: "February".to_string(),
                year: 2025,
                entries: vec![],
            },
        ],
    };

    conn.execute(
        "INSERT INTO persons (name) VALUES (?1)",
        &[&test_people.name],
    )
    .unwrap();
    let person_id = conn.last_insert_rowid() as i32;

    for month in test_people.months {
        conn.execute(
            "INSERT INTO months (month, year, person_id) VALUES (?1, ?2, ?3)",
            params![month.month, month.year, person_id],
        )
        .unwrap();

        let month_id = conn.last_insert_rowid() as i32;
        for entry in month.entries {
            conn.execute(
                "INSERT INTO entry (date, text, hours, month_id) VALUES (?1, ?2, ?3, ?4)",
                params![entry.date, entry.text, entry.hours, month_id],
            )
            .unwrap();
        }
    }

    // Insert entries for each month (if any)
}

pub fn create_database() -> Result<()> {
    println!("Connecting to a database...");
    let conn = Connection::open("database.db")?;
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
    )?;

    println!("Database created successfully");
    Ok(())
}
