import praw
import pdb
import re
import os
import time
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

END_OF_REPLY_MSG = "\n\n I'm a bot and this message was performed automatically. Contact /u/skeletonxf for issues. I'm also [open source](https://github.com/Skeletonxf/reddit-sexuality-definition-bot)"


# Check that the file that contains our username exists
if not os.path.isfile("config_bot.py"):
    print ("You must create a config file with your username and password.")
    print ("Please see config_skel.py")
    exit(1)

print("loading definition bot")

r = praw.Reddit(client_id=REDDIT_ID,
                client_secret=REDDIT_SECRET,
                refresh_token=REFRESH_TOKEN,
                user_agent=USER_AGENT)

#r.refresh_access_information(REFRESH_TOKEN)

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

print(replied_to)

print("about to search for comments to reply to")

def add_reply_if_valid(comment,triggers,reply,flag):
    "This checks for a comment in the comments to reply to and then replies avoiding quotes"
    for trigger in triggers:
        # search using regex for the trigger in the comment body
        for specific_trigger in re.finditer(trigger,comment.body,re.IGNORECASE):
            # splice substring going from comment body start to the trigger text location
            sub_comment = comment.body[:specific_trigger.start()]
            last_line_break = -1
            last_quote = -1
            # loop through substring looking for occurances of newlines
            for line_break in re.finditer("\n\n",sub_comment):
                last_line_break = line_break.start()
            
            # loop through substring looking for occurances of reddit quotes starting
            for quote in re.finditer("> ",sub_comment):
                last_quote = quote.start()
            
            # if user's comment has no quotes in
            if last_quote == -1:
                return flag + reply + "\n\n"
            else:
                # if user's comment has quotes in but the quote ends before the trigger keyword
                if not last_line_break == -1 and last_line_break > last_quote:
                    return flag + reply + "\n\n"
    return flag

def post_definitions():
    "Gets the top 5 posts from the subreddit"
    for subreddit_name in SUBREDDITS:
        subreddit = r.subreddit(subreddit_name)
        for submission in subreddit.hot(limit=5):
            # get the comments
            comments = submission.comments.list()
            # flag variable to stay as true if the bot replies to any comments
            reply_text = ""
            for comment in comments:
                if comment.id not in replied_to:
                    # avoid replying in more comments
                    if isinstance(comment, praw.models.Comment):
                        reply_text = add_reply_if_valid(comment,["\[\[Demisexuality\]\]", "\[\[Demisexual\]\]"],"A demisexual is a person who may experience sexual attraction but only after forming a strong emotion connection with that person(s). [Learn More](https://www.reddit.com/r/demisexuality/comments/2osqfz/links_and_resources_masterpost/)",reply_text)
                        reply_text = add_reply_if_valid(comment,["\[\[Asexuality\]\]", "\[\[Asexual\]\]"],"An asexual is a person who does not experience sexual attraction. [Learn More](http://www.asexuality.org/home/?q=overview.html)",reply_text)
                        reply_text = add_reply_if_valid(comment,["\[\[Gr[ae]y[ \-]Asexuality\]\]", "\[\[Gr[ae]y[ \-]A\]\]", "\[\[Gr[ae]ysexual\]\]"],"A grey asexual is a person at neither end of the spectrum on (a)sexual attraction. It can be used as an umbrella term for those who do not feel they fit as allosexual or asexual. [Learn More](http://www.asexuality.org/wiki/index.php?title=Gray-A_/_Grey-A)",reply_text)
                        reply_text = add_reply_if_valid(comment,["\[\[Autochorisexuality\]\]", "\[\[Autochoris\]\]", "\[\[Autochorisexual\]\]", "\[\[Autochorissexual\]\]", "\[\[Autochorissexuality\]\]"],"An autochorisexual person is in a subset of asexuality where there is a disconnect between oneself and a sexual target or object. For example a lack of desire to be a participant in sexual activies though still fantastising about sex. [Learn More](http://asexuals.wikia.com/wiki/Autochorissexual)",reply_text)
                        reply_text = add_reply_if_valid(comment,["\[\[Bisexuality\]\]", "\[\[Bisexual\]\]", "\[\[Bi\]\]"],"A bisexual person is a person who experiences sexual attraction to at least two genders. Some may be only attracted to men/women while others may consider themselves attracted to same sex and different sex - not excluding minority genders. [~~Learn More~~](https://www.reddit.com/r/SexualityDefBot/comments/510m0s/definitions/)",reply_text)
                        reply_text = add_reply_if_valid(comment,["\[\[Pansexuality\]\]", "\[\[Pansexual\]\]", "\[\[Pan\]\]"],"A pansexual person is a person who experiences sexual attraction irrespective of gender / to all genders. Some may consider themseles to be gender blind as apposed to gender attracted or to be attracted to a person's personality rather than gender or appearence. [~~Learn More~~](https://www.reddit.com/r/SexualityDefBot/comments/510m0s/definitions/)",reply_text)
                        # avoid replying to any more comments after this run
                        # if flag_replied is no longer empty
                        if not reply_text == "":
                            print("Bot replying to comment: " + comment.id)
                            # limits on reddit api
                            try:
                                comment.reply(reply_text+END_OF_REPLY_MSG)
                                # Store the comment id into the list
                                replied_to.append(comment.id)
                            except praw.exceptions.APIException as e:
                                print("Replying too soon or to an out of date comment!")
                                # TODO get sleep time for rate limit again
                                #print("Waiting " + str(e.sleep_time) + " seconds")
                                #time.sleep(e.sleep_time)
                                #comment.reply(reply_text+END_OF_REPLY_MSG)
                                #replied_to.append(comment.id)
                            # at most reply once per run
                            return
                else:
                    print("Already replied to comment: " + comment.id)
post_definitions()
                 
print("finished looking at comments")

print("writing replies to file")

# Write our updated list back to the file
with open("posts_replied_to.txt", "w") as f:
    for id in replied_to:
        f.write(id + "\n")

print("finished running")	
