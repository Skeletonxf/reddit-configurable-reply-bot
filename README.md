# reddit-configurable-reply-bot

## A Rust bot for writing reddit bots in Lua.

This repository contains a Rust bot to interact with reddit. The Rust code is highly configurable from a Lua script to control the reply behaviour, though the Rust code may not be high quality as I am still learning Rust. The bot scans a whitelisted list of subreddits, and only attempts to reply to a comment if it has not replied to it before. The `behavior.lua` script handles the logic of replying on this more abstract level of dealing with a single comment or  post, and calling `reply` if wanted.

The Rust code defines a number of globals for the Lua script:
- `author` - the reddit username that made this comment/post
- `comment` - the comment body if the Lua script is running on a comment
- `link` - the link url if the Lua script is running on a post
- `post` - the post body if the Lua script is running on a post
- `title` - the title of a post if the Lua script is running on a post
- `contains(string, substring) -> boolean` - a function that checks for a substring in a string
- `containsIgnoreCase(string, substring) -> boolean` - the same as `contains` but case insensitive
- `matchesRegex(string, regex) -> boolean` - a function that checks if the supplied string matches the regex ([Rust regex](https://docs.rs/regex/0.2.10/regex/#syntax) not Lua regex)
- `toLowercase(string) -> string` - a function that returns the lowercase version of a string (this is Unicode aware unlike Lua)
- `reply(string)` - replies to the post/comment (after replying the bot will not invoke your Lua script on this post/comment again, if you want to reply multiple times you need to do so in one pass and not wait to be reran)

Example bot that replies to 'ping' with 'pong' anywhere it sees it (this would be your `behaviour.lua` script).
```lua
local message = comment or post
if contains(message, 'ping') then
  reply('pong')
end
```
That was easy :)

Also you should look over the [Reddit bottiquette](https://www.reddit.com/wiki/bottiquette) before making your bot. This example would surely get your bot banned for spam and is only illustrative.

This repo also contains the legacy python script for the same behavior in the `python legacy` branch. The python script is no longer maintained.

While also being a configurable reply bot, the `behaviour.lua` script here is used to continue on the goal from the Python project of responding to comments containing specific words on /r/Demisexuality. The definition list is very much a work in progress and is still growing.

## Running

The Rust bot should run from a call to `cargo run` in the root directory. You will need to create an `authentication.json` file from the example `authentication.json.example` in order for the bot to connect to a reddit account to post from. The `subreddits.json` file contains an array of subreddits for the bot to scan and reply in.

## Credit

When I first made the Python version of this bot I mainly chose Python because I could not find a tutorial in any other language. I have come a long way since then, but could not have made the switch without the help of this online [book](https://doc.rust-lang.org/book/second-edition/).

## License

Reddit Configurable Reply Bot / Reddit Sexuality Definition Bot is free software: you can redistribute it and/or modify it under the terms of the GNU General Affero Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. The AGPL is quite restrictive, it requires source code to be made available even if you run your bot from a server to rather than distributing a binary for others to run. Github offers free hosting and you can get started by forking this repository.

(The now unmaintained python version of this project was licensed under the MIT license)

## Known issues

- Bot attempts to reply to archived content which fails and causes a crash. Because this fails the content is never added to the database of replies, and therefore the bot will crash every time it reaches the content.
