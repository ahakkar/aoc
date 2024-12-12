# -*- encoding: utf-8 -*-
'''
@File    :   03.py
@Time    :   03/12/2022, 09:18:45
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

def item_prio(ch:str) -> int:
    if ord(ch) > 90:   
        return ord(ch) - 96 #A-Z
    return ord(ch) - 38 #a-z

def silver(data:list):
    """
    param : puzzle input as list
    return: none
    """
    total:int = 0    
    
    for row in data:
        # split row to two
        #blindly assume all rows are even length
        row_length = len(row)
        a = row[0: row_length//2]
        b = row[row_length//2:]        
        
        # look for ch from str b, add it's prio if found
        for ch in a:
            if b.count(ch) > 0:
                total += item_prio(ch)
                break
            
    #print(total)
    
def analyze_grp(grp:list) -> int:
    for ch in grp[0]:
        if (grp[1].count(ch) > 0) and (grp[2].count(ch) > 0):
            return item_prio(ch)
    
def gold(data:list):
    """
    param : puzzle input as list
    return: none
    """
    total:int = 0
    grp:list = []
    i:int = 0
    
    for row in data:
        # every 3 rows analyze findings
        grp.append(row)
        i += 1
        if i == 3:
            total += analyze_grp(grp)
            grp = []
            i = 0
            
    #print(total)        

def main():
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\03\\puzzle.input") 
       
    silver(data)
    gold(data)
    return 0

import timeit    
time = timeit.timeit(main, number=1000)
print(f"{time*1000:.5f}ms")



