use clap::{App, Arg};
pub fn build_cli() -> App<'static> {
    App::new("taskus")
        .about("A CLI for fast and simple TODOs")
        .author("Megumin <meguminloli@protonmail.com>")
        .version("0.1.0")
        .subcommand(
            App::new("init")
                .about("Initialize a new table or override an existing one")
                .arg(
                    Arg::new("override")
                        .long("override")
                        .short('o')
                        .about("Override current table"),
                )
                .arg(Arg::new("table").takes_value(true).about("Chosen table")),
        )
        .subcommand(
            App::new("add")
                .about("Add a new task to a table")
                .arg(
                    Arg::new("table")
                        .long("table")
                        .short('t')
                        .takes_value(true)
                        .about("Chosen table"),
                )
                .arg(Arg::new("INPUT").about("The task").required(true)),
        )
        .subcommand(
            App::new("delete")
                .about("Delete a task from a table")
                .arg(
                    Arg::new("all")
                        .long("all")
                        .short('a')
                        .about("Delete all tasks matching a pattern"),
                )
                .subcommand(
                    App::new("tasks")
                        .about("Delete tasks")
                        .arg(
                            Arg::new("table")
                                .long("table")
                                .short('t')
                                .takes_value(true)
                                .about("Chosen table"),
                        )
                        .arg(
                            Arg::new("INPUT")
                                .about("The task")
                                .required(true)
                                .about("Can be a string or a number matching the id"),
                        ),
                )
                .subcommand(
                    App::new("tables").about("Delete tables").arg(
                        Arg::new("INPUT")
                            .about("The task")
                            .required(true)
                            .about("Can be a string or a number matching the id"),
                    ),
                ),
        )
        .subcommand(
            App::new("list")
                .about("List all tasks or tables")
                .arg(
                    Arg::new("disable-color")
                        .long("disable-color")
                        .short('d')
                        .about("Disable colors from output"),
                )
                .arg(
                    Arg::new("sort")
                        .long("sort")
                        .short('s')
                        .about("Sort according to completion"),
                )
                .subcommand(App::new("tables").about("Lists all tables from the database"))
                .subcommand(
                    App::new("tasks").about("Lists all tasks from a table").arg(
                        Arg::new("table")
                            .long("table")
                            .short('t')
                            .takes_value(true)
                            .about("Chosen table"),
                    ),
                ),
        )
        .subcommand(
            App::new("complete")
                .about("Mark a task as complete")
                .arg(
                    Arg::new("table")
                        .long("table")
                        .short('t')
                        .takes_value(true)
                        .about("Chosen table"),
                )
                .arg(
                    Arg::new("all")
                        .short('a')
                        .about("Complete all tasks matching a pattern"),
                )
                .arg(
                    Arg::new("INPUT")
                        .required(true)
                        .about("Can be a string or a number matching the id"),
                ),
        )
}
