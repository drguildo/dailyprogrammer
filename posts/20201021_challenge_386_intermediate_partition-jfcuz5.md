---
title: "[2020-10-21] Challenge #386 [Intermediate] Partition counts"
url: "https://old.reddit.com/r/dailyprogrammer/comments/jfcuz5/20201021_challenge_386_intermediate_partition/"
---

Today's challenge comes from [a recent Mathologer video](https://www.youtube.com/watch?v=iJ8pnCO0nTY).

# Background

There are 7 ways to partition the number 5 into the sum of positive integers:

    5 = 1 + 4 = 1 + 1 + 3 = 2 + 3 = 1 + 2 + 2 = 1 + 1 + 1 + 2 = 1 + 1 + 1 + 1 + 1

Let's express this as `p(5) = 7`. If you write down the number of ways to partition each number starting at 0 you get:

    p(n) = 1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, ...

By convention, `p(0) = 1`.

# Challenge

Compute `p(666)`. You must run your program all the way through to completion to meet the challenge. To check your answer, `p(666)` is a 26-digit number and the sum of the digits is 127. Also, `p(66) = 2323520`.

You can do this using the definition of `p(n)` above, although you'll need to be more clever than listing all possible partitions of 666 and counting them. Alternatively, you can use the formula for `p(n)` given in the next section.

If your programming language does not handle big integers easily, you can instead compute the last 6 digits of `p(666)`.

# Sequence formula

If you wish to see this section in video form, it's covered in the Mathologer video starting at 9:35.

The formula for `p(n)` can be recursively defined in terms of smaller values in the sequence. For example,

    p(6) = p(6-1) + p(6-2) - p(6-5)
        = p(5) + p(4) - p(1)
        = 7 + 5 - 1
        = 11

In general:

    p(n) =
        p(n-1) +
        p(n-2) -
        p(n-5) -
        p(n-7) +
        p(n-12) +
        p(n-15) -
        p(n-22) -
        p(n-26) + ...

While the sequence is infinite, `p(n) = 0` when `n < 0`, so you stop when the argument becomes negative. The first two terms of this sequence (`p(n-1)` and `p(n-2)`) are positive, followed by two negative terms (`-p(n-5)` and `-p(n-7)`), and then it repeats back and forth: two positive, two negative, etc.

The numbers that get subtracted from the argument form a second sequence:

    1, 2, 5, 7, 12, 15, 22, 26, 35, 40, 51, 57, 70, ...

This second sequence starts at 1, and the difference between consecutive values in the sequence (2-1, 5-2, 7-5, 12-7, ...) is a third sequence:

    1, 3, 2, 5, 3, 7, 4, 9, 5, 11, 6, 13, 7, ...

This third sequence alternates between the sequence `1, 2, 3, 4, 5, 6, ...` and the sequence `3, 5, 7, 9, 11, 13, ...`. It's easier to see if you write it like this:

    1,    2,    3,    4,    5,     6,     7,
       3,    5,    7,    9,    11,    13,    ...

Okay? So using this third sequence, you can generate the second sequence above, which lets you implement the formula for `p(n)` in terms of smaller `p` values.

# Optional Bonus

How fast can you find the sum of the digits of `p(666666)`.