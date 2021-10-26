use ansi_term::Color::*;
use sqlite::{State, Statement};
use std::io::Write;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;
use tabwriter::TabWriter;

use crate::BIN_NAME;

pub fn database_name() -> String {
    BIN_NAME.to_string() + ".db"
}

pub fn database_path() -> PathBuf {
    dirs::data_dir()
        .unwrap()
        .join("taskus")
        .join(database_name())
}

pub fn handle_table(s: Option<String>) -> String {
    match s {
        Some(s) => s,
        None => "general".to_string(),
    }
}

pub fn print_tasks(mut statement: Statement, color: bool) -> (String, usize) {
    let mut count = 0;
    let mut tw = TabWriter::new(Vec::new());
    if color {
        writeln!(
            &mut tw,
            "{}\t{}\t{}\t{}",
            Green.paint("id"),
            Blue.paint("task"),
            Purple.paint("status"),
            Cyan.paint("time")
        )
        .ok();
    } else {
        writeln!(&mut tw, "id\ttask\tstatus\ttime").ok();
    }
    while let State::Row = statement.next().unwrap() {
        let id = statement.read::<i64>(0).unwrap();
        let task = statement.read::<String>(1).unwrap();
        let time = statement.read::<i64>(2).unwrap();
        if time == 0 {
            if color {
                writeln!(
                    &mut tw,
                    "{}\t{}\t{}\t{}",
                    Green.paint(id.to_string()),
                    Blue.paint(task),
                    Purple.paint("uncompleted"),
                    Cyan.paint("-")
                )
                .ok();
            } else {
                writeln!(&mut tw, "{}\t{}\tuncompleted\tnull", id, task).ok();
            }
        } else {
            let dt = std::time::Duration::from_millis(time as u64);
            let now = std::time::SystemTime::now();
            let duration = now - dt;
            let time = format_time(duration.duration_since(UNIX_EPOCH).unwrap().as_secs());
            if color {
                writeln!(
                    &mut tw,
                    "{}\t{}\t{}\t{}",
                    Green.paint(id.to_string()),
                    Blue.paint(task),
                    Purple.paint("completed"),
                    Cyan.paint(time.to_string())
                )
                .ok();
            } else {
                writeln!(&mut tw, "{}\t{}\tcompleted\t{}", id, task, time).ok();
            }
        };
        count += 1;
    }
    (String::from_utf8(tw.into_inner().unwrap()).unwrap(), count)
}

pub fn format_time(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds - days * 86400) / 3600;
    let minutes = (seconds - days * 86400 - hours * 3600) / 60;
    let seconds = seconds - days * 86400 - hours * 3600 - minutes * 60;
    let d = if days == 1 { "day" } else { "days" };
    let h = if hours == 1 { "hour" } else { "hours" };
    let m = if minutes == 1 { "minute" } else { "minutes" };
    let s = if seconds == 1 { "second" } else { "seconds" };
    format!(
        "{} {} {} {} {} {} {} {} ago",
        days, d, hours, h, minutes, m, seconds, s
    )
}
