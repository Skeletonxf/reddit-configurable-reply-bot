extern crate failure;
extern crate rawr;
extern crate rlua;

use db::Database;

use rawr::options::ListingOptions;
use rawr::structures::comment::Comment;
use rawr::structures::subreddit::Subreddit;
use rawr::structures::submission::Submission;
use rawr::traits::Commentable;
use rawr::traits::Editable;

use rlua::Lua;

use failure::Error;

// expose database module for storing replies
pub mod db;

enum RedditContent<'a> {
    PostComment(&'a Comment<'a>),
    SelfPost(&'a Submission<'a>),
}

fn reply_to(commentable: &Commentable, reply: &str) -> Result<(), Error> {
    println!("Replying: {}", reply);
    commentable.reply(reply)?;
    Ok(())
}

// invokes the a lua instance on the behaviour script and makes the comment body
// available to the script
fn respond_to_comment(content: &RedditContent, database: &Database) -> Result<(), Error> {
    // create a lua instance to define comment reply behaviour
    let lua = Lua::new();
    let globals = lua.globals();

    match content {
        &RedditContent::PostComment(comment) => {
            // print out comment and post title
            // safe because this is always a comment
            let comment_body = comment.body().unwrap();
            println!("Comment '{}'", comment_body);

            // if these fail then the lua script will not work either
            globals.set("comment", comment_body)?;
        },
        &RedditContent::SelfPost(post) => {
            // will always be safe to unwrap in self posts
            let post_body = post.body().unwrap();
            let post_title = post.title();
            println!("Post '{}'\n'{}'", post_title, post_body);

            // if these fail then the lua script will not work either
            globals.set("post", post_body)?;
            globals.set("title", post_title)?;
        }
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

    // create a scope within which reply function is defined for use
    // function for lua to reply to the comment
    // would not compile outside the scope because database
    // will never exist for a static lifetime
    lua.scope(|scope| {
        lua.globals().set(
            "reply",
            scope.create_function_mut(|_, reply: String| {
                match content {
                    &RedditContent::PostComment(comment) => {
                        let result = reply_to(comment, &reply).and_then(|_| {
                            database.reply(comment)
                        });
                        return match result {
                            Ok(()) => Ok(()),
                            // convert errors into rLua external error
                            Err(e) => Err(rlua::Error::ExternalError(std::sync::Arc::new(e))),
                        }
                    },
                    &RedditContent::SelfPost(post) => {
                        let result = reply_to(post, &reply).and_then(|_| {
                            database.reply(post)
                        });
                        return match result {
                            Ok(()) => Ok(()),
                            // convert errors into rLua external error
                            Err(e) => Err(rlua::Error::ExternalError(std::sync::Arc::new(e))),
                        }
                    },
                }
            })?,
        )?;

        // run the code, the result is not actually used for anything
        // but errors should propagate
        // must run within the scope because the reply function will not be valid after
        lua.eval::<()>("require('behaviour')", Some("behaviour script"))
    })?;

    Ok(())
}

// recurses through the comment tree
fn recurse_on_comment(comment: Comment, database: &Database) -> std::result::Result<(), Error> {

    if !database.replied(&comment)? {
        respond_to_comment(&RedditContent::PostComment(&comment), database)?;
    }

    let replies = comment.replies();
    if replies.is_ok() {
        for reply in replies.unwrap().take(10) {
            recurse_on_comment(reply, database)?;
        }
    } else {
        eprintln!("APIError on nested comment"); // TODO better debugging info
    }
    Ok(())
}

fn search_post(post: Submission, database: &Database) -> std::result::Result<(), Error> {
    // make a copy of the title to continue referring to after post is consumed
    let title = String::from(post.title()).clone();
    println!("Scanning '{}'", title);

    if post.is_self_post() && !database.replied(&post)? {
        respond_to_comment(&RedditContent::SelfPost(&post), database)?;
    }

    // give the post to `replies` which will consume it
    let comments = post.replies();
    if comments.is_ok() {
        let comments = comments.unwrap().take(100);
        for comment in comments {
            recurse_on_comment(comment, database)?;
            //println!("Comment in '{}':\n{}\n", &title, comment.body().unwrap())
        }
    } else {
        eprintln!("APIError on post {}", title);
    }
    Ok(())
}

// runs the comment search and reply
pub fn run(subreddits: Vec<Subreddit>, database: &Database) -> std::result::Result<(), Error> {

    for subreddit in subreddits {
        let about = subreddit.about();
        if about.is_ok() {
            println!("{} {}", subreddit.name, about.unwrap().display_name());
        } else {
            eprintln!("Could not fetch about data in {}", subreddit.name);
        }
        let hot = subreddit.hot(ListingOptions::default());
        if hot.is_ok() {
            for post in hot.unwrap().take(5) {
                println!("Found '{}' in '{}'", post.title(), subreddit.name);
                println!();
                search_post(post, database)?;
            }
        } else {
            eprintln!("APIError on subreddit {}", subreddit.name);
        }
    }
    Ok(())
}
