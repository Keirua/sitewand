#!/bin/sh
# apt-get install inotify-tools
while inotifywait -e modify -r -q --format '%:e %f' templates/; do
  cargo run -- conf.json www
done