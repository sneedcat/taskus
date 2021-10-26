# taskus: a simple, but powerful TODO CLI

taskus is a tool for managing TODOs. It has support for most operating systems.

Shell completions are available under `$releaseDir/build/taskus-*/out/`. By default, it generates configs for `bash`, `zsh`, `pwsh`, `elvish` and `fish`.

It uses `sqlite` for the database. By default, it creates the database at `$XDG_DATA_HOME/taskus/taskus.db`.
The default table is `general`, but it doesn't exist by default, you should initialize it using `taskus init`.
For the other tables, use `taskus init <table>`.
To add a task, use `taskus add [-t <table>] <INPUT>`.
To delete a task, use `taskus delete tasks <ID or task pattern>`.
To delete a table, use `taskus delete table <table pattern>`.
To complete a task, use `taskus complete <ID or task pattern>`.
To list all tasks, use `taskus list tasks`.
To list all tables, use `taskus list tables`.
To check the optional arguments, check `taskus help` and for subcommands `taskus <subcommand> -h`.

### Taskus is licensed under UNLICENSE.

This is a simple software that I wrote mostly for myself, but if someone finds it useful, then use it.