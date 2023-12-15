// rustimport:pyo3

//:
//: [dependencies]
//: rusqlite = "0.19.0"

use pyo3::prelude::*;
use rusqlite::{Connection, NO_PARAMS};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}

#[pyfunction]
fn insert_person(name: String, age: i32) -> PyResult<()> {
    let conn = Connection::open("my_db.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age  INTEGER NOT NULL
        )",
        NO_PARAMS,
    )
    .unwrap();

    conn.execute(
        "INSERT INTO person (name, age) VALUES (?1, ?2)",
        &[&name, &age.to_string()],
    )
    .unwrap();

    Ok(())
}

#[pyfunction]
fn get_person(id: i32) -> PyResult<String> {
    let conn = Connection::open("my_db.db").unwrap();

    let mut stmt = conn
        .prepare("SELECT id, name, age FROM person WHERE id = ?1")
        .unwrap();
    let mut person_iter = stmt
        .query_map(&[&id.to_string()], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                age: row.get(2)?,
            })
        })
        .unwrap();

    let out = format!("Found person {:?}", person_iter.next().unwrap().unwrap());
    Ok(out)
}
