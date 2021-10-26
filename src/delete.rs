use crate::{
    utils::{database_path, handle_table, print_tasks},
    Delete, DeleteEnum, CONNECTION_FAILED,
};
use ansi_term::Color::*;
use sqlite::{Connection, State};

pub fn delete(delete: Delete) {
    let db_path = database_path();
    match delete.action {
        DeleteEnum::Tables => {
            let connection = sqlite::open(db_path).expect(CONNECTION_FAILED);
            let mut statement = connection
                .prepare(format!(
                    "SELECT name FROM sqlite_master WHERE type='table' AND name LIKE '{}%';",
                    delete.input
                ))
                .unwrap();
            let mut v = Vec::new();
            while let State::Row = statement.next().unwrap() {
                let name = statement.read::<String>(0).unwrap();
                v.push(name);
            }
            if v.is_empty() {
                println!(
                    "{}",
                    Red.bold().paint(format!(
                        "There are no tasks that match `{}` pattern.",
                        delete.input
                    ))
                )
            } else if v.len() == 1 || delete.all {
                for t in v {
                    connection.execute(format!("DROP TABLE {}", t)).unwrap();
                }
            } else {
                println!(
                    "{}",
                    Red.bold()
                        .paint("There are multiple tables matching this pattern.")
                );
                for t in v {
                    println!("{}", Red.paint(t));
                }
            }
        }
        DeleteEnum::Tasks { table } => {
            let table = handle_table(table);
            let mut connection = sqlite::open(db_path).expect(CONNECTION_FAILED);
            match delete.input.parse::<usize>() {
                Ok(id) => delete_task_by_id(&mut connection, &table, id),
                Err(_) => delete_task_by_text(&mut connection, &table, &delete.input, delete.all),
            }
        }
    }
}

fn delete_task_by_id(connection: &mut Connection, table: &str, id: usize) {
    connection
        .execute(format!("DELETE FROM {} WHERE id = {}", table, id))
        .unwrap();
}

fn delete_task_by_text(connection: &mut Connection, table: &str, arg: &str, all: bool) {
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
            Blue.bold()
                .paint(format!("There are no tasks that match `{}` pattern.", arg))
        )
    } else if count == 1 || all {
        connection
            .execute(format!("DELETE FROM {} WHERE task LIKE '{}%'", table, arg))
            .unwrap();
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
