---
title: "[2021-06-14] Challenge #394 [Difficult] RSA encryption"
url: "https://old.reddit.com/r/dailyprogrammer/comments/nzmvsj/20210614_challenge_394_difficult_rsa_encryption/"
---

If you're not familiar with some of the background topics for today's challenge, you'll need to do some reading on your own. Feel free to ask if you're lost, but try to figure it out yourself first. This is a difficult challenge.

Implement the RSA key generation process following [the specification on Wikipedia](https://en.wikipedia.org/wiki/RSA_%28cryptosystem%29#Key_generation), or some other similar specification. Randomly generate 256-bit or larger values for `p` and `q`, using [the Fermat primality test](https://en.wikipedia.org/wiki/Primality_test#Fermat_primality_test) or something similar. Use `e = 65537`. Provide functions to encrypt and decrypt a whole number representing a message, using your selected `n`. Verify that when you encrypt and then decrypt the input `12345`, you get `12345` back.

It's recommended that you use a large-number library for this challenge if your language doesn't support big integers.

*(This is a repost of [Challenge #60 [difficult]](https://www.reddit.com/r/dailyprogrammer/comments/ukj67/642012_challenge_60_difficult/), originally posted by u/rya11111 in June 2012.)*