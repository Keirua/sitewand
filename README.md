Duct-tape powered static site generator.

Against all odds, it powers my [personnal homepage](www.keiruaprod.fr).

# Features

 - cli tool
 - json configuration
 - routing
 - Jinja-based templates
 - filesystem watcher :D

# How it works

 - templates are in the template/directory
 - watch.sh watches the modifications of the templates, and rebuilds the static site
 - a conf.json describes the list of routes

# Todo

To be honest the code right now it's quite shameful (`unwrap`...) but, hey, it does the job OK ?

Some stuff I'd like to add in a near future:

 - [ ] generation of the sitemap.xml
 - [ ] dealing with assets