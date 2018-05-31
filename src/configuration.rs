extern crate failure;
extern crate json;
extern crate rawr;

use self::json::JsonValue;

use db;
use db::Database;

use rawr::auth::PasswordAuthenticator;
use rawr::client::RedditClient;

use LibResult;

use std::fs::File;
use std::io::Read;

/*
 * The Configuration data for the bot contained in one struct
 */
pub struct Configuration {
    pub authentication: AuthenticationData,
    pub subreddits: Vec<String>,
    pub database: Database,
    pub client: RedditClient,
}

pub struct AuthenticationData {
    pub id: String,
    pub secret: String,
    pub username: String,
    pub password: String,
}

pub fn get() -> LibResult<Configuration> {
    let authentication = read_json("authentication.json").and_then(|mut json| {
        Ok(AuthenticationData {
            id: String::from(json.remove("id").as_str().expect("No id specified in json")),
            secret: String::from(json.remove("secret").as_str().expect("No secret specified in json")),
            username: String::from(json.remove("username").as_str().expect("No id specified in json")),
            password: String::from(json.remove("password").as_str().expect("No id specified in json"))
        })
    })?;

    let subreddits = read_json("subreddits.json").and_then(|json| {
        get_subreddits(json)
    })?;

    let authenticator = PasswordAuthenticator::new(
        &authentication.id,
        &authentication.secret,
        &authentication.username,
        &authentication.password
    );
    let client = RedditClient::new("linux:rawr:v0.0.1 (by /u/skeletonxf)", authenticator);

    let database = db::from_connection("db.sqlite").or_else(|e| {
        eprintln!("Problem with SQL database: {}", e);
        Err(e)
    })?;
    Ok(Configuration {
        authentication,
        subreddits,
        database,
        client
    })
}

fn read_file(name: &str) -> LibResult<String> {
    let mut file = File::open(name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_json(name: &str) -> LibResult<JsonValue> {
    let contents = read_file(name)?;
    let parsed = json::parse(&contents)?;
    Ok(parsed)
}

// Returns a list of subreddit Strings specified in the subreddits.json
fn get_subreddits(data: JsonValue) -> LibResult<Vec<String>> {
    let mut list = Vec::new();
    for element in data.members() {
        let name = element.as_str().expect("All elements in subreddits.json array should be strings"); // TODO;
        list.push(name.to_owned());
    }
    println!("Subreddits '{}'", data.dump());
    Ok(list)
}
