#!/usr/bin/env bash

# This script just calls `cargo run` to execute the bot and appends the output (and any errors) to a log file for inspecting. It mainly exists for use with cron jobs and other automation tools to periodically execute the bot.

# The time interval for running the bot should be chosen carefuly to avoid rate limits or attempting to run the bot when it is still running (ie not too frequently).

# https://stackoverflow.com/questions/7526971/how-to-redirect-both-stdout-and-stderr-to-a-file

# The colour codes can still be seen in the log file via `cat`, but will not be so easily viewed from a text editor. If you regularly inspect the logs and want to retain the coloured output then expanding this script with something like ansi2html https://github.com/ralphbean/ansi2html may be useful.

# Note that using this in cron means you need to specify the PATH and the directory to run from (the directory of this file).

~/.cargo/bin/cargo run >> logs.txt 2>&1

