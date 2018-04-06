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

pub mod db;

// invokes the a lua instance on the behaviour script and makes the comment body
// available to the script
fn respond_to_comment(comment_body: &str) -> Result<bool, rlua::Error> {
    // create a lua instance to define comment reply behaviour
    let lua = Lua::new();

    let globals = lua.globals();

    // if these fail then the lua script will not work either
    globals.set("comment", comment_body).unwrap();

    // although the lua code should never need to query contains for anything
    // other than the comment body, this function needs to last for a static
    // lifetime and the comment body does not, so create a more general
    // contains function that words for any strings
    let contains = lua.create_function(|_, (substring1, substring2): (String, String)| {
        Ok(substring1.contains(&substring2))
    })?;
    // this should be prefered to trying to convert case in lua as
    // rust handles Unicode properly
    let contains_ignore_case = lua.create_function(|_, (substring1, substring2): (String, String)| {
        Ok(substring1.to_lowercase().contains(&substring2.to_lowercase()))
    })?;
    globals.set("contains", contains)?;
    globals.set("containsIgnoreCase", contains_ignore_case)?;
    let reply = lua.create_function(|_, comment: String| {
        println!("Totally replying: {}", comment);
        Ok(true)
    })?;
    globals.set("reply", reply)?;

    // run the code and take the result as a boolean
    // this will need changing into the reply string or even a table
    // specifying further info
    // or the lua state needs to be given a global function from rust
    // that performs the reply
    let result = lua.eval::<bool>("return require('behaviour')", Some("testing the script"))?;
    Ok(result)
}

// recurses through the comment tree
fn recurse_on_comment(title: &str, comment: Comment, database: &Database) {
    // print out comment and post title
    let comment_body = comment.body().unwrap(); // safe because this is always a comment
    println!("Comment in '{}':\n{}\n", title, comment_body);

    if !database.replied(&comment) {
        // TODO handle replying to comment
        match respond_to_comment(&comment_body) {
            Err(e) => println!("Lua error {}", e),
            Ok(v) => {
                println!("Lua returned {}", v);
                database.reply(&comment)
            }
        }
    }

    let replies = comment.replies();
    if replies.is_ok() {
        for reply in replies.unwrap().take(10) {
            recurse_on_comment(title, reply, database);
        }
    } else {
        println!("APIError on nested comment"); // TODO better debugging info
    }
}

fn search_post(post: Submission, database: &Database) {
    // make a copy of the title to continue referring to after post is consumed
    let title = String::from(post.title()).clone();
    if post.is_self_post() && !database.replied(&post) {
        // will always be safe to unwrap the body in self posts
        println!("Post '{}' contents:\n{}\n", title, post.body().unwrap());
        // todo create an enum to identify if comment or post
        // TODO handle replying to comment
        let post_comment = &[&title, "\n", &post.body().unwrap()].join("");
        match respond_to_comment(post_comment) {
            Err(e) => println!("Lua error {}", e),
            Ok(v) => {
                println!("Lua returned {}", v);
                database.reply(&post)
            }
        }
    }
    // give the post to `replies` which will consume it
    let comments = post.replies();
    if comments.is_ok() {
        let comments = comments.unwrap().take(100);
        for comment in comments {
            // deref the String to pass to the recurse with the ampersand
            recurse_on_comment(&title, comment, database);
            //println!("Comment in '{}':\n{}\n", &title, comment.body().unwrap())
        }
    } else {
        println!("APIError on post {}", title);
    }
}

// runs the comment search and reply
pub fn run(subreddits: Vec<Subreddit>, database: &Database) {
    for subreddit in subreddits {
        let about = subreddit.about();
        if about.is_ok() {
            println!("{} {}", subreddit.name, about.unwrap().display_name());
        } else {
            println!("Could not fetch about data in {}", subreddit.name);
        }
        let hot = subreddit.hot(ListingOptions::default());
        if hot.is_ok() {
            for post in hot.unwrap().take(5) {
                println!("Found '{}' in '{}'", post.title(), subreddit.name);
                println!();
                search_post(post, database)
            }
        } else {
            println!("APIError on subreddit {}", subreddit.name);
        }
    }
}
