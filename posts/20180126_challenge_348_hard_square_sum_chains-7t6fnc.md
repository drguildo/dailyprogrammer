---
title: "[2018-01-26] Challenge #348 [Hard] Square Sum Chains"
url: "https://old.reddit.com/r/dailyprogrammer/comments/7t6fnc/20180126_challenge_348_hard_square_sum_chains/"
---

#Description

For this challenge your task is, given a number N, rearrange the numbers 1 to N so that all adjacent pairs of numbers sum up to square numbers. 

There might not actually be a solution. There also might be multiple solution. You are only required to find one, if possible.

For example, the smallest number for which this is possbile is 15:

    8 1 15 10 6 3 13 12 4 5 11 14 2 7 9

     8 +  1 =  9 = 3^2
     1 + 15 = 16 = 4^2
    15 + 10 = 25 = 5^2
    10 +  6 = 16 = 4^2
    ...

#Example Input

    15
    8

#Example Output

    8 1 15 10 6 3 13 12 4 5 11 14 2 7 9
    Not possible

#Challenge Input

    23
    24
    25
    256

# Credit

This challenge was suggested by user /u/KeinBaum, many thanks. If you have an idea for a challenge, please share it in /r/dailyprogrammer_ideas and there's a good chance we'll use it.
