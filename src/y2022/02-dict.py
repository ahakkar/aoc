# -*- encoding: utf-8 -*-
'''
@File    :   02-test.py
@Time    :   02/12/2022, 12:57:09
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

def silver(data:list):
    total:int = 0
    scoring:dict = {"B X": 1, "C X": 7, "A X": 4,
                    "A Y": 8, "B Y": 5, "C Y": 2,
                    "C Z": 6, "A Z": 3, "B Z": 9}
    
    for row in data: 
        total += scoring[row]
            
    #print("silver:", total)        
    
def gold(data:list):
    """
    param : puzzle input as list
    return: none
    """
    total:int = 0
    scoring:dict = {"B X": 1, "C X": 2, "A X": 3,
                    "A Y": 4, "B Y": 5, "C Y": 6,
                    "C Z": 7, "A Z": 8, "B Z": 9}
    
    for row in data: 
        total += scoring[row]
            
    #print("gold:", total)

def main():
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\02\\puzzle.input") 
    
    #silver: 13009
    #gold: 10398
    silver(data)
    gold(data)
    return 0

import timeit    
time = timeit.timeit(main, number=1000)
print(f"{time*1000:.5f}ms")
