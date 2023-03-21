#!/bin/sh

ps -eL -o s,pid,tid,cls,pri,comm | grep "threaded-ml" | tr -s " " | cut -d " " -f 2,3 | xargs -L 1 cargo run --
ps -eL -o s,pid,tid,cls,pri,comm | grep "threaded-ml"