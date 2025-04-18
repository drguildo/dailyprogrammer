---
title: "[2016-07-22] Challenge #276 [Hard] ∞ Loop solver part 2"
url: "https://old.reddit.com/r/dailyprogrammer/comments/4u3e96/20160722_challenge_276_hard_loop_solver_part_2/"
---

This is the same challenge as /u/jnazario's excellent [∞ Loop solver](https://www.reddit.com/r/dailyprogrammer/comments/4rug59/20160708_challenge_274_hard_loop_solver/) but for larger inputs.

The input format is different, as you will be given a presolved partial grid, where each cell is the possible rotations that line up with a possible rotation of neighbour cells.

The challenge is to find ALL of the valid grid solutions

# 20x20 input visualization

    ┌─┬─────┬────┬───────┬────┬───┬───┬────┬─────┬────────┬────┬────────┬────┬─────┬──┬──┬──┬──┬──┬──┐
    │6│12   │6   │10     │10  │12 │6  │12  │6    │12      │6   │14      │12  │6    │10│10│10│14│14│12│
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │7│13   │3   │14     │12  │3  │9  │7   │15   │9       │5   │7       │11  │9    │6 │12│6 │13│5 │5 │
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │7│9    │6   │9      │7   │10 │10 │9   │7    │10      │13  │7       │10  │10   │9 │5 │5 │5 │3 │9 │
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │5│6    │15  │12     │5   │6  │14 │14  │15   │12      │5   │3       │10  │14   │10│11│11│15│10│12│
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │7│13   │3   │9      │3   │15 │11 │13  │7    │9       │7   │12      │6   │11   │10│10│10│9 │6 │9 │
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │7│11   │14  │14     │14  │9  │6  │15  │15   │12      │5   │3       │15  │14   │14│12│6 │12│3 │12│
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │5│6    │9   │3      │9   │6  │9  │5   │7    │13      │5   │6       │15  │15   │15│13│7 │13│6 │13│
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │5│5    │6   │10     │10  │13 │6  │15  │15   │11 13   │13 7│7 13 11 │11 7│11   │15│11│9 │3 │15│9 │
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │7│9    │5   │6      │10  │11 │9  │7   │9    │6 3     │11  │11 13 14│14 7│10   │11│14│12│6 │15│12│
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │5│6    │9   │3      │12  │6  │10 │9   │6    │13 11 14│6 12│14 7    │9   │6    │10│9 │7 │9 │5 │5 │
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │7│11   │14  │10     │9   │7  │10 │14  │13 11│7 14    │11  │11      │10  │13   │6 │14│9 │6 │13│5 │
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │7│12   │7   │12     │6   │13 │6  │9   │3 6  │13      │6   │10      │12  │7    │11│11│14│15│13│5 │
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │7│13 11│3 9 │11 13 7│13 7│3 9│9 3│6 12│14 7 │15      │11  │10      │9   │3    │14│10│9 │3 │9 │5 │
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │7│13 14│6 12│14 7   │11  │12 │6  │13  │5    │3       │14  │12      │6   │12   │5 │6 │14│14│12│5 │
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │5│3    │15  │11     │12  │7  │9  │7   │11   │12      │5   │7       │9   │7    │15│11│13│7 │13│5 │
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │5│6    │9   │6      │11  │13 │6  │13  │6    │15      │9   │7       │10  │13   │3 │10│9 │3 │15│13│
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │3│13   │6   │15     │12  │7  │15 │9   │3    │13      │6   │13 11   │6 12│11 7 │14│10│12│6 │15│9 │
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │6│13   │3   │11     │15  │15 │13 │6   │10   │15      │11  │11 14   │11  │14 11│13│6 │15│9 │3 │12│
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │7│11   │12  │6      │15  │9  │5  │7   │14   │9       │6   │14 13   │12 6│7 14 │9 │5 │7 │12│6 │13│
    ├─┼─────┼────┼───────┼────┼───┼───┼────┼─────┼────────┼────┼────────┼────┼─────┼──┼──┼──┼──┼──┼──┤
    │3│10   │9   │3      │11  │10 │11 │11  │11   │10      │9   │3       │11  │11   │10│11│11│9 │3 │9 │
    └─┴─────┴────┴───────┴────┴───┴───┴────┴─────┴────────┴────┴────────┴────┴─────┴──┴──┴──┴──┴──┴──┘

1. The numbers in each cell are indexes (0-based) into the looper tiles `╹╺┗╻┃┏┣╸┛━┻┓┫┳╋` (leading index 0 is space)

2. The 4 digit binary representation of each index indicates whether there is a tick that points `WSEN`

3. Cells with a single index are forced moves.  Cells with multiple indexes are potential moves.

4. The general strategy for finding all valid final (ones with single indexes per cell) grids is to repeatedly split the grid based on one multiple cell (where each grid has a unique index in that cell), and then find all forced moves in each independent grid.

5. A forced move by row is one where the left cells' East tick is equal to the right cell's West tick.  By column, the top cell's South tick is equal to the lower cell's North tick.

**input**  (each row separated by LF, each cell by comma, each candidate by space)

    20x20
    6,12,6,10,10,12,6,12,6,12,6,14,12,6,10,10,10,14,14,12                
    7,13,3,14,12,3,9,7,15,9,5,7,11,9,6,12,6,13,5,5                       
    7,9,6,9,7,10,10,9,7,10,13,7,10,10,9,5,5,5,3,9                        
    5,6,15,12,5,6,14,14,15,12,5,3,10,14,10,11,11,15,10,12                
    7,13,3,9,3,15,11,13,7,9,7,12,6,11,10,10,10,9,6,9                     
    7,11,14,14,14,9,6,15,15,12,5,3,15,14,14,12,6,12,3,12                 
    5,6,9,3,9,6,9,5,7,13,5,6,15,15,15,13,7,13,6,13                       
    5,5,6,10,10,13,6,15,15,11 13,13 7,7 13 11,11 7,11,15,11,9,3,15,9     
    7,9,5,6,10,11,9,7,9,6 3,11,11 13 14,14 7,10,11,14,12,6,15,12         
    5,6,9,3,12,6,10,9,6,13 11 14,6 12,14 7,9,6,10,9,7,9,5,5              
    7,11,14,10,9,7,10,14,13 11,7 14,11,11,10,13,6,14,9,6,13,5            
    7,12,7,12,6,13,6,9,3 6,13,6,10,12,7,11,11,14,15,13,5                 
    7,13 11,3 9,11 13 7,13 7,3 9,9 3,6 12,14 7,15,11,10,9,3,14,10,9,3,9,5
    7,13 14,6 12,14 7,11,12,6,13,5,3,14,12,6,12,5,6,14,14,12,5           
    5,3,15,11,12,7,9,7,11,12,5,7,9,7,15,11,13,7,13,5                     
    5,6,9,6,11,13,6,13,6,15,9,7,10,13,3,10,9,3,15,13                     
    3,13,6,15,12,7,15,9,3,13,6,13 11,6 12,11 7,14,10,12,6,15,9           
    6,13,3,11,15,15,13,6,10,15,11,11 14,11,14 11,13,6,15,9,3,12          
    7,11,12,6,15,9,5,7,14,9,6,14 13,12 6,7 14,9,5,7,12,6,13              
    3,10,9,3,11,10,11,11,11,10,9,3,11,11,10,11,11,9,3,9                  
    

**output**

to save space just provide the number of distinct valid grids.  (I get 12)

# 30x30 challenges

thanks to /u/bearific for creating a generator for this challenge.  The above and larger inputs are available here:    
https://gist.github.com/FrankRuis/0aa761b9562a32ea7fdcff32f1768eb0

"reduced input" (above) formats of the 30x30 challenges:  (you may use the original input format and solve these anyway you like)

**first input**

    6,10,14,12,6,14,10,12,6,12,6,14,10,12,6,10,14,14,14,12,6,14,12,6,10,14,10,12,6,12                          
    3,14,13,7,13,3,14,15,15,15,11,13,6,9,5,6,11,9,5,3,15,15,13,5,6,15,10,15,13,5                               
    6,11,15,15,15,10,13 11,7 11,15,9,6,9,7,12,3,13,6,10,11,14,11,15,15,11,15,9,6,13,7,13                       
    7,12,3,13,5,6,13 14,7 14,11,14,9,6,15,11,14,15,11,10,12,7,12,7,13 11,6 12,13 7,6 12,11 7,13,5,5            
    7,11,14,9,3,9,7,13,6,15,14,13,5,6,9,3,10,10,13,3,15,13,3 6,15,15,11 13,14 7,13,5,5                         
    7,14,13,6,14,12,5,7,15,15,9,3,9,5,6,12,6,14,9,6,15,13 11,6 12 3 9,9 3,7 13,14 7,15,13,7,9                  
    7,15,13,5,5,3,9,5,5,5,6,14,14,9,3,11,11,13,6,15,15,13 14,7 11 14,14,15,15,15,11,11,12                      
    7,11,9,5,7,12,6,13,3,13,3,13,3,10,10,10,14,11 7,9,5,3,11,13 14,7 11,15,11,11,10,12,5                       
    5,6,10,9,5,5,5,7,12,5,6,9,6,12,6,14,13 11,6 12 3 9,12 6,7 13,12 6,6 12,9 3,7 14,15,10,12,6,15,13           
    7,9,6,12,3,9,5,5,3,9,3,14,11 13,11 13 7,11 13 7,13 11 7,5 10,7 13 14,11 7,13,5,7,14,11,9,6,11,13,3,13      
    5,6,11,9,6,12,3,13,6,14,14,13,6,10,14 11,11,11 14,13,6 3,15,11,15,9,6,12,7,10,9,6,13                       
    3,11,10,10,9,3,10,15,9,7,15,13,5,6,13 14,6 12,14 13 7,9 3,7 13 14,11 7,14,9,6,11,13,3,12,6,11,9            
    6,10,14,10,10,12,6,15,10,11 13,13 7,7 11,13,5,5,3,11,14,13,6 3,15,12,5,6,15,12,5,7,14,12                   
    5,6,9,6,14,11,15,15,10,12 9,7,11 14,13,7,13,6,12,5,3,11 14,9,5,5,7,15,15,11,15,15,9                        
    3,15,14,15,13,6,9,5,6,13 11 14,3 9,14 13 7,13 7,3 9,11 7,11,15,9,6,14 11,10,11,15,11 13,11 7,13,6,11,11,12 
    6,15,15,15,11,15,14,11 13,13 7,7 14,12,3,15,10,12 9,6,9,6,11 13,11 7 14,10,12,3,14 13,14 7,15,11,12,6,13   
    3,9,7,9,6,13 11,7 13 11,14 11 7,15,11,15,12,5,6,13 14,3 9,12 6,3 9,10 5,14 11 7,10,15,14,13,5,3,10,15,11,13
    6,14,11,14,15,13 11 14,7 13 11 14,11 13 7 14,11 7,10,9,3,11,13,5,6,13,6,12 9,3 6,10,13,3,13,5,6,12,3,10,9  
    3,13,6,13,3,13 14,7 13 14,14 13 11 7,14 11 7,14,12,6,10,9,5,5,3,15,11 13 14,14 7,10,13,6,15,11,13,5,6,10,12
    6,15,9,3,14,9,7,13 14,7 14,15,11,11,10,12,3,15,12,3,14 13,9 3,6 12,11 7,13,7,10,11,11,15,12,5              
    7,13,6,10,15,14,9,5,3,11,14,12,6,9,6,9,3,14,9,6,15,12 9,5,7,10,10,12,3,13,5                                
    7,9,7,14,11,11,12,5,6,10,11,13,7,12,5,6,12,7,10,11,13 11,3 9 6 12,13 11 7,7 11,14,14,11,12,5,5             
    5,6,13,7,12,6,13,5,3,14,14,13,3,15,11,11,11,13,6,12,7 13 14,10 5,11 7 14,9 12,3,15,14,11,11,13             
    7,9,7,9,5,7,11,15,14,13,5,7,12,3,10,14,12,3,13 11,3 9,9 3,6 12 3 9,14 13 7,14 7,10,11,15,14,12,5           
    7,12,3,10,11,15,14,11,9,3,9,3,15,12,6,13,3,10,13 14,6 12,12 6,7 14,11,15,14,12,3,13,5,5                    
    3,9,6,10,12,7,9,6,14,10,12,6,13,7,15,15,12,6,9,7,15,11,12,3,13,3,12,3,9,5                                  
    6,12,7,14,9,7,14,9,7,12,3,9,3,15,11 13,11 7,9,5,6,15,15,14,15,12,3,14,13,6,14,9                            
    7,15,13,7,10,11,11,10,13,5,6,10,14,13,6 3,14 11,10,9,5,5,3,13,5,5,6,15,11,15,15,12                         
    7,9,3,13,6,14,12,6,15,11,11,10,11,11,13 14,7 14,14,12,7,15,12,7,15,13,3,13,6,11,15,13                      
    3,10,10,9,3,9,3,9,3,10,10,10,10,10,9,3,9,3,9,3,11,9,3,11,10,9,3,10,11,9                                    

**input 2** 

    6,10,14,12,6,14,10,12,6,12,6,14,10,12,6,10,14,14,14,12,6,14,12,6,10,14,10,12,6,12                          
    3,14,13,7,13,3,14,15,15,15,11,13,6,9,5,6,11,9,5,3,15,15,13,5,6,15,10,15,13,5                               
    6,11,15,15,15,10,13 11,7 11,15,9,6,9,7,12,3,13,6,10,11,14,11,15,15,11,15,9,6,13,7,13                       
    7,12,3,13,5,6,13 14,7 14,11,14,9,6,15,11,14,15,11,10,12,7,12,7,13 11,6 12,13 7,6 12,11 7,13,5,5            
    7,11,14,9,3,9,7,13,6,15,14,13,5,6,9,3,10,10,13,3,15,13,3 6,15,15,13 11,14 7,13,5,5                         
    7,14,13,6,14,12,5,7,15,15,9,3,9,5,6,12,6,14,9,6,15,11 13,12 6 9 3,3 9,13 7,7 14,15,13,7,9                  
    7,15,13,5,5,3,9,5,5,5,6,14,14,9,3,11,11,13,6,15,15,14 13,11 7 14,14,15,15,15,11,11,12                      
    7,11,9,5,7,12,6,13,3,13,3,13,3,10,10,10,14,11 7,9,5,3,11,14 13,11 7,15,11,11,10,12,5                       
    5,6,10,9,5,5,5,7,12,5,6,9,6,12,6,14,13 11,6 12 3 9,12 6,7 13,12 6,6 12,9 3,14 7,15,10,12,6,15,13           
    7,9,6,12,3,9,5,5,3,9,3,14,13 11,7 13 11,13 11 7,7 13 11,5 10,13 7 14,7 11,13,5,7,14,11,9,6,11,13,3,13      
    5,6,11,9,6,12,3,13,6,14,14,13,6,10,11 14,11,11 14,13,6 3,15,11,15,9,6,12,7,10,9,6,13                       
    3,11,10,10,9,3,10,15,9,7,15,13,5,6,14 13,12 6,14 7 13,9 3,7 13 14,11 7,14,9,6,11,13,3,12,6,11,9            
    6,10,14,10,10,12,6,15,10,13 11,7 13,11 7,13,5,5,3,11,14,13,6 3,15,12,5,6,15,12,5,7,14,12                   
    5,6,9,6,14,11,15,15,10,9 12,7,14 11,13,7,13,6,12,5,3,11 14,9,5,5,7,15,15,11,15,15,9                        
    3,15,14,15,13,6,9,5,6,14 13 11,9 3,7 13 14,13 7,3 9,11 7,11,15,9,6,14 11,10,11,15,13 11,7 11,13,6,11,11,12 
    6,15,15,15,11,15,14,13 11,7 13,7 14,12,3,15,10,12 9,6,9,6,13 11,7 11 14,10,12,3,13 14,7 14,15,11,12,6,13   
    3,9,7,9,6,13 11,7 13 11,11 7 14,15,11,15,12,5,6,13 14,9 3,6 12,9 3,5 10,7 11 14,10,15,14,13,5,3,10,15,11,13
    6,14,11,14,15,13 11 14,7 13 11 14,14 13 11 7,11 7,10,9,3,11,13,5,6,13,6,9 12,3 6,10,13,3,13,5,6,12,3,10,9  
    3,13,6,13,3,13 14,7 13 14,13 11 7 14,14 7 11,14,12,6,10,9,5,5,3,15,14 13 11,14 7,10,13,6,15,11,13,5,6,10,12
    6,15,9,3,14,9,7,13 14,7 14,15,11,11,10,12,3,15,12,3,13 14,3 9,12 6,7 11,13,7,10,11,11,15,12,5              
    7,13,6,10,15,14,9,5,3,11,14,12,6,9,6,9,3,14,9,6,15,9 12,5,7,10,10,12,3,13,5                                
    7,9,7,14,11,11,12,5,6,10,11,13,7,12,5,6,12,7,10,11,11 13,12 6 9 3,7 13 11,11 7,14,14,11,12,5,5             
    5,6,13,7,12,6,13,5,3,14,14,13,3,15,11,11,11,13,6,12,14 13 7,5 10,11 7 14,12 9,3,15,14,11,11,13             
    7,9,7,9,5,7,11,15,14,13,5,7,12,3,10,14,12,3,13 11,3 9,9 3,3 9 6 12,14 13 7,7 14,10,11,15,14,12,5           
    7,12,3,10,11,15,14,11,9,3,9,3,15,12,6,13,3,10,13 14,6 12,12 6,14 7,11,15,14,12,3,13,5,5                    
    3,9,6,10,12,7,9,6,14,10,12,6,13,7,15,15,12,6,9,7,15,11,12,3,13,3,12,3,9,5                                  
    6,12,7,14,9,7,14,9,7,12,3,9,3,15,13 11,7 11,9,5,6,15,15,14,15,12,3,14,13,6,14,9                            
    7,15,13,7,10,11,11,10,13,5,6,10,14,13,3 6,11 14,10,9,5,5,3,13,5,5,6,15,11,15,15,12                         
    7,9,3,13,6,14,12,6,15,11,11,10,11,11,14 13,14 7,14,12,7,15,12,7,15,13,3,13,6,11,15,13                      
    3,10,10,9,3,9,3,9,3,10,10,10,10,10,9,3,9,3,9,3,11,9,3,11,10,9,3,10,11,9          