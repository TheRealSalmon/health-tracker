mod db;

use db::{db_init, get_con};
use tauri::{command, generate_context, generate_handler};

#[command]
fn println(s: &str) {
    println!("{s}");
}

#[command]
fn get_data(datatype: &str) -> String {
    let data_type = datatype;

    let mut array = json::array![];
    let con = get_con();
    let mut statement = con
        .prepare(
            "SELECT
                *
            FROM
                data
            WHERE
                data_type = ?1
            ORDER BY
                timestamp DESC",
        )
        .unwrap();
    let tuples = statement
        .query_map(rusqlite::params![data_type], |row| {
            Ok((
                row.get::<_, String>(0).unwrap(),
                row.get::<_, String>(1).unwrap(),
                row.get::<_, f64>(2).unwrap(),
                row.get::<_, String>(3).unwrap(),
            ))
        })
        .unwrap();
    for tuple in tuples {
        let tuple = tuple.unwrap();

        array
            .push(json::object! {
                data_type: tuple.0,
                timestamp: tuple.1,
                value: tuple.2,
                unit: tuple.3,
            })
            .unwrap();
    }

    json::stringify(array)
}

#[command]
fn submit_data(datatype: &str, timestamp: &str, value: f64, unit: &str) {
    let data_type = datatype;

    // TODO: remove, for developing
    println!("{}, {}, {}, {}", data_type, timestamp, value, unit,);

    let con = get_con();
    con.execute(
        "INSERT INTO data 
        (
            data_type,
            timestamp,
            value,
            unit
        )
        VALUES (?1, ?2, ?3, ?4)",
        (
            data_type.to_string(),
            timestamp.to_string(),
            value,
            unit.to_string(),
        ),
    )
    .unwrap();
}

#[command]
fn delete_data(timestamp: &str) {
    let con = get_con();
    con.execute(
        "DELETE FROM
            data
        WHERE
            timestamp = ?1",
        (timestamp.to_string(),),
    )
    .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    db_init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(generate_handler![
            println,
            get_data,
            submit_data,
            delete_data
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}
