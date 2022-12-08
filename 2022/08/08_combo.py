# -*- encoding: utf-8 -*-
'''
@File    :   08.py
@Time    :   08/12/2022, 09:05:14
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2022
@Desc    :   This is sooo innefficient.. 200 x 100 x 100 = 2 million checks :D
'''

import numpy

def read_file(filename:str) -> list:
    """
    param : str, filename to read
    return: list, lines
    """
    
    data:list = []
    
    try:
        with open (filename, "r") as read_file:
            data: list = read_file.read().splitlines()
        read_file.close()
    except FileNotFoundError:
        print(f"Bad file name! {filename}")
        exit()
    except:
        print("SOS")
        exit()
        
    return data;

class Solution:
    def __init__(self, data:list):
        self.__trees = self.massage(data)
        # array x y size
        self.__size_x:int = self.__trees.shape[0]-1
        self.__size_y:int = self.__trees.shape[1]-1
        
    # massage data to separate digits
    def massage(self, data:list):
        temp: list = []
            
        for row in data:
            items = map(int, row)
            items = list(items)
            temp.append(items)
        
        # add digits to 2D array
        return numpy.array(temp)                

    # combining methods does not make this any faster at all
    def combo(self): 
        # current col and row
        x:int = 0
        y:int = 0
        
        total_visible: int = 0      # for silver
        scenic_scores: list = []    # for gold
        
        # iterate through every single number and hope it doesn't take forever
        for row in self.__trees:
            for digit in row:
                if (y > 0 and x > 0) and (y < self.__size_y and x < self.__size_x):                    
                    max_trees_around:list = []
                    
                    # how to slice numpy 2D arrays:   
                    # [start_row_index:end_row_index, start_column_index:end_column_index]  
                    lf_slice = self.__trees[y:y+1, 0:x]                     #left
                    rg_slice = self.__trees[y:y+1, x+1:self.__size_x+1]     #right       
                    up_slice = self.__trees[0:y, x:x+1]                     #up
                    dn_slice = self.__trees[y+1:self.__size_y+1, x:x+1]     #down
                    
                    # SILVER
                    max_trees_around.append(numpy.max( lf_slice )) 
                    max_trees_around.append(numpy.max( rg_slice )) 
                    max_trees_around.append(numpy.max( up_slice )) 
                    max_trees_around.append(numpy.max( dn_slice ))
                    
                    for tree in max_trees_around:
                        if digit > tree:                        
                            total_visible +=1
                            break
                    
                    # GOLD
                    # need to flip left and up rows so we can "look" from tree digit to right direction
                    score = self.calc_scenic(digit, numpy.flip(lf_slice) ) 
                    score *= self.calc_scenic(digit, rg_slice ) 
                    score *= self.calc_scenic(digit, numpy.flip(up_slice) )  
                    score *= self.calc_scenic(digit, dn_slice )    

                    scenic_scores.append(score)      
                                                    
                x += 1
            # reset column index and switch to new row index
            x = 0
            y += 1
            
        # add edge trees to total
        print("silver:", total_visible + self.__size_x*2 + self.__size_y*2) # 1854
        print("gold:", max(scenic_scores)) # 196 is too low, 2173500 too big, 527340 correct       
    
    # calculates the scenic value to one direction
    def calc_scenic(self, digit, arr):
        score:int = 0

        # flatten a 2d numpy array and convert it to a python list, so it's easier to iterate
        for num in arr.flatten().tolist():      
            if num >= digit:
                score += 1
                break
            if num < digit:
                score += 1
            
        return score

def main():
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\08\\puzzle.input")  
    sol = Solution(data)
    sol.combo()

    return 0

if __name__ == "__main__":
    main()

#1185 ms so ~120ms per run    
#import timeit    
#time = timeit.timeit(main, number=10)
#print(f"{time*1000:.5f}ms")