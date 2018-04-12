extern crate failure;
extern crate rawr;

use db::Database;

use LibResult;

use rawr::options::ListingOptions;
use rawr::structures::comment::Comment;
use rawr::structures::subreddit::Subreddit;
use rawr::structures::submission::Submission;
use rawr::traits::Commentable;
use rawr::traits::Content;
use rawr::traits::Editable;

use respond_to_comment;

/*
 * Reddit module to handle rawr api, exposing RedditContent for other
 * modules to use
 */

pub enum RedditContent<'a> {
    PostComment(&'a Comment<'a>),
    SelfPost(&'a Submission<'a>),
    LinkPost(&'a Submission<'a>),
}

impl <'a> RedditContent<'a> {
    /*
     * Gets the name of the content, for storing in the db.
     */
    pub fn name(&self) -> &str {
        self.info().name()
    }

    /*
     * Gets the body of the reddit content.
     * Link posts have no body so return None
     */
    pub fn body(&self) -> Option<String> {
        match self {
            &RedditContent::PostComment(comment) => Some(comment.body().unwrap()),
            &RedditContent::SelfPost(post) => Some(post.body().unwrap()),
            _ => None,
        }
    }
    /*
     * Gets the title of the reddit content.
     * Comments have no title so return None
     */
    pub fn title(&self) -> Option<String> {
        match self {
            &RedditContent::SelfPost(post) => Some(post.title().to_string()),
            &RedditContent::LinkPost(post) => Some(post.title().to_string()),
            _ => None,
        }
    }

    /*
     * Gets the link url of the reddit content.
     * This is only defined for link posts
     */
    pub fn link_url(&self) -> Option<String> {
        match self {
             &RedditContent::LinkPost(post) => Some(post.link_url().unwrap()),
             _ => None,
        }
    }

    /*
    * The type of reddit content as a string
    */
    pub fn content_type(&self) -> String {
        match self {
            &RedditContent::SelfPost(_) => String::from("self post"),
            &RedditContent::LinkPost(_) => String::from("link post"),
            &RedditContent::PostComment(_) => String::from("comment"),
        }
    }

    pub fn is_comment(&self) -> bool {
        match self {
            &RedditContent::PostComment(_) => true,
            _ => false,
        }
    }

    pub fn reply(&self, reply: &str) -> LibResult<()> {
        println!("Replying: {}", reply);
        self.commentable().reply(reply)?;
        Ok(())
    }

    fn commentable(&self) -> &Commentable {
        match self {
            &RedditContent::PostComment(comment) => comment,
            &RedditContent::SelfPost(post) => post,
            &RedditContent::LinkPost(post) => post,
        }
    }

    fn info(&self) -> &Content {
        match self {
            &RedditContent::PostComment(comment) => comment,
            &RedditContent::SelfPost(post) => post,
            &RedditContent::LinkPost(post) => post,
        }
    }
}

pub fn search(subreddit: &Subreddit, database: &Database) -> LibResult<()> {
    let about = subreddit.about();
    if about.is_ok() {
        println!("{} {}", subreddit.name, about.unwrap().display_name());
    } else {
        eprintln!("Could not fetch about data in {}", subreddit.name);
    }
    let hot = subreddit.hot(ListingOptions::default());
    if hot.is_ok() {
        for post in hot.unwrap().take(7) {
            println!("Found '{}' in '{}'", post.title(), subreddit.name);
            println!();
            search_post(post, database)?;
        }
    } else {
        eprintln!("APIError on subreddit {}", subreddit.name);
    }
    Ok(())
}

// Responds to the post if it has not been responded to already
// and then recurses on the comment tree
fn search_post(post: Submission, database: &Database) -> LibResult<()> {
    // make a copy of the title to continue referring to after post is consumed
    let title = String::from(post.title()).clone();
    println!("Scanning '{}'", title);

    if post.is_self_post() {
        let post = RedditContent::SelfPost(&post);
        if !database.replied(&post)? {
            respond_to_comment(&post, database)?;
        }
    } else {
        let post = RedditContent::LinkPost(&post);
        if !database.replied(&post)? {
            respond_to_comment(&post, database)?;
        }
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

// Responds to every comment in this tree that has not already been
// responded to.
fn recurse_on_comment(comment: Comment, database: &Database) -> LibResult<()> {
    {
        let comment = RedditContent::PostComment(&comment);
        if !database.replied(&comment)? {
            respond_to_comment(&comment, database)?;
        }
    }
    // consume the Comment to get its replies
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
