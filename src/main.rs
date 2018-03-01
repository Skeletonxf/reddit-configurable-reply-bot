extern crate json;
extern crate rawr;
extern crate sexuality_def_bot;

use rawr::auth::PasswordAuthenticator;
use rawr::client::RedditClient;
use rawr::structures::subreddit::Subreddit;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io;
use std::process;

use json::JsonValue;

fn read_file(name: &str) -> Result<String, io::Error> {
    let mut file = File::open(name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_json(name: &str) -> Result<JsonValue, Box<Error>> {
    let contents = read_file(name)?;
    let parsed = json::parse(&contents)?;
    Ok(parsed)
}

/// Produces the RedditClient from the authentication data
fn get_client(mut data: JsonValue) -> RedditClient {
    let authenticator = PasswordAuthenticator::new(
        data.remove("id").as_str().expect("No id specified in json"),
        data.remove("secret").as_str().expect("No secret specified in json"),
        data.remove("username").as_str().expect("No id specified in json"),
        data.remove("password").as_str().expect("No id specified in json")
    );
    RedditClient::new("linux:rawr:v0.0.1 (by /u/skeletonxf)", authenticator)
}

// Returns a list of Subreddits specified in the subreddits.json
fn get_subreddits(data: JsonValue, client: &RedditClient) -> Vec<Subreddit> {
    let mut list = Vec::new();
    for element in data.members() {
        let name = element.as_str().expect(
            "All elements in subreddits.json array should be strings"); // TODO
        list.push(client.subreddit(name));
    }
    println!("{}", data.dump());
    list
}

fn main() {
    let authentication_data = read_json("authentication.json").unwrap_or_else(|e| {
        println!("Problem with authentication data: {}", e);
        process::exit(1);
    });
    let client = get_client(authentication_data);

    let json_subreddits_data = read_json("subreddits.json").unwrap_or_else(|e| {
        println!("Problem with subreddits data: {}", e);
        process::exit(1);
    });
    let subreddits = get_subreddits(json_subreddits_data, &client);

    let behaviour = read_file("behaviour.lua").unwrap_or_else(|e| {
        println!("Problem reading behaviour script: {}", e);
        process::exit(1);
    });

    sexuality_def_bot::run(subreddits, &behaviour).unwrap_or_else(|e| {
        println!("Problem running bot: {}", e);
        process::exit(1);
    });;
}
