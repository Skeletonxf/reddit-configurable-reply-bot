extern crate rawr;
extern crate rlua;

use rawr::options::ListingOptions;
use rawr::structures::comment::Comment;
use rawr::structures::subreddit::Subreddit;
use rawr::structures::submission::Submission;
use rawr::traits::Commentable;
use rawr::traits::Editable;

use rlua::Lua;

// invokes the a lua instance on the behaviour script and makes the comment body
// available to the script
fn respond_to_comment(comment_body: &str, behaviour: &str) -> Result<bool, rlua::Error> {
    // create a lua instance to define comment reply behaviour
    let lua = Lua::new();

    // if this fails then the lua script will not work either
    lua.globals().set("comment", comment_body).unwrap();

    // run the code and take the result as a boolean
    // this will need changing into the reply string or even a table
    // specifying further info
    // or the lua state needs to be given a global function from rust
    // that performs the reply
    let result = lua.eval::<bool>(behaviour, Some("testing the script"))?;
    Ok(result)
}

// recurses through the comment tree
fn recurse_on_comment(title: &str, comment: Comment, behaviour: &str) {
    // print out comment and post title
    let comment_body = comment.body().unwrap(); // safe because this is always a comment
    println!("Comment in '{}':\n{}\n", title, comment_body);

    // TODO handle replying to comment
    match respond_to_comment(&comment_body, behaviour) {
        Err(e) => println!("Lua error {}", e),
        Ok(v) => println!("Lua returned {}", v),
    }

    let replies = comment.replies();
    if replies.is_ok() {
        for reply in replies.unwrap().take(10) {
            recurse_on_comment(title, reply, behaviour);
        }
    } else {
        println!("APIError on nested comment"); // TODO better debugging info
    }
}

fn search_post(post: Submission, behaviour: &str) {
    // make a copy of the title to continue referring to after post is consumed
    let title = String::from(post.title()).clone();
    if post.is_self_post() {
        // will always be safe to unwrap the body in self posts
        println!("Post '{}' contents:\n{}\n", title, post.body().unwrap());
    }
    // give the post to `replies` which will consume it
    let comments = post.replies();
    if comments.is_ok() {
        let comments = comments.unwrap().take(100);
        for comment in comments {
            // deref the String to pass to the recurse with the ampersand
            recurse_on_comment(&title, comment, behaviour);
            //println!("Comment in '{}':\n{}\n", &title, comment.body().unwrap())
        }
    } else {
        println!("APIError on post {}", title);
    }
}

// runs the comment search and reply
pub fn run(subreddits: Vec<Subreddit>, behaviour: &str) {
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
                search_post(post, behaviour)
            }
        } else {
            println!("APIError on subreddit {}", subreddit.name);
        }
    }
}
