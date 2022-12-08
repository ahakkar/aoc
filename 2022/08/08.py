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

    def silver(self):
        """
        param : puzzle input as list
        return: none
        """      
    
        x:int = 0
        y:int = 0
        
        total_visible: int = 0
        
        # iterate through every single number and hope it doesn't take forever
        for row in self.__trees:
            for digit in row:
                if (y > 0 and x > 0) and (y < self.__size_y and x < self.__size_x):
                    # [start_row_index:end_row_index, start_column_index:end_column_index]
                    #print(numpy.max( trees[y:y+1, 0:x] )) # trees left from digit
                    #print(numpy.max( trees[y:y+1, x+1:size_x+1] )) # trees right from digit
                    #print(numpy.max( trees[0:y, x:x+1] )) # trees up from digit
                    #print(numpy.max( trees[y+1:size_y+1, x:x+1] )) # trees down from digit     
                    
                    max_trees_around:list = []
                    max_trees_around.append(numpy.max( self.__trees[y:y+1, 0:x] )) # trees left from digit
                    max_trees_around.append(numpy.max( self.__trees[y:y+1, x+1:self.__size_x+1] )) # trees right from digit
                    max_trees_around.append(numpy.max( self.__trees[0:y, x:x+1] )) # trees up from digit
                    max_trees_around.append(numpy.max( self.__trees[y+1:self.__size_y+1, x:x+1] )) # trees down from digit  
                    
                    for tree in max_trees_around:
                        if digit > tree:                        
                            total_visible +=1
                            break
                                                    
                x += 1
            x = 0
            y += 1
        
        # add edge trees to total
        print(total_visible + self.__size_x*2 + self.__size_y*2) # 1854
            
    def gold(self):
        """
        param : puzzle input as list
        return: none
        """
        
        x:int = 0
        y:int = 0
        
        scenic_scores: list = []
        
        # iterate through every single number and hope it doesn't take forever
        for row in self.__trees:
            for digit in row:
                if (y > 0 and x > 0) and (y < self.__size_y and x < self.__size_x):
                    score:int = 0
                    # [start_row_index:end_row_index, start_column_index:end_column_index]
                    #print(self.__trees[y:y+1, 0:x] ) # trees left from digit
                    #print(self.__trees[y:y+1, x+1:self.__size_x+1] ) # trees right from digit
                    #print(self.__trees[0:y, x:x+1] ) # trees up from digit
                    #print(self.__trees[y+1:self.__size_y+1, x:x+1] ) # trees down from digit    
                    
                    # need to flip so we can "Look" from the digit's direction to left
                    score = self.calc_scenic(digit, numpy.flip(self.__trees[y:y+1, 0:x]) ) # trees left from digit
                    #print(score)
                    score *= self.calc_scenic(digit, self.__trees[y:y+1, x+1:self.__size_x+1] ) # trees right from digit
                    #print(score)
                    # need to flip so we can "Look" from the digit's direction to up
                    score *= self.calc_scenic(digit, numpy.flip(self.__trees[0:y, x:x+1]) ) # trees up from digit
                    #print(score)
                    score *= self.calc_scenic(digit, self.__trees[y+1:self.__size_y+1, x:x+1] ) # trees down from digit     
                    #print(score)
                    #print("huoh")
                    
                    scenic_scores.append(score) 
                                                    
                x += 1
            x = 0
            y += 1
        
        print(max(scenic_scores)) # 196 is too low # 2173500 too big
        # 527340 correct
    
    def calc_scenic(self, digit, arr):
        score:int = 0
        #print(arr.flatten().tolist())

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
    sol.silver()
    sol.gold()

    return 0

if __name__ == "__main__":
    main()

#1185 ms so ~120ms per run    
import timeit    
time = timeit.timeit(main, number=10)
print(f"{time*1000:.5f}ms")
