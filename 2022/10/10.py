# -*- encoding: utf-8 -*-
'''
@File    :   10.py
@Time    :   10/12/2022, 10:19:59
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2022
@Desc    :   TEMP
'''

from collections import deque 
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
        self.cmds:deque = deque(data)
        self.reg:int = 1        
        self.cycle:int = 1
        self.draw_cycle:int = 1
        
        self.signals:list = []               
              
    def run(self):
        """
        For now, consider the signal strength (the cycle number multiplied by the
        value of the X register) during the 20th cycle and every 40 cycles after that
        (that is, during the 20th, 60th, 100th, 140th, 180th, and 220th cycles).
        
        - addx V takes two cycles to complete. 
          After two cycles, the X register is increased by the value V. (V can be negative.)
        - noop takes one cycle to complete. It has no other effect.
        """
        while True:
            params = ""
            if len(self.cmds) > 0:
                params = self.cmds.popleft()
             
            # command takes 2 cycles    
            if params[0:4] == "addx": 
                for _ in range(0,2):
                    self.calculate()                    
                self.reg += int(params[5:])                 
             
            # command takes 1 cycle   
            elif params[0:4] == "noop": 
                for _ in range(0,1):
                    self.calculate()                 
            
            # exit if out of commands
            else:
                break            
    
        # 14560
        print("Silver:", sum(self.signals))
        
        # gold: EKRHEPUZ (as read from CRT screen)
        
    def calculate(self):
        self.draw()  
        if self.cycle in (20, 60, 100, 140, 180, 220):
            self.signals.append(self.reg * self.cycle) 
        self.cycle += 1
    
    def draw(self):
        if (self.draw_cycle >= self.reg) and (self.draw_cycle < self.reg+3):
            print("#", end="")
        else:
            print(".", end="")
        if self.draw_cycle == 40:
            self.draw_cycle = 0
            print("\n", end="") 
        self.draw_cycle +=1

def main():
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\10\\puzzle.input") 
    
    sol = Solution(data)
    sol.run()

    return 0

if __name__ == "__main__":
    main()
