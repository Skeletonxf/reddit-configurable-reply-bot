extern crate rawr;

use rawr::options::ListingOptions;
use rawr::structures::comment::Comment;
use rawr::structures::subreddit::Subreddit;
use rawr::structures::submission::Submission;
use rawr::traits::Commentable;
use rawr::traits::Editable;

// recurses through the comment tree
fn recurse_on_comment(title: &str, comment: Comment) {
    // print out comment and post title
    println!("Comment in '{}':\n{}\n", title, comment.body().unwrap());
    // TODO handle replying to comment
    let replies = comment.replies();
    if replies.is_ok() {
        for reply in replies.unwrap().take(10) {
            recurse_on_comment(title, reply);
        }
    } else {
        println!("APIError on nested comment"); // TODO better debugging info
    }
}

fn search_post(post: Submission) {
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
            recurse_on_comment(&title, comment);
            //println!("Comment in '{}':\n{}\n", &title, comment.body().unwrap())
        }
    } else {
        println!("APIError on post {}", title);
    }
}

// runs the comment search and reply
pub fn run(subreddits: Vec<Subreddit>) {
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
                search_post(post)
            }
        } else {
            println!("APIError on subreddit {}", subreddit.name);
        }
    }
}
