use std::collections::HashMap;

use crate::{
    client::TimeTrackerClient,
    commands::{
        AddGroupCommand, AddTaskCommand, AddTimeCommand, ListGroupCommand, ListTaskCommand,
    },
};

pub struct HttpTimeTrackerClient {
    client: reqwest::blocking::Client,
    base_url: String,
}

impl HttpTimeTrackerClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url,
        }
    }

    fn make_url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path)
    }
}

impl TimeTrackerClient for HttpTimeTrackerClient {
    fn add_task(&self, command: AddTaskCommand) -> Result<String, String>{
        let url = self.make_url("tasks");
        let mut params = HashMap::new();
        params.insert("name", command.name);
        params.insert("group", command.group);
        let response = self.client.post(&url).json(&params).send();
        if let Err(error) = response {
            return Err(error.to_string());
        } 
        Ok(String::from("Task added"))
    }

    fn list_task(&self, command: ListTaskCommand) -> Result<String, String> {
        let url = self.make_url("tasks");
        let mut params = HashMap::new();
        if let Some(group) = command.group {
            params.insert("group", group);
        }
        let response = self.client.get(&url).query(&params).send();
        if let Err(error) = response {
            return Err(error.to_string());
        }
        // join task names with newlines
        Ok(response.unwrap().json::<serde_json::Value>().unwrap().to_string())
    }

    fn add_group(&self, command: AddGroupCommand) -> Result<String, String> {
        let url = self.make_url("groups");
        let mut params = HashMap::new();
        params.insert("name", command.name);
        let response = self.client.post(&url).json(&params).send();
        if let Err(error) = response {
            return Err(error.to_string());
        }
        let response = response.unwrap().json::<serde_json::Value>().unwrap();
        Ok(response.to_string())
        
    }

    fn list_group(&self, _command: ListGroupCommand) -> Result<String, String> {
        let url = self.make_url("groups");
        let response = self.client.get(&url).send();
        if let Err(error) = response {
            return Err(error.to_string());
        }
        let response = response.unwrap().json::<serde_json::Value>().unwrap();
        Ok(response.to_string())
    }

    fn add_time(&self, command: AddTimeCommand) -> Result<String, String> {
        let url = self.make_url("times");
        let mut params = HashMap::new();
        params.insert("task", command.task);
        let response = self.client.post(&url).json(&params).send();
        if let Err(error) = response {
            return Err(error.to_string());
        }
        let response = response.unwrap().json::<serde_json::Value>().unwrap();
        Ok(response.to_string())
    }

    fn get_status(&self) -> Result<String, String> {
        let url = self.make_url("status");
        let response = self.client.get(&url).send();
        if let Err(error) = response {
            return Err(error.to_string());
        }
        let response = response.unwrap().json::<serde_json::Value>().unwrap();
        Ok(response.to_string())
    }
}
