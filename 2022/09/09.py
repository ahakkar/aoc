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

SIZE = 10

class Solution:
    
    def __init__(self, data:list):
        self.__data:list = data
        self.__moves = len(self.__data)
        self.__moves_done = 0
        self.h_pos = (SIZE-1, 1)
        self.t_pos = (SIZE-1, 0)
        self.t_prev = ()
        self.mapx = [["." for i in range(SIZE)] for j in range(SIZE)] 
        self.mapx[self.h_pos[0]][self.h_pos[1]] = "H"      

    def silver(self):
        """
        param : puzzle input as list
        return: none
        """
        print("TBD")
        
    def gold(self):
        """
        param : puzzle input as list
        return: none
        """
        print("TBD")
    
    def print(self):
        for row in self.mapx:
            print(''.join(row))
        
    def walk(self):                      
        while True:
            # return when all moves have been done
            if self.__moves_done == self.__moves:
                return  
            
            cmd = input("Cmd: ")
            if cmd != "":
                break            
            
            print("H", self.h_pos, "T", self.t_pos)  
            self.print()
            self.move()
    
    def move(self):
        if self.__moves_done < self.__moves:
            dir:str = self.__data[self.__moves_done][0:1]
            amt:int = int(self.__data[self.__moves_done][2:])
            diagonal: bool = False
            #print(dir, amt)
            
            i:int = 0
            while i < amt:               
                # replace H with .; move H, render H again
                self.mapx[self.h_pos[0]][self.h_pos[1]] = "." 
                print("H", self.h_pos, "T", self.t_pos)  
                
                # diagonal dir
                if (self.t_pos[0] != self.h_pos[0]) and (self.t_pos[1] != self.h_pos[1]):
                    self.t_pos == self.h_pos
                elif self.h_pos == self.t_pos:
                    pass
                
                if dir == "R":
                    self.h_pos = (self.h_pos[0], self.h_pos[1]+1)
                    if self.h_pos != self.t_pos:
                        self.t_pos = (self.t_pos[0], self.t_pos[1]+1)
                            
                elif dir == "L":
                    self.h_pos = (self.h_pos[0], self.h_pos[1]-1)
                    if self.h_pos != self.t_pos:
                        self.t_pos = (self.t_pos[0], self.t_pos[1]-1)
                        
                elif dir == "U":
                    self.h_pos = (self.h_pos[0]-1, self.h_pos[1])
                    if self.h_pos != self.t_pos:      
                        self.t_pos = (self.t_pos[0]-1, self.t_pos[1])
                            
                elif dir == "D":                    
                    self.h_pos = (self.h_pos[0]+1, self.h_pos[1])
                    if self.h_pos != self.t_pos:   
                        self.t_pos = (self.t_pos[0]+1, self.t_pos[1])
                else:
                    print("error error with dir")
                    exit(0)   
                                 
                self.mapx[self.h_pos[0]][self.h_pos[1]] = "H" 
                      
                if self.h_pos != self.t_pos:
                    self.mapx[self.t_pos[0]][self.t_pos[1]] = "T"                 
                
                self.print()               
                #time.sleep(0.5)
                input("waiting..")
                i += 1
            
            self.__moves_done += 1
    
    def temp(self):
        # move T
        if self.h_pos != self.t_pos:                                      
            # NE
            if self.t_pos == (self.h_pos[0]+1, self.h_pos[1]-1):
                print("ne")
                self.t_pos == self.h_pos
            # NW
            elif self.t_pos == (self.h_pos[0]+1, self.h_pos[1]+1):
                print("nw")
                self.t_pos == self.h_pos                        
            # SE
            elif self.t_pos == (self.h_pos[0]-1, self.h_pos[1]-1):
                print("se")
                self.t_pos == self.h_pos                        
            # SW
            elif self.t_pos == (self.h_pos[0]-1, self.h_pos[1]+1):
                print("sw")
                self.t_pos == self.h_pos    
        if self.t_prev:
            self.mapx[self.t_prev[0]][self.t_prev[1]] = "."
            self.mapx[self.t_pos[0]][self.t_pos[1]] = "T"
            self.t_prev = (self.t_pos[0], self.t_pos[1])
            
            if dir == "R":
                self.t_pos = (self.t_pos[0], self.t_pos[1]+1)
            elif dir == "L":
                self.t_pos = (self.t_pos[0], self.t_pos[1]-1)
            elif dir == "U":
                self.t_pos = (self.t_pos[0]-1, self.t_pos[1])
            elif dir == "D":                    
                self.t_pos = (self.t_pos[0]+1, self.t_pos[1])
            else:
                print("error error with dir")
                exit(0) 
            

def main():
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\09\\simple.input") 
    
    sol = Solution(data)  
    sol.walk()  
    return 0

if __name__ == "__main__":
    main()
