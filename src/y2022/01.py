# -*- encoding: utf-8 -*-
'''
@File    :   01a.py
@Time    :   01/12/2022, 10:40:01
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2022
@Desc    :   TEMP
'''

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

def calc_calories(data:list):
    total: int = 0
    elves:list = []
    
    i:int = 0
    while i < len(data):
        if len(data[i]) > 0:            
            total += int(data[i])
        else:
            elves.append(total)
            total = 0
        # add last kcals
        if i == (len(data) -1):
            elves.append(total)
        
        i += 1
        
    return elves

# get fattest elf
def sol_a(elves:list) -> int:            
    return max(elves)

# get three fattest
def sol_b(elves:list) -> int: 
    elves.sort()
    max_three:list = elves[-3:]   
            
    return sum(max_three)        

def main():
    elves:list = calc_calories(read_file("D:\\GDrive\\Prog\\aoc\\2022\\01\\puzzle.input")) 
    
    fattest_elf:int = sol_a(elves)
    #print(fattest_elf)
    
    sum_three:int = sol_b(elves)
    #print(sum_three)
    
    return 0

import timeit    
time = timeit.timeit(main, number=1000)
print(f"{time*1000:.5f}ms")


