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
        self.h_pos = (0, 0)
        self.h_prev = ()
        self.t_pos = (0, 0)
        self.t_moved = 0

    def print(self):
        for row in self.mapx:
            print(''.join(row))
        
    def walk(self):                      
        while True:
            # return when all moves have been done
            if self.__moves_done == self.__moves:
                print("silver:", self.t_moved)
                return  
            
            #cmd = input("Cmd: ")
            #if cmd != "":
            #    break            
            
            self.move()
    
    def move(self):
        if self.__moves_done < self.__moves:
            dir:str = self.__data[self.__moves_done][0:1]
            amt:int = int(self.__data[self.__moves_done][2:])
            
            i:int = 0
            while i < amt:  
                # if t is more than 1 away from h, move t to h's prev place
                if abs(self.h_pos[0]-self.t_pos[0]) > 1 or \
                   abs(self.h_pos[1]-self.t_pos[1]) > 1:
                    self.t_pos = self.h_prev  
                    self.t_moved += 1                    
          
                print("H", self.h_pos, "T", self.t_pos)   
                
                # save previous point
                self.h_prev = self.h_pos          

                if   dir == "R":  self.h_pos = (self.h_pos[0],  self.h_pos[1]+1)                            
                elif dir == "L":  self.h_pos = (self.h_pos[0],  self.h_pos[1]-1)                       
                elif dir == "U":  self.h_pos = (self.h_pos[0]-1,self.h_pos[1])                              
                elif dir == "D":  self.h_pos = (self.h_pos[0]+1,self.h_pos[1])

                #time.sleep(0.5)
                #input("waiting..")
                i += 1
            
            self.__moves_done += 1       

def main():
    # 9038 too much
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\09\\puzzle.input") 
    
    sol = Solution(data)  
    sol.walk()  
    return 0

if __name__ == "__main__":
    main()
