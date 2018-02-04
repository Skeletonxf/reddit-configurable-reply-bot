extern crate json;
extern crate rawr;

use rawr::client::RedditClient;
use rawr::auth::PasswordAuthenticator;
use rawr::options::ListingOptions;

use std::fs::File;
use std::io::Read;
use std::io;

use json::JsonValue;

fn get_authentication() -> Result<JsonValue, io::Error> {
    let mut file = File::open("authentication.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(json::parse(&contents).expect("JSON parsing error"))
}

///
/// Produces the RedditClient from the data in authentication.json,
/// panicing if the data does not exist
///
fn get_client() -> RedditClient {
    let mut data = get_authentication().expect("Unable to open authentication.json");
    let authenticator = PasswordAuthenticator::new(
        data.remove("id").as_str().expect("No id specified in json"),
        data.remove("secret").as_str().expect("No secret specified in json"),
        data.remove("username").as_str().expect("No id specified in json"),
        data.remove("password").as_str().expect("No id specified in json")
    );
    RedditClient::new("linux:rawr:v0.0.1 (by /u/skeletonxf)", authenticator)
}

fn main() {
    let client = get_client();
    let all = client.subreddit("all");
    let listing = all.hot(ListingOptions::default()).expect("Could not fetch posts");
    for post in listing {
        println!("{}", post.title());
    }
}
