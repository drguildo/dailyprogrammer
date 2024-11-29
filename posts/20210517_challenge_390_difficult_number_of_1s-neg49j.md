---
title: "[2021-05-17] Challenge #390 [Difficult] Number of 1's"
url: "https://old.reddit.com/r/dailyprogrammer/comments/neg49j/20210517_challenge_390_difficult_number_of_1s/"
---

# Warmup

Given a number n, determine the number of times the digit "1" appears if you write out all numbers from 1 to n inclusive.

    f(1) = 1
    f(5) = 1
    f(10) = 2
    f(20) = 12
    f(1234) = 689
    f(5123) = 2557
    f(70000) = 38000
    f(123321) = 93395
    f(3^35) = 90051450678399649

You should be able to handle large inputs like 3^(35) efficiently, meaning much faster than iterating over all numbers from 1 to n. Find f(5^(20)) before posting your solution. The answer is 15 digits long and the sum of its digits is 74.

# Challenge

f(35199981) = 35199981. Efficiently find the sum of all n such that f(n) = n. This should take a fraction of a second, depending on your programming language.

The answer is 11 digits long and the sum of its digits is 53.

*(This is a repost of [Challenge #45 [difficult]](https://www.reddit.com/r/dailyprogrammer/comments/sv6xs/4272012_challenge_45_difficult/), originally posted by u/oskar_s in April 2012. Check that post for hints and more detail.)*