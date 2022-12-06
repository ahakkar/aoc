# -*- encoding: utf-8 -*-
'''
@File    :   05.py
@Time    :   05/12/2022, 10:20:23
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

# get top crates from each stack, assume no stacks are empty :D
def top(crates:dict):
    solution:str = ""
    for key, val in crates.items():
        solution += val.pop()
        
    print(solution)

def silver(crates:dict, commands:list):
    """
    param : puzzle input as list
    return: none
    """

    for row in commands:        
        # extract numbers from command
        nums:int = [int(x) for x in row.replace('move', "").replace('from', "")\
                                       .replace('to', "").strip().split('  ')]
        for i in range(0, nums[0]):
            crates[nums[2]].append(crates[nums[1]].pop())        

    top(crates)
    
def gold(crates:dict, commands:list):
    """
    param : puzzle input as list
    return: none
    """
    for row in commands:        
        # extract numbers from command
        nums:int = [int(x) for x in row.replace('move', "").replace('from', "")\
                                       .replace('to', "").strip().split('  ')]
        move:list = []
        
        # still move crates one by one, but to a temp list
        for i in range(0, nums[0]):
            move.append(crates[nums[1]].pop())             
        
        # extract crates from temp list to main list, one by one
        for i in range(0, len(move)):
            crates[nums[2]].append(move.pop())     

    top(crates)
    
def top2(crates:list):
    print(crates)
    
   
def string_test(crates:str, commands:list):
    crates:list = crates.split(' ')
    print(crates)
    for row in commands: 
        # extract numbers from command
        move, fr, to = [int(x) for x in row.replace('move', "")\
                                           .replace('from', "")\
                                           .replace('to', "")\
                                           .strip()\
                                           .split('  ')]
        #print(move, fr, to)
        crates[to] += crates[fr][len(crates[fr])-move:len(crates[fr])]
        crates[fr] = crates[fr][:len(crates[fr])-move]
        
    top2(crates)

# create dict with lists from puzzle crate input
def fix(crates:list) -> list:
    fixed:dict = {}
    i:int = 1
    for stack in crates.split(' '):
        fixed[i] = []
        for crate in stack:
            fixed[i].append(crate)
        i += 1
            
    return fixed              

def main():
    commands:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\05\\puzzle.input") 
    
    simple:str = "ZN MCD P" 
    puzzle:str = "DHNQTWVB DWB TSQWJC FJRNZTP GPVJMST BWFTN BLDQFHVN HPFR ZSMBLNPH"    
    
    #string_test(simple, commands) #PSNRGBTFT
    silver(fix(puzzle), commands) #PSNRGBTFT
    gold(fix(puzzle), commands) #BNTZFPMMW
    return 0

main()

# 1320ms :-E
#import timeit    
#time = timeit.timeit(main, number=1000)
#print(f"{time*1000:.5f}ms")
