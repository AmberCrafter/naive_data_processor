use std::path::Path;

use sqlite::Connection;

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn db_get<T: AsRef<Path> + AsRef<std::ffi::OsStr>>(path: T) -> sqlite::Result<Connection> {
    let p = Path::new(&path);
    let conn;


    if p.exists() {
        conn = sqlite::Connection::open(path)?;
    } else {
        conn = sqlite::Connection::open(path)?;
        for dtype in ["integer", "float", "text"] {
            let tablename = format!("{}Table", capitalize(dtype));
            let query = format!("CREATE TABLE IF NOT EXISTS {tablename} (
                id integer primary key autoincrement,
                datetime text,
                parameter text,
                value {dtype},
                flag UNSIGNED BIG INT
            )");

            conn.execute(query)?;
        }
    }
    Ok(conn)
}