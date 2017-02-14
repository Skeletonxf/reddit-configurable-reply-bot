# reddit-sexuality-definition-bot

This repo contains a python script for automated running of a bot to repspond to comments to provide set definitions
of sexuality words. The definition list is very much a work in progress and still growing.

## Credits

Much thanks to all of the following for making this script
http://pythonforengineers.com/build-a-reddit-bot-part-1/
https://www.reddit.com/r/GoldTesting/comments/3cm1p8/how_to_make_your_bot_use_oauth2/
https://github.com/praw-dev/praw
https://praw.readthedocs.io/en/latest/getting_started/authentication.html#installed-application


## Misc

The old Automoderator code for reddit's default subreddit is depreceated and no longer going to be
updated.

If you want this bot to run on your subreddit send a message to /u/skeletonxf

If you want to adjust or add definitions send a message or do a pull request

## Trying to run this yourself or build your own bot off this

You need to install pip and python then use pip to install praw. You need a reddit account from which you use https://www.reddit.com/prefs/apps/ and make an app and give it your account, permissions on your account and OAuth info to access your account. You need to then make a new file as instructed by this project called config_bot.py, here you fill out the OAuth data so your script can access your account, you do NOT commit this file anywhere public ever - the file name is in .gitignore for a reason.And then you can finally (hopefully) run this script. You may run into version confict issues if it's been a while since I've updated the script or whatever and will have to go look up changes to praw's API or anything else manually. Once your script does not crash anymore it will automatically set up a txt file on first run to track the comments the bot has replied to and use that as its memory - thus, this implementation would not scale well for a high acitvity / high use robot, the txt file would become insanely large. For automation you then just make a .bat or .sh (that you make your system run every so often) to jump to the right place on your file system and run 'python script.py', I found auotmating this on Windows caused popups so I had to use pythonw to prevent the creation of a cmd window on running. For automation I just put another .bat file in my startup folder, and had that one run without a visible cmd window either, and call the .bat for running the script in set intervals, that way the startup folder .bat didn't get crashed by unexpected behaviour in the script/reddit/praw.
