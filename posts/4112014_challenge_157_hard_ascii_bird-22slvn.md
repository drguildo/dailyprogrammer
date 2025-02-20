---
title: "[4/11/2014] Challenge #157 [Hard] ASCII Bird"
url: "https://old.reddit.com/r/dailyprogrammer/comments/22slvn/4112014_challenge_157_hard_ascii_bird/"
---

#Description:

In the news lately there has been a lot of press about a game called Flappy Bird. I have noticed many people have rushed to make clones of this game. 


For those who want to know more about the game [Click here for wikipedia] (http://en.wikipedia.org/wiki/Flappy_Bird)


So I thought we need to join in on the craze and come up with our own version of Flappy Bird. ASCII Bird. It is flappy bird with ASCII.


More or less you control a bird flying through randomly generated obstacles scrolling right to left at you. You decide when the bird flaps to gain height and if you don't do anything he will fall. If he falls to the ground or hits an obstacle the game is over. For every obstacle he flys over or under with success he gains a point. 




#Input: 

We will take a single input from the player of the game. A number between 0-4. This represents the "flap" for our bird. The value would represent how high we like our bird to move. 


#Output:

This is mostly a visual challenge. After we get the input we have to show the map. 


* @ = our bird
* . = empty space
* # = obstacle.

The board will be 10 rows high by 20 columns.

##example:

	..........#.......#.
	..........#.......#.
	..........#.........
	..........#.........
	.@........#.........
	....................
	......#.............
	......#........#....
	......#........#....
	......#........#....

	(score 0) 0-4?


After you enter a number the forward velocity of the bird will be 2 columns. In those 2 columns you must move the bird based on the velocity. If you typed 1-4 then the board shifts over 2 columns and the bird will go up that many (if it wants to go above the top row it will not) 

If you type a 0 instead our bird will decay his flight by 2 rows down. 


If flappy bird flys over or under an obstacle he will advance his score by 1 point. If he goes below the bottom row on  a decay or makes contact with a obstacle he will die and the game is over (display the final score - maybe ask to play again)


The board is updated 2 columns at a time. You have to keep track of it. Randomly every 7-10 columns on either top or bottom you will generate an obstacle that is 2-4 in height hanging from the top or coming up from the bottom. Once you spawn an obstacle the next will spawn 7-10 columns away. (note each top and bottom needs to be tracked separate and are not related. This can create for some interesting maps)


##example after typing a 2 for our move with above then 2 moves of a 0

    ........#.......#...
	........#.......#...
	.@......#...........
	........#...........
	........#...........
	....................
	....#...............
	....#........#......
	....#........#......
	....#........#......

	(score 0) 0-4?
	
	......#.......#...
	......#.......#...
	......#...........
	......#...........
	.@....#...........
	..................
	..#...............
	..#........#......
	..#........#......
	..#........#......

	(score 0) 0-4?
	
	
	....#.......#.....
	....#.......#.....
	....#.............
	....#.............
	....#.............
	..................
	#@...............#
	#........#.......#
	#........#.......#
	#........#.......#

	(score 1) 0-4?

Our bird spawns in the middle of the rows in height and as above should have 1 column behind him. He will pretty much just move up or down in that column as the board "shifts" its display right to left and generating the obstacles as needed.



#Notes:

As always if you got questions/concerns post away and we can tackle it.


#Extra Challenge:

Make it graphical and go from ASCII Bird to Flappy Bird.