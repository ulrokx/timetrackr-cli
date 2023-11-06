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
        let response = self.client.post(url).json(&params).send();
        match response {
            Ok(response) => {
                if response.status().is_client_error() {
                    let response = response.json::<serde_json::Value>().unwrap();
                    Err(response["message"].to_string())
                } else {
                    Ok(String::from("Task added"))
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }

    fn list_task(&self, command: ListTaskCommand) -> Result<String, String> {
        let url = self.make_url("tasks");
        let mut params = HashMap::new();
        if let Some(group) = command.group {
            params.insert("group", group);
        }
        let response = self.client.get(url).query(&params).send();
        match response {
            Ok(response) => {
                if response.status().is_client_error() {
                    let response = response.json::<serde_json::Value>().unwrap();
                    Err(response["message"].to_string())
                } else {
                    Ok(response.json::<serde_json::Value>().unwrap().to_string())
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }

    fn add_group(&self, command: AddGroupCommand) -> Result<String, String> {
        let url = self.make_url("groups");
        let mut params = HashMap::new();
        params.insert("name", command.name);
        let response = self.client.post(url).json(&params).send();        
        // if status code is user error, print message field in json body
        match response {
            Ok(response) => {
                if response.status().is_client_error() {
                    let response = response.json::<serde_json::Value>().unwrap();
                    Err(response["message"].to_string())
                } else {
                    Ok(String::from("Group added"))
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }

    fn list_group(&self, _command: ListGroupCommand) -> Result<String, String> {
        let url = self.make_url("groups");
        let response = self.client.get(url).send();
        match response {
            Ok(response) => {
                if response.status().is_client_error() {
                    let response = response.json::<serde_json::Value>().unwrap();
                    Err(response["message"].to_string())
                } else {
                    Ok(response.json::<serde_json::Value>().unwrap().to_string())
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }

    fn add_time(&self, command: AddTimeCommand) -> Result<String, String> {
        let url = self.make_url("times");
        let mut params = HashMap::new();
        params.insert("task", command.task);
        let response = self.client.post(url).json(&params).send();
        match response {
            Ok(response) => {
                if response.status().is_client_error() {
                    let response = response.json::<serde_json::Value>().unwrap();
                    Err(response["message"].to_string())
                } else {
                    Ok(String::from("Time added"))
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }

    fn get_status(&self) -> Result<String, String> {
        let url = self.make_url("status");
        let response = self.client.get(url).send();
        match response {
            Ok(response) => {
                if response.status().is_client_error() {
                    let response = response.json::<serde_json::Value>().unwrap();
                    Err(response["message"].to_string())
                } else {
                    Ok(response.json::<serde_json::Value>().unwrap().to_string())
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }
}
