use rusqlite::{params, Connection, Result};
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub months: Vec<Month>,
}
impl Default for Person {
    fn default() -> Self {
        Person {
            id: 0,
            name: "".to_string(),
            months: Vec::new(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Month {
    pub id: i32,
    pub month: String,
    pub year: i32,
    pub entries: Vec<Entry>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub month_id: i32,
    pub date: String,
    pub text: String,
    pub hours: f32,
}
pub fn get_months(name: String) -> Result<Vec<Month>> {
    let conn = Connection::open("database.db").unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM months WHERE person_id = (SELECT id FROM persons WHERE name = ?1)")
        .unwrap();
    let month_iter = stmt
        .query_map(params![name], |row| {
            // get entries for month
            let entries = get_entries(row.get(1)?);
            if entries.is_ok() {
                Ok(Month {
                    id: row.get(0)?,
                    month: row.get(2)?,
                    year: row.get(3)?,
                    entries: entries.unwrap(),
                })
            } else {
                Ok(Month {
                    id: row.get(0)?,
                    month: row.get(2)?,
                    year: row.get(3)?,
                    entries: Vec::new(),
                })
            }
        })
        .unwrap();
    let mut months: Vec<Month> = Vec::new();
    for month in month_iter {
        println!("{:?}", month);
        months.push(month.unwrap());
    }
    Ok(months) // return months
}
pub fn get_entries(month_id: i32) -> Result<Vec<Entry>> {
    let conn = Connection::open("database.db")?;
    let mut stmt = conn.prepare(
        "SELECT * FROM entries WHERE month_id = ?1 ORDER BY date ASC",
    )?;

    let entry_iter = stmt.query_map(params![month_id], |row| {
        println!("{:?}", row);
        Ok(Entry {
            month_id: row.get(1)?,
            date: row.get(2)?,
            text: row.get(3)?,
            hours: row.get(4)?,
        })
    })?;

    let mut entries = Vec::new();
    for entry in entry_iter {
        println!("{:?}", entry);
        entries.push(entry?);
    }

    Ok(entries)
}

pub fn save_entry(month_id: i32, entry: Entry) {
    println!("Month ID: {}", month_id);
    let conn = Connection::open("database.db").expect("Failed to open database");

    conn.execute(
        "INSERT INTO entries (month_id, date, text, hours) VALUES (?1, ?2, ?3, ?4)",
        params![month_id, entry.date, entry.text, entry.hours],
    )
    .expect("Failed to insert entry");

}

pub fn create_month(person_id: i32, month: Month) {
    let conn = Connection::open("database.db").unwrap();
    conn.execute(
        "INSERT INTO months (person_id, month, year) VALUES (?1, ?2, ?3)",
        params![person_id, month.month, month.year],
    )
    .unwrap();
}
pub fn get_all_profiles() -> Vec<Person> {
    let conn = Connection::open("database.db").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM persons").unwrap();
    let person_iter = stmt
        .query_map(params![], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                months: Vec::new(),
            })
        })
        .unwrap();
    let mut people: Vec<Person> = Vec::new();
    for person in person_iter {
        people.push(person.unwrap());
    }
    people
}
// pub fn save_data() {
//     let conn = Connection::open("database.db").unwrap();
//     let test_people = Person {
//         id: 2,
//         name: "Hermann".to_string(),
//         months: vec![
//             Month {

//                 month: "January".to_string(),
//                 year: 2025,
//                 entries: vec![],
//             },
//             Month {
//                 month: "February".to_string(),
//                 year: 2025,
//                 entries: vec![],
//             },
//         ],
//     };
//     conn.execute(
//         "INSERT INTO persons (name) VALUES (?1)",
//         &[&test_people.name],
//     )
//     .unwrap();
//     let person_id = conn.last_insert_rowid() as i32;

//     for month in test_people.months {
//         conn.execute(
//             "INSERT INTO months (month, year, person_id) VALUES (?1, ?2, ?3)",
//             params![month.month, month.year, person_id],
//         )
//         .unwrap();
//         let month_id = conn.last_insert_rowid() as i32;
//         for entry in month.entries {
//             conn.execute(
//                 "INSERT INTO entry (date, text, hours, month_id) VALUES (?1, ?2, ?3, ?4)",
//                 params![entry.date, entry.text, entry.hours, month_id],
//             )
//             .unwrap();
//         }
//     }
// }

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
