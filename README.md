# reddit-configurable-reply-bot

This repository contains a Rust bot to interact with reddit. The Rust code is highly configurable from a lua script to control the reply behaviour, though the Rust code may not be high quality as I am still learning Rust. The bot scans a whitelisted list of subreddits, and only attempts to reply to a comment if it has not replied to it before. The `behavior.lua` script handles the logic of replying on this more abstract level of dealing with a single comment or  post, and calling `reply` if wanted.

This repo also contains the legacy python script for the same behavior in the `python legacy` branch. The python script is no longer maintained.

While also being a configurable reply bot, the `behaviour.lua` script here is used to continue on the goal from the Python project of responding to comments to provide set definitions of sexuality words. The definition list is very much a work in progress and is still growing.

## Running

The Rust bot should run from a call to `cargo run` in the root directory. You will need to create a `authentication.json` file from the example `authentication.json.example` in order for the bot to connect to a reddit account to post from. The `subreddits.json` file contains an array of subreddits for the bot to scan and reply in.

## Credit

When I first made the Python version of this bot I only chose Python because I could not find a tutorial in any other language. I have come a long way since then, but could not have made the switch without the help of this online [book](https://doc.rust-lang.org/book/second-edition/).

## License

Reddit Configurable Reply Bot / Reddit Sexuality Definition Bot is free software: you can redistribute it and/or modify it under the terms of the GNU General Affero Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

(The now unmaintained python version of this project was licensed under the MIT license)

## Known issues

- Bot attempts to reply to archived content which fails and causes a crash. Because this fails the content is never added to the database of replies, and therefore the bot will crash every time it reaches the content.
