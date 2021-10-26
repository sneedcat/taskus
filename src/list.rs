use ansi_term::Color::*;
use sqlite::State;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    utils::{database_path, handle_table, print_tasks},
    List, Listing, CONNECTION_FAILED,
};

pub fn list(list: List) {
    let db_path = database_path();
    match list.listing {
        Listing::Tables => {
            let connection = sqlite::open(&db_path).expect(CONNECTION_FAILED);
            let s = if list.sort {
                "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name ASC;"
            } else {
                "SELECT name FROM sqlite_master WHERE type='table';"
            };
            let mut statement = connection.prepare(s).unwrap();
            if list.disable_color {
                println!("Tables:");
                while let State::Row = statement.next().unwrap() {
                    let name = statement.read::<String>(0).unwrap();
                    println!("- {}", name);
                }
            } else {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_micros();
                let mut color = now;
                println!("{}", Blue.bold().paint("Tables:"));
                while let State::Row = statement.next().unwrap() {
                    let name = statement.read::<String>(0).unwrap();
                    println!(
                        "{} {}",
                        Fixed(color as u8).paint("-"),
                        Fixed(color as u8 % 255).paint(name)
                    );
                    color += 60;
                }
            }
        }
        Listing::Tasks { table } => {
            let table = handle_table(table);
            let connection = sqlite::open(&db_path).expect(CONNECTION_FAILED);
            let s = if list.sort {
                format!("SELECT * FROM {} ORDER BY completed DESC, id ASC", table)
            } else {
                format!("SELECT * FROM {}", table)
            };
            let statement = connection.prepare(s).unwrap();
            let (text, count) = print_tasks(statement, !list.disable_color);
            if count == 0 {
                println!("{}", Red.italic().paint("The table is empty."));
            } else {
                println!("{}", text);
            }
        }
    }
}
