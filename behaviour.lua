-- Two new lines are needed for Reddit's markdown format
-- This is defined globally as it is used in all behaviour files too
newline = "\n\n"

-- This is a footer for the bot's posts, if you create your own bot
-- you should update this footer. This is also global as all behaviour
-- files concatenate this footer into the reply string.
footer = newline .. "*****" .. newline ..
  "I'm a bot and this message was performed automatically. " ..
  "Contact /u/skeletonxf for issues. I'm also " ..
  "[open source](https://github.com/Skeletonxf/reddit-configurable-reply-bot)"

local topics = require 'behaviour.weekly-topics'
topics.run()

local replies = require 'behaviour.comment-reply'
replies.run()

--error 'this will crash the bot'
