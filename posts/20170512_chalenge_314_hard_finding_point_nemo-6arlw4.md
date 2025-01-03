---
title: "[2017-05-12] Chalenge #314 [Hard] Finding Point Nemo"
url: "https://old.reddit.com/r/dailyprogrammer/comments/6arlw4/20170512_chalenge_314_hard_finding_point_nemo/"
---

# Description

What point on the world's oceans is furthest from any land? On Earth, it's slightly more than 1450 nautical miles from Ducie Island, Motu Nui, and Maher Island. The geographic coordinates of the real [Point Nemo](http://factinator.com/point-nemo/) are: s48:52:31.748 w123:23:33.069. The point was named after Jules Verne’s submarine Captain Nemo, a Latin name that also happens to mean “no one.” 

Your task today is given an ASCII art map, calculate the location of Point Nemo. The map will use ASCII symbols to shade land - mountains, grassland, desert, etc. The blank spaces are ocean. Find the spot in the ocean that is furthest away from any land.

# Input Descripton

You'll be given a two integers on a line telling you how wide (in characters) the map is at its maximum and how many lines to read. Then you'll be given the ASCII art map with the land filled in. Assume the blank space is ocean. The world wraps around, too, just like a real map. Unlike the real world, however, assume this world is a cylinder - it makes the geometry a lot easier. 

# Output Description

Your progam should emit the location of Point Nemo as a grid coordinate in x-y (e.g. 40,25). Count the upper left corner as 0,0. Calculate the Euclidean distance and report the closest whole number position (e.g. round to the nearest x,y coordinate).

# Challenge Input

    80 25
     ## #     # #    #               #      #                       ## ###         
	  ####   ###### ########   ######        ##### ######### #### #######
	   ########## ## #####    ####    #          #####################
	    #######################      ##            ### ##  #### ####  ##
		 ######### #########         ###            ##  #   ### ##   ##
	#	  # #####   #######         ###                      #      #
		  #   ###       ##                          ####### 
		  #    ###                                 ###########     #
		        ###   ##                          ##############              #
	#    		 ###                              ##############                #
				  ##                               #############
				#####                               ###########       ##
			  #########                             ##########      ##
			############                              #########     ##
		  ###############                              #######
		 ##############                                 #####           #########
		############### ##                               ###           ###########
		 ###############                                  #           ############
		  ############                                                ###   ####
		   #########      #                                
    #	      #####
			  
			  ########                        ######               #######
			###################### ###########################  ##############
	##############################################################################
