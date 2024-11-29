---
title: "[7/13/2012] Challenge #76 [difficult] (imgur album downloader)"
url: "https://old.reddit.com/r/dailyprogrammer/comments/wk0jf/7132012_challenge_76_difficult_imgur_album/"
---

Write a script that takes an [imgur](http://imgur.com) album id and an output directory as command line arguments (e.g., `./script DeOSG ./images`), and saves all images from the album in the output directory as `DeOSG-1.jpg`, `DeOSG-2.jpg`, etc.

**Hint**: To retrieve the picture URLs, parse the HTML page at "`http://imgur.com/a/`**(ID)**`/layout/blog`".