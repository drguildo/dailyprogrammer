---
title: "[2017-05-15] Challenge #315 [Easy] XOR Multiplication"
url: "https://old.reddit.com/r/dailyprogrammer/comments/6ba9id/20170515_challenge_315_easy_xor_multiplication/"
---

# Description

One way to think about bitwise *addition* (using the symbol `^`) as binary addition without carrying the extra bits:

	   101   5
	^ 1001   9
	  ----  
	  1100  12

	  5^9=12

So let's define XOR multiplcation (we'll use the symbol `@`) in the same way, the addition step doesn't carry:

	     1110  14
	   @ 1101  13
	    -----
	     1110
	       0
	   1110
	^ 1110 
	  ------
	  1000110  70

	  14@13=70

For this challenge you'll get two non-negative integers as input and output or print their XOR-product, using both binary and decimal notation.

# Input Description

You'll be given two integers per line. Example:

	5 9

# Output Description

You should emit the equation showing the XOR multiplcation result:

	5@9=45

**EDIT** I had it as `12` earlier, but that was a copy-paste error. Fixed.

# Challenge Input

	1 2
	9 0
	6 1
	3 3
	2 5
	7 9
	13 11
	5 17
	14 13
	19 1
	63 63

# Challenge Output

	1@2=2
	9@0=0
	6@1=6
	3@3=5
	2@5=10
	7@9=63
	13@11=127
	5@17=85
	14@13=70
	19@1=19
	63@63=1365
