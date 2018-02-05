extern crate json;
extern crate rawr;

use rawr::auth::PasswordAuthenticator;
use rawr::client::RedditClient;
use rawr::structures::subreddit::Subreddit;
use rawr::options::ListingOptions;

use std::fs::File;
use std::io::Read;
use std::io;

use json::JsonValue;

fn read_file(name: &str) -> Result<JsonValue, io::Error> {
    let mut file = File::open(name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(json::parse(&contents).expect("JSON parsing error"))
}

fn read_json(name: &str) -> JsonValue {
    read_file(name).expect(&format!("{}{}", "Unable to read ", name))
}

///
/// Produces the RedditClient from the data in authentication.json,
///
fn get_client() -> RedditClient {
    let mut data = read_json("authentication.json");
    let authenticator = PasswordAuthenticator::new(
        data.remove("id").as_str().expect("No id specified in json"),
        data.remove("secret").as_str().expect("No secret specified in json"),
        data.remove("username").as_str().expect("No id specified in json"),
        data.remove("password").as_str().expect("No id specified in json")
    );
    RedditClient::new("linux:rawr:v0.0.1 (by /u/skeletonxf)", authenticator)
}

//
// Returns a list of Subreddits specified in the subreddits.json
//
fn get_subreddits(client: &RedditClient) -> Vec<Subreddit> {
    let mut list = Vec::new();
    let data = read_json("subreddits.json");
    for element in data.members() {
        let name = element.as_str().expect(
            "All elements in subreddits.json array should be strings");
        list.push(client.subreddit(name));
    }
    println!("{}", data.dump());
    list
}

fn main() {
    let client = get_client();
    let subreddits = get_subreddits(&client);
    for subreddit in subreddits {
        //println!("{}{}",
        //    subreddit.name,
        //    subreddit.about().expect("Could not fetch 'about' data").display_name()); // error in rawr
        let hot = subreddit.hot(ListingOptions::default());
        if hot.is_ok() {
            for post in hot.unwrap().take(5) {
                println!("Found '{}' in '{}'", post.title(), subreddit.name);
            }
        } else {
            println!("APIError on subreddit {}", subreddit.name);
        }
    }
}
