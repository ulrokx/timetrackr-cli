use cli::run_cli;

mod cli;
mod client;
mod commands;
mod http_client;

fn main() {
    let client = http_client::HttpTimeTrackerClient::new(String::from("http://localhost:8080"));
    run_cli(client);
}
