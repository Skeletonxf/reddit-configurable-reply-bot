extern crate ansi_term;
extern crate failure;
extern crate rawr;

use ansi_term::Colour::Blue;
use ansi_term::Colour::Green;
use ansi_term::Colour::Purple;

use configuration::Configuration;

use LibResult;

use rawr::options::ListingOptions;
use rawr::structures::comment::Comment;
use rawr::structures::subreddit::Subreddit;
use rawr::structures::submission::Submission;
use rawr::traits::Commentable;
use rawr::traits::Content;
use rawr::traits::Editable;

/*
 * Reddit module to handle rawr api, exposing RedditContent for other
 * modules to use
 */

pub enum RedditContent<'a, 'b> {
    PostComment(&'a Comment<'a>, &'b Configuration),
    SelfPost(&'a Submission<'a>, &'b Configuration),
    LinkPost(&'a Submission<'a>, &'b Configuration),
}

pub fn new_reddit_content_from_post<'a, 'b>(
    post: &'a Submission,
    config: &'b Configuration,
) -> RedditContent<'a, 'b> {
    if post.is_self_post() {
        RedditContent::SelfPost(post, config)
    } else {
        RedditContent::LinkPost(post, config)
    }
}

pub fn new_reddit_content_from_comment<'a, 'b>(
    comment: &'a Comment,
    config: &'b Configuration,
) -> RedditContent<'a, 'b> {
    RedditContent::PostComment(comment, config)
}

impl <'a, 'b> RedditContent<'a, 'b> {
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
            &RedditContent::PostComment(comment, _) => Some(
                // If no comment body return empty string.
                // This should happen only in unusual circumstances
                // like deleted comments.
                comment.body().unwrap_or(String::new())),
            &RedditContent::SelfPost(post, _) => Some(
                // If no self post body then return empty string.
                // The self post that broke the bot prior to
                // this fix:
                // https://www.reddit.com/r/demisexuality/comments/9ian5v/do_demisexuals_experience_actual_lust_for_their/
                post.body().unwrap_or(String::new())),
                // Link posts are never expected to have a body
            _ => None,
        }
    }

    /*
     * Gets the title of the reddit content.
     * Comments have no title so return None
     */
    pub fn title(&self) -> Option<String> {
        match self {
            &RedditContent::SelfPost(post, _) => Some(post.title().to_string()),
            &RedditContent::LinkPost(post, _) => Some(post.title().to_string()),
            _ => None,
        }
    }

    /*
     * Gets the link url of the reddit content.
     * This is only defined for link posts
     */
    pub fn link_url(&self) -> Option<String> {
        match self {
             &RedditContent::LinkPost(post, _) => Some(
                 post.link_url().expect("Failed to get link url of link post")),
             _ => None,
        }
    }

    /**
     * Gets the name of the author of this reddit content.
     */
    pub fn author(&self) -> String {
        match self {
            &RedditContent::PostComment(post, _) => post.author().name,
            &RedditContent::SelfPost(post, _) => post.author().name,
            &RedditContent::LinkPost(post, _) => post.author().name,
        }
    }

    /*
    * The type of reddit content as a string
    */
    pub fn content_type(&self) -> String {
        match self {
            &RedditContent::SelfPost(_, _) => String::from("self post"),
            &RedditContent::LinkPost(_, _) => String::from("link post"),
            &RedditContent::PostComment(_, _) => String::from("comment"),
        }
    }

    pub fn is_comment(&self) -> bool {
        match self {
            &RedditContent::PostComment(_, _) => true,
            _ => false,
        }
    }

    /*
     * Checks if the content has been replied to by the bot already,
     * either due to an existing bot's comment in reply or because
     * the bot has logged this reply in the database
     * (in case its comment was deleted)
     */
    pub fn replied_to(&self) -> LibResult<(bool)> {
        // create a copy of the content to fetch its replies as the copy will
        // be consumed
        let replies = match self {
            &RedditContent::PostComment(comment, _) => comment.clone().replies(),
            &RedditContent::SelfPost(post, _) => post.clone().replies(),
            &RedditContent::LinkPost(post, _) => post.clone().replies(),
        };
        // check database first as it doesn't use up rate limits
        Ok(
            self.config().database.replied(self.name())? ||
            replies?.any(|c| {
                c.author().name == self.config().authentication.username
            })
        )
    }

    /*
     * Replies to the comment and logs it in the database
     */
    pub fn reply(&self, reply: &str) -> LibResult<()> {
        println!("{} {}", Blue.paint("Replying:"), reply);
        self.commentable().reply(reply)?; // TODO check why this might have failed
        println!("{}", Green.paint("Replied, saving to db"));
        self.config().database.reply(self.name())?;
        Ok(())
    }

    fn commentable(&self) -> &'a Commentable {
        match self {
            &RedditContent::PostComment(comment, _) => comment,
            &RedditContent::SelfPost(post, _) => post,
            &RedditContent::LinkPost(post, _) => post,
        }
    }

    fn info(&self) -> &'a Content {
        match self {
            &RedditContent::PostComment(comment, _) => comment,
            &RedditContent::SelfPost(post, _) => post,
            &RedditContent::LinkPost(post, _) => post,
        }
    }

    fn config(&self) -> &'b Configuration {
        match self {
            &RedditContent::PostComment(_, config) => config,
            &RedditContent::SelfPost(_, config) => config,
            &RedditContent::LinkPost(_, config) => config,
        }
    }
}

// Since the crawler owns the Subreddit its lifetime is only tied to the config reference
pub struct SubredditCrawler<'c> {
    config: &'c Configuration,
    subreddit: Subreddit<'c>,
}

/*
 * Creates a crawler for this subreddit and configuration
 */
pub fn new_subreddit_crawler<'c>(
    subreddit: &str,
    config: &'c Configuration,
) -> SubredditCrawler<'c> {
    SubredditCrawler {
        config,
        subreddit: config.client.subreddit(subreddit)
    }
}

impl<'c> SubredditCrawler<'c> {
    /*
     * Runs the crawler with this behavior
     */
    pub fn run<F>(&self, behavior: &F) -> LibResult<()>
    where F: Fn(&RedditContent) -> LibResult<()> {
        self.search(behavior)
    }

    fn search<F>(&self, behavior: &F) -> LibResult<()>
    where F: Fn(&RedditContent) -> LibResult<()> {
        let about = self.subreddit.about();
        if about.is_ok() {
            println!("{} {} {}", Purple.paint("Subreddit"), self.subreddit.name,
                    about.expect("Failed to get subreddit about").display_name());
        } else {
            eprintln!("Could not fetch about data in {}", self.subreddit.name);
        }
        let hot = self.subreddit.hot(ListingOptions::default())?;
        for post in hot.take(13) {
            println!("Found '{}' in '{}'", post.title(), self.subreddit.name);
            println!();
            self.search_post(post, &behavior)?;
        }
        Ok(())
    }

    /*
     * Scans the post, possibly replying and then recurses on the post comments
     */
    fn search_post<F>(&self, post: Submission, behavior: &F) -> LibResult<()>
    where F: Fn(&RedditContent) -> LibResult<()> {
        // make a copy of the title to continue referring to after post is consumed
        let title = String::from(post.title()).clone();
        println!("Scanning '{}'", title);

        {
            let content = new_reddit_content_from_post(&post, self.config);
            if !content.replied_to()? {
                behavior(&content)?;
            }
            // take back the post
        }

        // give the post to `replies` which will consume it
        let comments = post.replies()?.take(100);
        for comment in comments {
            self.scan_comment_tree(comment, behavior)?;
        }
        Ok(())
    }

    fn scan_comment_tree<F>(&self, comment: Comment, behavior: &F) -> LibResult<()>
    where F: Fn(&RedditContent) -> LibResult<()> {
        {
            let content = new_reddit_content_from_comment(&comment, self.config);

            if !content.replied_to()? {
                behavior(&content)?;
            }
            // take back the comment
        }

        // consume the Comment to get its replies
        let replies = comment.replies()?;
        for reply in replies.take(10) {
            self.scan_comment_tree(reply, behavior)?;
        }
        Ok(())
    }
}
