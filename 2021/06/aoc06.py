# -*- encoding: utf-8 -*-
'''
@File    :   aoc06.py
@Time    :   09/11/2022, 17:39:32
@Author  :   Antti Hakkarainen
@Contact :   a.hakkarainen@iki.fi
@Task    :   Advent of Code 2021
@Desc    :   TEMP
'''

# -*- encoding: utf-8 -*-
'''
@File    :   05a.py
@Time    :   08/11/2022, 14:32:20
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2021
@Desc    :   TEMP
'''

import numpy

class Fish:
    
    def __init__(self, filename:str) -> None: 
        MAX_FISH_AGE = 8  
             
        self.data:list = []
        self.fish:dict = {}
        
        # init empty dict with fish ages
        for num in range (0, MAX_FISH_AGE+1):
            self.fish[num] = 0
        
        self.read_file(filename)
        self.add_fish_to_dict()  
        
    def get_fish(self) -> dict:
        return self.fish    

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
    
    def add_fish_to_dict(self) -> None:
        """
        param : TODO
        return: none
        """
        
        ages = [int(i) for i in self.data[0].split(',')]
        for age in ages:
            self.fish[age] +=1
            
    def advance(self) -> None:
        """
        param : TODO
        return: none
        """
        temp_dict = self.fish.copy()
        new_fish = 0
        six_day_fish = 0
        
        for age, amount in self.fish.items():            
            if age == 0:
                new_fish = amount
                six_day_fish = amount
            if age == 1:
                temp_dict[0] = amount 
            if age == 2:
                temp_dict[1] = amount 
            if age == 3:
                temp_dict[2] = amount 
            if age == 4:
                temp_dict[3] = amount 
            if age == 5:
                temp_dict[4] = amount 
            if age == 6:
                temp_dict[5] = amount                
            if age == 7:
                temp_dict[6] = amount 
            if age == 8:
                temp_dict[7] = amount
                temp_dict[8] = 0
                
        temp_dict[6] += six_day_fish
        temp_dict[8] = new_fish
        
        self.fish = temp_dict.copy()
        
    def total_fish(self) -> int:
        total:int = 0
        for key, val in self.fish.items():
            total += val    
            
        return total     
         

def main():
    days:int = 256
    school = Fish("06.input") 
    for i in range (0,days):
        school.advance() 
        #print(school.get_fish())     
    
    print(f"total fish after {days} days", school.total_fish())
    
    return 0

if __name__ == "__main__":
    from time import time
    start = time()
    
    main()

    # print the difference between start and end time in ms
    print("The time of execution of above program is:",
        (time()-start) * 10**3, "ms")  