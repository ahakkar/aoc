# -*- encoding: utf-8 -*-
'''
@File    :   09.py
@Time    :   09/12/2022, 10:27:48
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2022
@Desc    :   TEMP
'''
import time

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
        self.__data:list = data
        self.__moves = len(self.__data)
        self.__moves_done = 0
        self.h_pos:tuple = (0, 0)
        self.t_pos:tuple = (0, 0)
        self.h_prev = ()
        self.t_moves:set = set()
        self.t_moves.add(self.t_pos)
        #print(self.t_moves)  
        
             
    def walk(self):                      
        while True:
            # return when all moves have been done
            if self.__moves_done == self.__moves:
                print("Final position: H", self.h_pos, "T", self.t_pos)
                print("silver:", len(self.t_moves))
                return  

            self.move()
    
    def move(self):
        if self.__moves_done < self.__moves:
            dir:str = self.__data[self.__moves_done][0:1]
            amt:int = int(self.__data[self.__moves_done][2:])
            #print("command:", self.__data[self.__moves_done])
            
            i:int = 0
            while i < amt:  
                # if t is more than 1 away from h, move t to h's prev place
                if abs(self.h_pos[0]-self.t_pos[0]) > 1 or \
                   abs(self.h_pos[1]-self.t_pos[1]) > 1:
                    self.t_pos = self.h_prev  
                    self.t_moves.add(self.t_pos)                    
          
                #print("H", self.h_pos, "T", self.t_pos, "moves:", len(self.t_moves))   
                
                # save previous point
                self.h_prev = self.h_pos          

                if   dir == "R":  self.h_pos = (self.h_pos[0],  self.h_pos[1]+1)                            
                elif dir == "L":  self.h_pos = (self.h_pos[0],  self.h_pos[1]-1)                       
                elif dir == "U":  self.h_pos = (self.h_pos[0]+1,self.h_pos[1])                              
                elif dir == "D":  self.h_pos = (self.h_pos[0]-1,self.h_pos[1])

                i += 1
            
            self.__moves_done += 1     
            
    def print(self):  
        SIZE = 20
        mapx = [["." for i in range(SIZE)] for j in range(SIZE)] 
        
        for val in self.t_moves:
            mapx[val[0]][val[1]] = "#"
        
        for i in range(len(mapx)-1, -1, -1):            
            print(''.join(mapx[i])) 
        
    def test(self):
        minimum = min(min(self.t_moves))
        maximum = max(max(self.t_moves))
        SIZE = abs(minimum) + maximum
        print(SIZE)
        mapx = [["." for i in range(SIZE)] for j in range(SIZE)] 
        
        new_set = set()
        for val in self.t_moves:
            new_set.add((val[0]+minimum, val[1]+minimum))   
            
        print(new_set)    
        
        return
def main():
    # 9038 too much # 6174 too low #6175 is good, missing one due to a bug
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\09\\medium.input") 
    
    sol = Solution(data)  
    sol.walk()  
    sol.print()
    return 0

if __name__ == "__main__":
    main()
