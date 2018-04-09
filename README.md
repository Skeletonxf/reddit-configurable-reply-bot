# reddit-sexuality-definition-bot

This repository contains a Rust bot to interact with reddit. The Rust code is highly configurable, though may not be high quality code as I am still learning rust. The bot scans a whitelisted list of subreddits, and only attempts to reply to a comment if it has not replied to it before. The `behavior.lua` script handles the logic of replying on this more abstract level of dealing with a single comment or self post, and calling `reply` if wanted. The bot does not yet handle link posts.

This repo also contains the legacy python script for the same behavior: responding to comments to provide set definitions of sexuality words. The definition list is very much a work in progress and still growing. The python script is no longer maintained.

## Running

The Rust bot should run from a call to `cargo run` in the root directory. You will need to create a `authentication.json` file from the example `authentication.json.example` in order for the bot to connect to a reddit account to post from. The `subreddits.json` file contains an array of subreddits for the bot to scan and reply in.

## Credit

When I first made the python version of this bot I only chose python because I could not find a tutorial in any other language. I have come a long way since then, but could not have made the switch without the help of this online [book](https://doc.rust-lang.org/book/second-edition/).

## License

Reddit Sexuality Definition Bot is free software: you can redistribute it and/or modify it under the terms of the GNU General Affero Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

(The now unmaintained python version of this project was licensed under the MIT license)
