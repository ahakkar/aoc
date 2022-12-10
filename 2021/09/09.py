# -*- encoding: utf-8 -*-
'''
@File    :   09.py
@Time    :   10/12/2022, 15:44:35
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2022
@Desc    :   TEMP
'''

RISK_LEVEL:int = 1

def read_file(filename:str) -> list:
    """
    param : str, filename to read
    return: list, lines
    """
    
    data:list = []
    
    try:
        with open (filename, "r") as read_file:
            data = read_file.read().splitlines()        
    except FileNotFoundError:
        print(f"Bad file name! {filename}")
        exit()
    except Exception:
        print("SOS")
        exit()
        
    return data;

class Solution:
    def __init__(self, data:list):
        self.__data:list = []
        self.__low_points:list = [] # tuples with y,x coords
        self.__risk_levels:int = 0
        self.__size_of_basins:list = []
        self.__basin_coords:set = set()
        
        # massage input data to 2d grid
        for row in data:
            self.__data.append(list(map(int, row)))
        
        # find low points
        for y, row in enumerate(self.__data):
            for x, height in enumerate(row):
                self.find_low_points(height, x, y)
                           
        # do a recurcisve depth-first search of basins     
        for lp in self.__low_points:            
            self.depth_first_search(lp[0], lp[1])
            self.__size_of_basins.append(len(self.__basin_coords))
            self.__basin_coords = set()            
        
        # sort items by descending order so we can choose 3 largest
        self.__size_of_basins.sort(reverse=True)
        from functools import reduce
        from operator import mul
        
        # print results:
        print("silver:", self.__risk_levels)
        print("gold", reduce(mul, self.__size_of_basins[0:3]))
            
    def is_index_valid(self, list, index):
        if -1 < index < len(list): 
            return True
        return False
    
    def depth_first_search(self, y:int, x:int):              
        if (self.__data[y][x] == 9):
            return  
        
        self.__data[y][x] = 9
        self.__basin_coords.add((y, x))
        if self.is_index_valid(self.__data[y], x+1):
            self.depth_first_search(y, x+1)

        if self.is_index_valid(self.__data[y], x-1):
            self.depth_first_search(y, x-1)           

        if self.is_index_valid(self.__data, y+1):
            self.depth_first_search(y+1, x)

        if self.is_index_valid(self.__data, y-1):
            self.depth_first_search(y-1, x)        
        
    def find_low_points(self, height:str, x:int, y:int) -> None:
        """Find the low points - the locations that are 
        lower than any of its adjacent locations from a 2D grid.
        """

        sur:list = []

        if self.is_index_valid(self.__data[y], x+1):
            sur.append(self.__data[y][x+1])

        if self.is_index_valid(self.__data[y], x-1):
            sur.append(self.__data[y][x-1])            

        if self.is_index_valid(self.__data, y+1):
            sur.append(self.__data[y+1][x])

        if self.is_index_valid(self.__data, y-1):
            sur.append(self.__data[y-1][x])

        if len(sur) > 0 and height < min(sur):
            self.__low_points.append((y, x))
            self.__risk_levels += RISK_LEVEL + height        

            #print(height, end= " ")
            #print(sur)
        
def main():
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2021\\09\\puzzle.input") 
    sol = Solution(data) # 195 too low, 417 too low, 425 correct

    return 0

main()