use crate::utils::{database_path, handle_table};
use crate::{Add, CONNECTION_FAILED};
use ansi_term::Color::*;

pub fn add(add: Add) {
    let db_path = database_path();
    let connection = sqlite::open(db_path).expect(CONNECTION_FAILED);
    let table = handle_table(add.table);
    connection
        .execute(format!(
            "INSERT INTO {} VALUES(null, \"{}\", null);",
            table, add.input
        ))
        .expect("Couldn't insert task");
    println!(
        "{}",
        Blue.blink().paint(format!(
            "Task `{}` was added to table {}.",
            add.input, table
        ))
    );
}
