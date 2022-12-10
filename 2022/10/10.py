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
        self.cyc2:deque = deque()
        self.cyc1:deque = deque()
        self.cyc0:int = None
        
        self.cycle:int = 1
        
        self.signals:list = []
    
    def garbage(self):
        """
        For now, consider the signal strength (the cycle number multiplied by the
        value of the X register) during the 20th cycle and every 40 cycles after that
        (that is, during the 20th, 60th, 100th, 140th, 180th, and 220th cycles).
        
        - addx V takes two cycles to complete. 
          After two cycles, the X register is increased by the value V. (V can be negative.)
        - noop takes one cycle to complete. It has no other effect.
        """
        cycle:int = 1
        
        while True:   
            if self.cyc0 != None:
                self.reg += self.cyc0
                self.cyc0 = None                  

            if len(self.cmds) > 0:
                params = self.cmds.popleft()
                
                if params[0:4] == "addx":   
                    self.cyc2.append(int(params[5:]))      
                    
            #print("cycle:", cycle, self.cyc2, self.cyc1, self.reg)  
            #time.sleep(0.2)   
               
            if cycle in (20, 60, 100, 140, 180, 220):
                print("reg:", self.reg, "signal:", self.reg * cycle)   
                if cycle == 220:
                    break 
            
            cycle += 1  
            
            # move values from 1 cycles to 0
            if len(self.cyc1) > 0 and self.cyc0 == None:               
                self.cyc0 = self.cyc1.popleft()
            
            # move values from 2 cycles to 1
            if len(self.cyc2) > 0 and self.cyc0 == None:               
                self.cyc1.append(self.cyc2.popleft())  
                
    def is_calc_complete(self):
        if self.cycle in (20, 60, 100, 140, 180, 220):
            self.signals.append(self.reg * self.cycle)
            print("cycle:", self.cycle, "reg:", self.reg, "signal:", self.reg * self.cycle)   
        self.cycle += 1
                
    def silver(self):
        while True:
            params = ""
            if len(self.cmds) > 0:
                params = self.cmds.popleft()
             
            # command takes 2 cycles    
            if params[0:4] == "addx": 
                for _ in range(0,2):
                    self.is_calc_complete()                    
                self.reg += int(params[5:])                 
             
            # command takes 1 cycle   
            elif params[0:4] == "noop": 
                for _ in range(0,1):
                    self.is_calc_complete()                 
            
            # cycle if out of commands
            else:
                self.cycle += 1
                self.is_calc_complete() 
                if self.cycle >= 220: break            
    
        # 14560
        print("Silver:", sum(self.signals))
    
    def gold(self):
        pass

def main():
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\10\\puzzle.input") 
    
    sol = Solution(data)
    sol.silver()

    return 0

if __name__ == "__main__":
    main()
