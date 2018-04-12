extern crate ansi_term;
extern crate failure;
extern crate rawr;
extern crate regex;
extern crate rlua;

// expose database module for storing replies
pub mod db;

// reddit crate module wrapper
mod reddit;

use ansi_term::Colour::Yellow;
use ansi_term::Colour::Green;

use db::Database;

use rawr::client::RedditClient;

use reddit::RedditContent;

use regex::Regex;

use rlua::Lua;

use failure::Error;

pub type LibResult<T> = Result<T, Error>;

// This file is part of Reddit Sexuality Definition Bot.
//
// Reddit Sexuality Definition Bot is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Reddit Sexuality Definition Bot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with Reddit Sexuality Definition Bot.  If not, see <http://www.gnu.org/licenses/>.

// converts errors into rLua external errors
fn propagate_to_rlua(error: Error) -> rlua::Error {
    rlua::Error::ExternalError(std::sync::Arc::new(error.into()))
}

// invokes the lua instance on the behaviour script
// making the reddit content information available to the script
fn respond_to_comment(content: &RedditContent, database: &Database) -> LibResult<()> {
    // create a lua instance to define comment reply behaviour
    let lua = Lua::new();
    let globals = lua.globals();

    {
        let body = content.body();
        match body {
            Some(body) => {
                println!("{} '{}'", Yellow.paint("Comment"), &body);
                if content.is_comment() {
                    globals.set("comment", body)?;
                } else {
                    globals.set("post", body)?;
                }
            }
            None => (),
        }
    }
    {
        let title = content.title();
        match title {
            Some(title) => {
                println!("{} '{}'", Green.paint("Title"), &title);
                globals.set("title", title)?;
            },
            None => (),
        }
    }
    {
        let link_url = content.link_url();
        match link_url {
            Some(link_url) => {
                println!("{} '{}'", Green.paint("Link"), &link_url);
                globals.set("link", link_url)?;
            },
            None => (),
        }
    }
    {
        let content_type = content.content_type();
        globals.set("__type", content_type)?;
    }

    let contains = lua.create_function(
            |_, (substring1, substring2): (String, String)| {
        Ok(substring1.contains(&substring2))
    })?;
    globals.set("contains", contains)?;

    // this should be prefered to trying to convert case in lua as
    // rust handles Unicode properly
    let contains_ignore_case = lua.create_function(
            |_, (substring1, substring2): (String, String)| {
        Ok(substring1.to_lowercase().contains(&substring2.to_lowercase()))
    })?;
    globals.set("containsIgnoreCase", contains_ignore_case)?;

    let to_lowercase = lua.create_function(
            |_, string: String| {
        Ok(string.to_lowercase())
    })?;
    globals.set("toLowercase", to_lowercase)?;

    let matches_regex = lua.create_function(
            |_, (string, regex): (String, String)| {
        // TODO avoid recompilation
        // https://docs.rs/regex/0.2.10/regex/#example-avoid-compiling-the-same-regex-in-a-loop
        let regex = Regex::new(&regex);
        return match regex {
            Ok(regex) => Ok(regex.is_match(&string)),
            Err(e) => Err(propagate_to_rlua(e.into())),
        }
    })?;
    globals.set("matchesRegex", matches_regex)?;

    // create a scope within which reply function is defined for use
    // function for lua to reply to the comment
    // would not compile outside the scope because database
    // will never exist for a static lifetime
    lua.scope(|scope| {
        lua.globals().set(
            "reply",
            scope.create_function_mut(|_, reply: String| {

                let result = content.reply(&reply).and_then(|_| {
                    database.reply(content)
                });
                return match result {
                    Ok(()) => Ok(()),
                    Err(e) => Err(propagate_to_rlua(e)),
                }
            })?,
        )?;

        // run the code, the result is not used for anything
        // but errors should propagate
        // must run within the scope because the reply function will not be valid after
        lua.eval::<()>("require('behaviour')", Some("behaviour script"))
    })?;

    Ok(())
}

// runs the comment search and reply on each subreddit
pub fn run(subreddits: &Vec<String>, client: &RedditClient, database: &Database) -> LibResult<()> {
    for subreddit in subreddits {
        let subreddit = client.subreddit(subreddit);
        reddit::search(&subreddit, database)?;
    }
    Ok(())
}
