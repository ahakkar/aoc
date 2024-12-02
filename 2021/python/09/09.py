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
        
        # massage input data to 2d grid with one digit in each index
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
        print("silver:", self.__risk_levels) # 425
        print("gold", reduce(mul, self.__size_of_basins[0:3])) #1135260
            
    def is_index_valid(self, list, index):
        
        if -1 < index < len(list): 
            return True
        return False
    
    def depth_first_search(self, y:int, x:int): 
        """Recursively flood the are around provided point. Initially a
        lowest point found with find_low_points is provided. Method then 
        marks the point as visited with setting the value to 9
        visited/impassable). Collects the value to __basin_coords set, so
        we can figure out later how many points a single basin has.
        
        - Checks every surrounding point (left, right, up, down) recursively.
        - Returns when the current cell is 9.
        - Recursion is finished when all cells in a basin are marked as 9.        
        """             
        if self.__data[y][x] == 9:
            return
    
        self.__data[y][x] = 9
        self.__basin_coords.add((y, x))

        for i, j in [(0,1), (0,-1), (1,0), (-1,0)]:
            if self.is_index_valid(self.__data, y+i) and self.is_index_valid(self.__data[y+i], x+j):
                   
                self.depth_first_search(y+i, x+j)     
        
    def find_low_points(self, height:str, x:int, y:int) -> None:
        """Find the low points - the locations that are 
        lower than any of its adjacent locations from a 2D grid.
        
        If the current point is lower than all surrounding points, collects
        the info to __low_points and the risk level to __risk_levels.
        """

        sur:list = [] # list of surrounding points

        for i, j in [(0,1), (0,-1), (1,0), (-1,0)]:
            if self.is_index_valid(self.__data, y+i) and self.is_index_valid(self.__data[y+i], x+j):                   
                sur.append(self.__data[y+i][x+j])

        if len(sur) > 0 and height < min(sur):
            self.__low_points.append((y, x))
            self.__risk_levels += RISK_LEVEL + height      

        
def main():
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2021\\09\\puzzle.input") 
    sol = Solution(data)

    return 0

main()