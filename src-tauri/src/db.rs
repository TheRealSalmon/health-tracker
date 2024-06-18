use rusqlite::Connection;

pub fn db_init() {
    let db_path_string = get_db_path();

    let db_path = std::path::Path::new(&db_path_string);
    let db_dir = db_path.parent().unwrap();

    // TODO: remove, for developing
    std::fs::remove_file(db_path).unwrap();

    if !db_dir.exists() {
        std::fs::create_dir(db_dir).unwrap();
    }
    if !db_path.exists() {
        std::fs::File::create(db_path).unwrap();

        create_tables(&get_con());
    }

    // TODO: remove, for developing
    let con = get_con();
    let data = [
        ("Weight", "2024-06-16T02:51:02.037Z", 131.0, "lbs"),
        ("Weight", "2024-06-17T02:51:02.037Z", 130.0, "lbs"),
        ("Weight", "2024-06-18T02:51:02.037Z", 131.2, "lbs"),
        ("Weight", "2024-06-19T02:51:02.037Z", 130.8, "lbs"),
        ("Weight", "2024-06-20T02:51:02.037Z", 131.4, "lbs"),
        ("Weight", "2024-06-21T02:51:02.037Z", 133.0, "lbs"),
        ("Weight", "2024-06-22T02:51:02.037Z", 134.0, "lbs"),
    ];
    for d in data {
        con.execute(
            "INSERT INTO data 
            (
                data_type,
                timestamp,
                value,
                unit
            )
            VALUES (?1, ?2, ?3, ?4)",
            (d.0.to_string(), d.1.to_string(), d.2, d.3.to_string()),
        )
        .unwrap();
    }
}

pub fn get_con() -> Connection {
    let db_path = get_db_path();
    Connection::open(db_path).unwrap()
}

pub fn create_tables(con: &Connection) {
    con.execute(
        "CREATE TABLE data (
                data_type    TEXT NOT NULL,
                timestamp   TEXT PRIMARY KEY,
                value       FLOAT NOT NULL,
                unit        TEXT NOT NULL
            )",
        (),
    )
    .unwrap();
}

pub fn get_db_path() -> String {
    dirs::data_dir().unwrap().to_str().unwrap().to_string() + "/health-tracker/database.sqlite"
}
