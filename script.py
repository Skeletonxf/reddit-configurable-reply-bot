import praw
import pdb
import re
import os
from config_bot import * # get user and pass

'''
	The MIT License (MIT)
    
    Copyright (c) 2016 Skeletonxf
    
    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:
    
    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.
    
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
'''

END_OF_REPLY_MSG = "/n I'm a bot and this message was performed automatically. Contact /u/skeletonxf for issues. I'm also [open source](https://github.com/Skeletonxf/reddit-sexuality-definition-bot)"

# Check that the file that contains our username exists
if not os.path.isfile("config_bot.py"):
    print ("You must create a config file with your username and password.")
    print ("Please see config_skel.py")
    exit(1)

print("loading definition bot")

r = praw.Reddit(USER_AGENT)

r.set_oauth_app_info(REDDIT_ID,REDDIT_SECRET,REDDIT_URI)

r.refresh_access_information(REFRESH_TOKEN)

print("logged in on OAuth")

# Have we run this code before? If not, create an empty list
if not os.path.isfile("posts_replied_to.txt"):
    replied_to = []

# If we have run the code before, load the list of posts we have replied to
else:
    # Read the file into a list
    with open("posts_replied_to.txt", "r") as f:
        replied_to = f.read()
        replied_to = replied_to.split("\n")
		# remove empty values, but in Python 3 returns a filter object
        replied_to = filter(None, replied_to)
        # convert back to list
        replied_to = list(replied_to)

print("about to search for comments to reply to")

def replyToComment(comment,triggers,reply,flag):
    "This checks for a comment in the comments to reply to and then replies"
    if comment.id not in replied_to:
        for trigger in triggers:
            # search using regex for the trigger in the comment body
            if re.search(trigger,comment.body,re.IGNORECASE):
                comment.reply(reply+END_OF_REPLY_MSG)
                print ("Bot replying to comment: ", comment.body)
                return True
    return flag

# Get the top 5 posts from the subreddit
subreddit = r.get_subreddit(SUBREDDIT)
for submission in subreddit.get_hot(limit=5):
    # get the comments
    comments = praw.helpers.flatten_tree(submission.comments)
    # flag variable to stay as true if the bot replies to any comments
    flag_replied = False
    for comment in comments:
        flag_replied = replyToComment(comment,["[[Demisexuality]]", "[[Demisexual]]"],"A demisexual is a person who may experience sexual attraction but only after forming a strong emotion connection with someone. [Learn More](https://www.reddit.com/r/demisexuality/comments/2osqfz/links_and_resources_masterpost/)",flag_replied)
        flag_replied = replyToComment(comment,["[[Asexuality]]", "[[Asexual]]"],"An asexual is a person who does not experience sexual attraction. [Learn More](http://www.asexuality.org/home/?q=overview.html)",flag_replied)
        flag_replied = replyToComment(comment,["[[Autochorisexuality]]", "[[Autochoris]]", "[[Autochorisexual]]", "[[Autochorissexual]]", "[[Autochorissexuality]]"],"An autochorisexual person is in a subset of asexuality where there is a disconnect between oneself and a sexual target/object. For example a lack of desire to be a participant in sexual activies though still fantastising about sex. [Learn More](http://asexuals.wikia.com/wiki/Autochorissexual)",flag_replied)
    # avoid replying to any more comments after this run
    if flag_replied:
        # Store the comment id into the list
        replied_to.append(comment.id)

print("finished looking at comments")

print("writing replies to file")

# Write our updated list back to the file
with open("posts_replied_to.txt", "w") as f:
    for id in replied_to:
        f.write(id + "\n")

print("finished running")	
