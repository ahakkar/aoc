# -*- encoding: utf-8 -*-
'''
@File    :   05a.py
@Time    :   08/11/2022, 14:32:20
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2021
@Desc    :   TEMP
'''

import numpy

class Lines:
    
    def __init__(self, filename:str) -> None:        
        self.data: list
        self.lines: list = []
        # we could use 0 here for max_x and max_y, and inspect all the
        # numbers in add_lines_to_list, but it doubles the runtime
        self.max_x: int = 10
        self.max_y: int = 10
        
        self.read_file(filename)
        self.add_lines_to_list()          
        self.Matrix = numpy.zeros((self.max_x+1, self.max_y+1)) 

    def read_file(self, filename:str) -> None:
        """
        param : str, filename to read
        return: None
        """
        
        try:
            with open (filename, "r") as read_file:
                self.data: list = read_file.read().splitlines()
            read_file.close()
        except OSError:
            print("Bad file name!")
            exit()
            
    def get_lines(self) -> list:
        return self.lines
    
    def get_matrix(self):
        return self.Matrix

    def add_lines_to_list(self) -> None:
        for row in self.data:
            if row != "":
                temp = row.split(' -> ')
                #x, y = [int(i) for i in temp[0].split(',')]
                #if x > self.max_x: self.max_x = x 
                #if y > self.max_y: self.max_y = y   
                #x, y = [int(i) for i in temp[1].split(',')]
                #if x > self.max_x: self.max_x = x 
                #if y > self.max_y: self.max_y = y      
                self.lines.append((temp[0], temp[1]))
                
    def print_lines(self) -> None:
        for val in self.lines:
            print(f"{val[0]} -> {val[1]}")
            
             
    def draw_line(self, line:tuple) -> None:         
        x1, y1 = [int(i) for i in line[0].split(',')]    
        x2, y2 = [int(i) for i in line[1].split(',')]
        
        # swap drawing dir from left to right if neccessary
        if x1 > x2:                
            x1, x2 = x2, x1  
        # swap drawing dir from top to bot if neccessary
        if y1 > y2:
            y1, y2 = y2, y1    
        
        # horizontal or vertical line
        if x1 == x2 or y1 == y2: 
            # "draw" the line to 2d array
            self.Matrix[y1:y2+1 , x1:x2+1] +=1              
        #diagonal line
        else:   
            print("line not drawn: diagonal")         
            pass   
            
    def print_matrix(self) -> None:
        """
        param : TODO
        return: none
        """
        
        for row in self.Matrix:
            for num in row:
                if int(num) == 0:
                    print('.', end="")
                else:
                    print(int(num), end="")
            print()
      
    def calculate_overlap(self) -> int:
        """
        param : TODO
        return: none
        """        
        return numpy.count_nonzero(self.Matrix > 1)
    
    def resize_array(self, size_x:int, size_y:int) -> None:
        """
        param : TODO
        return: none
        """
        
        temp = numpy.zeros(size_x, size_y) 
        #self.Matrix = numpy.resize(self.Matrix, (size_x, size_y))
        self.max_x = size_x
        self.max_y = size_y
 
def main():
    lines = Lines("simple.input")    
    list_of_lines = lines.get_lines()
    
    #lines.draw_line(list_of_lines[0])
    #lines.draw_line(list_of_lines[1])
    #lines.draw_line(list_of_lines[2])
    #lines.draw_line(list_of_lines[3])
    
    for line in list_of_lines:
        lines.draw_line(line)
    
    lines.print_matrix()
    lines.resize_array(50,50)   
    lines.print_matrix() 
    
    overlap = lines.calculate_overlap()
    print("overlap:", overlap)
    
    #lines.print_matrix()
    
    return 0

if __name__ == "__main__":
    from time import time
    start = time()

    main()
  
    # record end time
    end = time()

    # print the difference between start and end time in ms
    print("The time of execution of above program is:",
        (end-start) * 10**3, "ms")   
