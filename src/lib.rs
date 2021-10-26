use std::process::exit;

pub mod add;
pub mod cli;
pub mod complete;
pub mod delete;
pub mod init;
pub mod list;
pub mod utils;

const BIN_NAME: &str = env!("CARGO_PKG_NAME");
const CONNECTION_FAILED: &str =
    "Couldn't connect to database.\nCheck if it exists, if it doesn't, run `taskus init <table>`.";

pub fn gen_opts() -> Opts {
    let app = crate::cli::build_cli();
    let mut c = app.clone();
    let matches = app.get_matches();
    let subcmd = if let Some(m) = matches.subcommand_matches("add") {
        let table = m.value_of("table").map(|a| a.to_string());
        let input = m.value_of("INPUT").unwrap().to_string();
        SubCommand::Add(Add { table, input })
    } else if let Some(m) = matches.subcommand_matches("list") {
        let sort = m.is_present("sort");
        let disable_color = m.is_present("disable-color");
        let sub = if let Some(_m) = m.subcommand_matches("tables") {
            Listing::Tables
        } else if let Some(m) = m.subcommand_matches("tasks") {
            let table = m.value_of("table").map(|a| a.to_string());
            Listing::Tasks { table }
        } else {
            let mut a = c.find_subcommand("list").unwrap().clone();
            a.print_help().ok();
            exit(1);
        };
        SubCommand::List(List {
            sort,
            disable_color,
            listing: sub,
        })
    } else if let Some(m) = matches.subcommand_matches("delete") {
        let all = m.is_present("all");
        let (input, sub) = if let Some(m) = m.subcommand_matches("tables") {
            (m.value_of("INPUT").unwrap().to_string(), DeleteEnum::Tables)
        } else if let Some(m) = m.subcommand_matches("tasks") {
            let table = m.value_of("table").map(|a| a.to_string());
            (
                m.value_of("INPUT").unwrap().to_string(),
                DeleteEnum::Tasks { table },
            )
        } else {
            let mut a = c.find_subcommand("delete").unwrap().clone();
            a.print_help().ok();
            exit(1);
        };
        SubCommand::Delete(Delete {
            action: sub,
            all,
            input,
        })
    } else if let Some(m) = matches.subcommand_matches("complete") {
        let table = m.value_of("table").map(|a| a.to_string());
        let all = m.is_present("all");
        let input = m.value_of("INPUT").unwrap().to_string();
        SubCommand::Complete(Complete { table, all, input })
    } else if let Some(m) = matches.subcommand_matches("init") {
        let over = m.is_present("override");
        let table = m.value_of("table").map(|a| a.to_string());
        SubCommand::Init(Init { over, table })
    } else {
        c.print_long_help().ok();
        exit(1);
    };
    Opts { subcmd }
}

pub struct Opts {
    pub subcmd: SubCommand,
}

pub enum SubCommand {
    Init(Init),
    Add(Add),
    Delete(Delete),
    List(List),
    Complete(Complete),
}

pub struct Init {
    over: bool,
    table: Option<String>,
}

pub struct List {
    sort: bool,
    disable_color: bool,
    listing: Listing,
}

pub enum Listing {
    Tables,
    Tasks { table: Option<String> },
}

pub enum DeleteEnum {
    Tables,
    Tasks { table: Option<String> },
}

pub struct Add {
    table: Option<String>,
    input: String,
}

pub struct Delete {
    action: DeleteEnum,
    all: bool,
    input: String,
}

pub struct Complete {
    table: Option<String>,
    all: bool,
    input: String,
}
