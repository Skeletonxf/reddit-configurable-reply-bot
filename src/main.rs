extern crate json;
extern crate rawr;
extern crate rlua;
extern crate sexuality_def_bot;

use rawr::auth::PasswordAuthenticator;
use rawr::client::RedditClient;
use rawr::structures::subreddit::Subreddit;

use rlua::Lua;

use std::fs::File;
use std::process;
use std::io::Read;
use std::io;

use json::JsonValue;

fn read_file(name: &str) -> Result<String, io::Error> {
    let mut file = File::open(name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
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
            "All elements in subreddits.json array should be strings");
        list.push(client.subreddit(name));
    }
    println!("{}", data.dump());
    list
}

fn main() {
    // need to work out how to pass and retrieve values: https://docs.rs/rlua/0.12.2/rlua/struct.Lua.html
    //let lua = Lua::new();
    //lua.eval::<()>(
    //   r#"
    //   print('hello world')
    //   "#,
    //   None,
    //);
    //fn respond_to(string : &str) will call lua code on contents of some Commentable

    let authentication_data = read_file("authentication.json").unwrap_or_else(|e| {
        println!("Problem reading authentication data: {}", e);
        process::exit(1);
    });
    let json_authentication_data = json::parse(&authentication_data).unwrap_or_else(|e| {
        println!("Problem parsing authentication data: {}", e);
        process::exit(1);
    });
    let client = get_client(json_authentication_data);

    let subreddits_data = read_file("subreddits.json").unwrap_or_else(|e| {
        println!("Problem reading subreddits data: {}", e);
        process::exit(1);
    });
    let json_subreddits_data = json::parse(&subreddits_data).unwrap_or_else(|e| {
        println!("Problem parsing subreddits data: {}", e);
        process::exit(1);
    });
    let subreddits = get_subreddits(json_subreddits_data, &client);

    sexuality_def_bot::run(subreddits);
}
