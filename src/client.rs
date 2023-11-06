use crate::commands::{
    AddGroupCommand, AddTaskCommand, AddTimeCommand, ListGroupCommand, ListTaskCommand,
};

pub trait TimeTrackerClient {
    fn add_task(&self, command: AddTaskCommand) -> Result<String, String>;
    fn list_task(&self, command: ListTaskCommand) -> Result<String, String>;
    fn add_group(&self, command: AddGroupCommand) -> Result<String, String>;
    fn list_group(&self, command: ListGroupCommand) -> Result<String, String>;
    fn add_time(&self, command: AddTimeCommand) -> Result<String, String>;
    fn get_status(&self) -> Result<String, String>;
}
