use ansi_term::Color::*;
use std::{
    process::exit,
    time::{SystemTime, UNIX_EPOCH},
};

use sqlite::Connection;

use crate::{
    utils::{database_path, handle_table, print_tasks},
    Complete, CONNECTION_FAILED,
};

pub fn complete(complete: Complete) {
    let db_path = database_path();
    let table = handle_table(complete.table);
    let mut connection = sqlite::open(db_path).expect(CONNECTION_FAILED);
    match complete.input.parse::<usize>() {
        Ok(id) => complete_id(&mut connection, &table, id),
        Err(_) => complete_task(&mut connection, &table, &complete.input, complete.all),
    }
}

fn complete_id(connection: &mut Connection, table: &str, id: usize) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    connection
        .execute(format!(
            "UPDATE {} SET completed = {} WHERE id = {}",
            table, now, id
        ))
        .unwrap();
}

fn complete_task(connection: &mut Connection, table: &str, arg: &str, all: bool) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let statement = connection
        .prepare(format!(
            "SELECT * FROM {} WHERE task LIKE '{}%'",
            table, arg
        ))
        .unwrap();
    let (text, count) = print_tasks(statement, false);
    if count == 0 {
        println!(
            "{}",
            Purple
                .bold()
                .paint(format!("There are no tasks matching {} pattern.", arg))
        )
    } else if count == 1 || all {
        connection
            .execute(format!(
                "UPDATE {} SET completed = {} WHERE task LIKE '{}%'",
                table, now, arg
            ))
            .unwrap();
        exit(0);
    } else {
        println!(
            "{}",
            White.bold().paint(format!(
                "There are multiple tasks that match `{}` pattern.",
                arg
            ))
        );
        println!("{}", text);
    }
}
