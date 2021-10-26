use std::{io::Read, process::exit};

use crate::{
    utils::{database_path, handle_table},
    Init,
};
use ansi_term::Color::*;

pub fn init(init: Init) {
    let db_path = database_path();
    let table = handle_table(init.table);
    if db_path.exists() {
        let connection = sqlite::open(&db_path).expect("Connection to database failed.");
        let mut statement = connection
            .prepare(format!(
                "SELECT name FROM sqlite_master WHERE type='table' AND name='{}'",
                table
            ))
            .unwrap();
        let exists = statement.next().is_ok() && statement.read::<String>(0).is_ok();
        if exists && !init.over {
            println!(
                "{}",
                Red.bold()
                    .paint(format!("Table `{}` already exists.", table))
            );
            println!("{}", Red.paint("Do you want to override it?"));
            println!("{}", Red.paint("(y)es or (n)o"));
            let mut input = vec![b'n'; 1];
            std::io::stdin()
                .read_exact(&mut input)
                .expect("Could not read from stdin");
            if input[0] == b'n' || input[0] == b'N' {
                exit(1);
            }
        }
        if exists {
            drop(statement);
            connection.execute(format!("DROP TABLE {}", table)).unwrap();
        }
    } else if std::fs::create_dir_all(&db_path.parent().unwrap()).is_err() {
        println!(
            "{}",
            Red.bold()
                .paint("You don't have permissions to create the path")
        );
        exit(1);
    };
    let conn = sqlite::open(&db_path).expect("Connection to database failed.");
    if conn
        .execute(format!(
            "CREATE TABLE {} (id INTEGER PRIMARY KEY ASC, task TEXT, completed INTEGER);",
            table
        ))
        .is_err()
    {
        if (&db_path).exists() {
            std::fs::remove_file(&db_path).ok();
        }
        println!(
            "{}",
            Red.bold()
                .paint("Couldn't create new table.\nPlease try again.")
        );
        exit(1);
    }
    println!("{}", Green.paint("The database was initialized."));
}
