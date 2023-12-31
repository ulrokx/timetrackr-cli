use crate::{client::TimeTrackerClient, commands::*};
use clap::{Parser, Subcommand};
use std::io::Write;
use termcolor::{Color, StandardStream, ColorSpec, ColorChoice, WriteColor};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    noun: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(subcommand)]
    Task(TaskCommand),
    #[command(subcommand)]
    Group(GroupCommand),
    #[command(subcommand)]
    Time(TimeCommand),
    Status,
}

#[derive(Debug, Subcommand)]
enum TaskCommand {
    Add(AddTaskCommand),
    List(ListTaskCommand),
}

#[derive(Debug, Subcommand)]
enum GroupCommand {
    Add(AddGroupCommand),
    List(ListGroupCommand),
}

#[derive(Debug, Subcommand)]
enum TimeCommand {
    Add(AddTimeCommand),
}

fn print_error(error: String) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
    writeln!(stdout, "{}", error).unwrap();

}

fn handle_result(result: Result<String, String>) {
    match result {
        Ok(message) => println!("{}", message),
        Err(error) => print_error(error),
    }
}

pub fn run_cli(client: impl TimeTrackerClient) {
    let args = Arguments::parse();
    match args.noun {
        Command::Task(task) => match task {
            TaskCommand::Add(add) => handle_result(client.add_task(add)),
            TaskCommand::List(list) => handle_result(client.list_task(list)),
        },
        Command::Group(group) => match group {
            GroupCommand::Add(add) => handle_result(client.add_group(add)),
            GroupCommand::List(list) => handle_result(client.list_group(list)),
        },
        Command::Time(time) => match time {
            TimeCommand::Add(add) => handle_result(client.add_time(add)),
        },
        Command::Status => handle_result(client.get_status()),
    }
}
