Title: [2015-05-08] Challenge #213 [Hard] Stepstring discrepancy

# Description

Define the _discrepancy_ of a string of any two symbols (I'll use `a` and `b`) to be the absolute difference between the counts of each of the two symbols in the string. For example, all of the following strings have a discrepancy of 3: 

    aaa 
    bbb 
    abbbb 
    aababaa 
    baababbababababababbbaabbaaaabaaabbaa 

Define a _stepstring_ of a string to be any string that can be formed by starting at any position x in the string, stepping through the string n characters at a time, and ending before any position y. In python, this is any string that can be formed using slice notation `s[x:y:n]`. For example, some stepstrings of the string `"abcdefghij"` are: 

    d
    defg
    acegi
    bdfhj
    dfh
    beh
    ai
    abcdefghij

Your problem is, given a string of up to 10,000 characters, find the largest discrepancy of any stepstring of the string. For instance, this string:

    bbaaabababbaabbaaaabbbababbaabbbaabbaaaaabbababaaaabaabbbaaa 

has this string as a stepstring (corresponding to the python slice notation `s[4:56:4]`): 

    aaaabaaaaabaa 

which has a discrepancy of 9. Furthermore, no stepstring has a discrepancy greater than 9. So the correct solution for this string is 9. 

# Input Description

A series of strings (one per line) consisting of `a` and `b` characters. 

# Output Description

For each string in the input, output the largest discrepancy of any stepstring of the string. (Optionally, also give the slice notation values corresponding to the stepstring with the largest discrepancy.) 

# Sample Input

    bbaaabababbaabbaaaabbbababbaabbbaabbaaaaabbababaaaabaabbbaaa
    bbaaaababbbaababbbbabbabababababaaababbbbbbaabbaababaaaabaaa
    aaaababbabbaabbaabbbbbbabbbaaabbaabaabaabbbaabababbabbbbaabb
    abbabbbbbababaabaaababbbbaababbabbbabbbbaabbabbaaabbaabbbbbb

# Sample Output

    9
    12
    11
    15

# Challenge Input:

[Download the challenge input here](http://pastebin.com/raw.php?i=Xt3BV8nK): 8 lines of 10,000 characters each. 

# Challenge Output 

    113
    117
    121
    127
    136
    136
    138
    224

#Note

This problem was inspired by [a recent mathematical discovery](http://www.newscientist.com/article/dn25068-wikipediasize-maths-proof-too-big-for-humans-to-check.html#.Uwa72lK9jAR): the longest string for which your program should output 2 is 1,160 characters. Every string of 1,161 characters will yield a result of 3 or more. The proof of this fact was generated by a computer and is 13 gigabytes long!

# Credit

This challenge was submitted by /u/Cosmologicon. If you have an idea for a challenge, please share it in /r/dailyprogrammer_ideas. 