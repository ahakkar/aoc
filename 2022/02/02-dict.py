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
    scoring:dict = {"BX": 1, "CX": 7, "AX": 4,
                    "AY": 8, "BY": 5, "CY": 2,
                    "CZ": 6, "AZ": 3, "BZ": 9}
    
    for row in data: 
        total += scoring["".join(row.split())]
            
    print("silver:", total)        
    
def gold(data:list):
    """
    param : puzzle input as list
    return: none
    """
    total:int = 0
    scoring:dict = {"BX": 1, "CX": 2, "AX": 3,
                    "AY": 4, "BY": 5, "CY": 6,
                    "CZ": 7, "AZ": 8, "BZ": 9}
    
    for row in data: 
        total += scoring["".join(row.split())]
            
    print("gold:", total)

def main():
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\02\\puzzle.input") 
    
    #silver: 13009
    #gold: 10398
    silver(data)
    gold(data)
    return 0

main()

"""
import timeit    
time = timeit.timeit(main, number=1000)
print(f"{time*1000:.5f}ms")
"""