# -*- encoding: utf-8 -*-
'''
@File    :   aoc08.py
@Time    :   09/11/2022, 21:20:53
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2021
@Desc    :   TEMP
'''
class Display:
    
    def __init__(self, filename:str):
        """
        param : TODO
        return: none
        """
        self.data:list = []
        self.read_file(filename)
        
        
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
            
    def count_easy(self) -> int:
        """
        param : TODO
        return: none
        """
        count:int = 0
        
        for row in self.data:
            parts = [x.strip().split(" ") for x in row.split('|')]
            #print(parts[1])
            for str in parts[1]:
                if len(str) in (2,4,3,7):
                    count += 1
                    
        return count
    
    def count_hard(self) -> int:
        """      
        
        1 = be
        8 = abcdefg
        4 = bceg

        0/6/9 = bcdefg 9?
        0/6/9 = acdefg 6?
        0/6/9 = abdefg 0? (koska c puuttuu)

        2/3/5 = cdefg 5?
        2/3/5 = bcdef 3?
        2/3/5 = abcdf 2?

        7 = bde
        ***************
        8 = abcdefg
        2/3/5 = bcdef
        0/6/9 = bcdefg
        4 = bceg

         dddd
        g	 b
        g	 b
         cccc	
        a	 e
        a	 e
         ffff  
        return: none
        """
        count:int = 0
                       
        for row in self.data:
            parts = [x.strip().split(" ") for x in row.split('|')]    
            keys:dict = self.solve_keys(parts[0] + parts[1])
            
            num: str = ""               
            for decode in parts[1]:
                decode = "".join([x for x in sorted(decode)])
                # this is so not optimal..
                # should have key:value in flipped order
                for key, val in keys.items():
                    if decode == val:
                        num += str(key)  
            
            count += int(num) # ..... really            
       
        return count
    
    def solve_keys(self, row:list) -> dict:
        """
        :param list row: _description_
        :return dict: _description_
        """
        keys:dict = {}
        set_069 = set()
        set_235 = set()            
            
        # sort string letters
        for str in row:
            str = "".join([x for x in sorted(str)])
            
            if len(str) == 2:
                keys[1] = str
            elif len(str) == 4:
                keys[4] = str
            elif len(str) == 3:
                keys[7] = str
            elif len(str) == 7:
                keys[8] = str
            elif len(str) == 6:  
                set_069.add(str) 
            elif len(str) == 5:
                set_235.add(str)
            else:
                print("!?!?!") 
        
        #print("SETS:", set_069, set_235)

        one_ch = {}
        for ch in keys[1]:
            one_ch[ch] = 1
            
        five_ch = {}
        for val in set_235:
            for ch in val:
                if ch in five_ch:
                    five_ch[ch] += 1
                else:
                    five_ch.update({ch: 1})
                    
        six_ch = {}
        for val in set_069:        
            for ch in val:            
                if ch in six_ch:
                    six_ch[ch] += 1
                else:
                    six_ch.update({ch: 1})   
        
        # figure out value 5
        for key, val in five_ch.items():              
            if val == 1 and key in six_ch:            
                if six_ch[key] == 3:                
                    for s in set_235:
                        if s.find(key) >= 0:                       
                            keys[5] = s # this is no 5 str                             
        set_235.remove(keys[5])    
                   
        # figure out value 3
        for val in set_235:
            hits = 0
            for ch in val:
                if ch in one_ch:
                    hits += 1
            if hits == 2:
                keys[3] = val # this is 3           
        
        # 2 is the remaining one
        set_235.remove(keys[3]) 
        keys[2] = set_235.pop() # pop() returns last item from set

        # find 6 first by looking which one is missing other piece from 1
        for val in set_069:
            hits = 0
            for ch in val:
                if ch in one_ch:
                    hits += 1
            if hits == 1:
                keys[6] = val # this is 6  
                
        set_069.remove(keys[6])
        
        # find 9 by comparing it to 3
        for val in set_069:
            diff = set(val).difference(set(keys[3]))
            if len(diff) == 1:
                keys[9] = val   
        
        # 0 is last        
        set_069.remove(keys[9])      
        keys[0] = set_069.pop()     

        return keys
        

def main():
    d = Display("task.input")
    total = d.count_hard()
    """
    if total != 26:
        print(f"wrong amount! ({total})")
    else:
    """
    #print("nums:", total)

import timeit    
time = timeit.timeit(main, number=1000)
print(f"{time*1000:.5f}ms")
