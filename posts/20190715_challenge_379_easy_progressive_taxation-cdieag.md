---
title: "[2019-07-15] Challenge #379 [Easy] Progressive taxation"
url: "https://old.reddit.com/r/dailyprogrammer/comments/cdieag/20190715_challenge_379_easy_progressive_taxation/"
---

# Challenge

The nation of Examplania has the following income tax brackets:

    income cap      marginal tax rate
      ¤10,000           0.00 (0%)
      ¤30,000           0.10 (10%)
     ¤100,000           0.25 (25%)
        --              0.40 (40%)

If you're not familiar with how tax brackets work, see the section below for an explanation.

Given a whole-number income amount up to ¤100,000,000, find the amount of tax owed in Examplania. Round down to a whole number of ¤.

# Examples

    tax(0) => 0
    tax(10000) => 0
    tax(10009) => 0
    tax(10010) => 1
    tax(12000) => 200
    tax(56789) => 8697
    tax(1234567) => 473326

# Optional improvement

One way to improve your code is to make it easy to swap out different tax brackets, for instance by having the table in an input file. If you do this, you may assume that both the income caps and marginal tax rates are in increasing order, the highest bracket has no income cap, and all tax rates are whole numbers of percent (no more than two decimal places).

However, because this is an Easy challenge, this part is optional, and you may hard code the tax brackets if you wish.

# How tax brackets work

A tax bracket is a range of income based on the income caps, and each tax bracket has a corresponding marginal tax rate, which applies to income within the bracket. In our example, the tax bracket for the range ¤10,000 to ¤30,000 has a marginal tax rate of 10%. Here's what that means for each bracket:

* If your income is less than ¤10,000, you owe 0 income tax.
* If your income is between ¤10,000 and ¤30,000, you owe 10% income tax on the income that exceeds ¤10,000. For instance, if your income is ¤18,000, then your income in the 10% bracket is ¤8,000. So your income tax is 10% of ¤8,000, or ¤800.
* If your income is between ¤30,000 and ¤100,000, then you owe 10% of your income between ¤10,000 and ¤30,000, plus 25% of your income over ¤30,000.
* And finally, if your income is over ¤100,000, then you owe 10% of your income from ¤10,000 to ¤30,000, plus 25% of your income from ¤30,000 to ¤100,000, plus 40% of your income above ¤100,000.

One aspect of progressive taxation is that increasing your income will never decrease the amount of tax that you owe, or your overall tax rate (except for rounding).

# Optional bonus

The overall tax rate is simply the total tax divided by the total income. For example, an income of ¤256,250 has an overall tax of ¤82,000, which is an overall tax rate of exactly 32%:

    82000 = 0.00 × 10000 + 0.10 × 20000 + 0.25 × 70000 + 0.40 × 156250
    82000 = 0.32 × 256250

Given a target overall tax rate, find the income amount that would be taxed at that overall rate in Examplania:

    overall(0.00) => 0 (or anything up to 10000)
    overall(0.06) => 25000
    overall(0.09) => 34375
    overall(0.32) => 256250
    overall(0.40) => NaN (or anything to signify that no such income value exists)

You may get somewhat different answers because of rounding, but as long as it's close that's fine.

The simplest possibility is just to iterate and check the overall tax rate for each possible income. That works fine, but if you want a performance boost, check out binary search. You can also use algebra to reduce the number of calculations needed; just make it so that your code still gives correct answers if you swap out a different set of tax brackets.