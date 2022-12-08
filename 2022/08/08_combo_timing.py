# -*- encoding: utf-8 -*-
'''
@File    :   08.py
@Time    :   08/12/2022, 09:05:14
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2022
@Desc    :   This is sooo innefficient.. 200 x 100 x 100 = 2 million checks :D
'''

import numpy, time

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
        
        time_slices = 0
        time_silver = 0
        time_gold = 0
        
        total_visible: int = 0      # for silver
        max_scenic_score: int = 0   # for gold
        
        # iterate through every single number and hope it doesn't take forever
        for row in self.__trees:
            for digit in row:
                if (y > 0 and x > 0) and (y < self.__size_y and x < self.__size_x):                    
                    max_trees_around:list = []
                    t0 = time.time()
                    # how to slice numpy 2D arrays:   
                    # [start_row_index:end_row_index, start_column_index:end_column_index]  
                    lf_slice = self.__trees[y:y+1, 0:x]                     #left
                    rg_slice = self.__trees[y:y+1, x+1:self.__size_x+1]     #right       
                    up_slice = self.__trees[0:y, x:x+1]                     #up
                    dn_slice = self.__trees[y+1:self.__size_y+1, x:x+1]     #down
                    
                    t1 = time.time()
                    time_slices += t1-t0
                    
                    t2 = time.time()
                    # SILVER - python's max() is quite a bit faster than any numpy's (a)max()
                    max_trees_around.append(max(lf_slice.flatten().tolist())) 
                    max_trees_around.append(max(rg_slice.flatten().tolist())) 
                    max_trees_around.append(max(up_slice.flatten().tolist())) 
                    max_trees_around.append(max(dn_slice.flatten().tolist())) 
                    
                    for tree in max_trees_around:
                        if digit > tree:                        
                            total_visible +=1
                            break
                        
                    t3 = time.time()
                    time_silver += t3-t2
                    
                    t4 = time.time()
                    # GOLD
                    # need to flip left and up rows so we can "look" from tree digit to right direction
                    score = self.calc_scenic(digit, numpy.flip(lf_slice) ) 
                    score *= self.calc_scenic(digit, rg_slice ) 
                    score *= self.calc_scenic(digit, numpy.flip(up_slice) )  
                    score *= self.calc_scenic(digit, dn_slice )    
                    
                    # if score is bigger than current max, update it
                    if score > max_scenic_score: max_scenic_score = score    
                    
                    t5 = time.time()
                    time_gold += t5-t4 
                                                    
                x += 1
            # reset column index and switch to new row index
            x = 0
            y += 1
            
        print(f"time to do slices: {time_slices*1000:.5f}ms")
        print(f"time for silver: {time_silver*1000:.5f}ms")
        print(f"time for gold: {time_gold*1000:.5f}ms")
        print("time total:", f"{(time_slices+time_silver+time_gold)*1000:.5f}ms")
        
        # add edge trees to total
        print("silver:", total_visible + self.__size_x*2 + self.__size_y*2) # 1854
        print("gold:", max_scenic_score) # 196 is too low, 2173500 too big, 527340 correct       
    
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

#if __name__ == "__main__":
#    main()

#1185 ms so ~120ms per run    
import timeit    
time = timeit.timeit(main, number=1)
print(f"{time*1000:.5f}ms")