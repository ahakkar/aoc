# -*- encoding: utf-8 -*-
'''
@File    :   02.py
@Time    :   02/12/2022, 07:08:30
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
    """
    param : puzzle input as list
    return: none
    """
    total:int = 0
    for row in data: 
        key, val = row.split(' ')        
        # rock
        if val == "X":
            # rock
            if key == "A":
                total += 3 #draw                
            # paper
            elif key == "B":                
                pass
            elif key == "C":
                total += 6 # won
            else:
                print("wrong data!")
                
            # for choosing rock
            total += 1
        # paper
        elif val == "Y":
            # rock
            if key == "A": # rock
                total += 6 # won                
            # paper
            elif key == "B":                
                total += 3 #draw  
            elif key == "C": # scissors
                pass #loss
            else:
                print("wrong data!")
                
            # for choosing paper
            total += 2
        # scissors
        elif val == "Z":                       
            if key == "A": # rock
                pass #loss                
            # paper
            elif key == "B":                
                total += 6 # won
            elif key == "C":
                total += 3 # draw
            else:
                print("wrong data!")
                
            # for choosing scissors
            total += 3
        else:
            print("wrong data!")
            
    #print("silver:", total)
        
    
def gold(data:list):
    """
    param : puzzle input as list
    return: none
    """
    total:int = 0
    for row in data: 
        key, val = row.split(' ')        
        # rock
        if key == "A":
            if val == "Y": # need to draw
                total += 1 + 3
            elif val == "X": #need to lose
                total += 3 + 0
            elif val == "Z": # need to win
                total += 2 + 6
            else:
                print("wrong data!")
        # paper
        elif key == "B":
            if val == "Y": # need to draw
                total += 2 + 3
            elif val == "X": #need to lose
                total += 1 + 0
            elif val == "Z": # need to win
                total += 3 + 6
        # scissors
        elif key == "C":
            if val == "Y": # need to draw
                total += 3 + 3
            elif val == "X": #need to lose
                total += 2 + 0
            elif val == "Z": # need to win
                total += 1 + 6

        else:
            print("wrong data!")
            
    #print("gold:", total)

"""
The score for a single round is the score for the shape you selected
(1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the 
outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

A= rock
B = Paper
C = Scissors
Y = Paper
X = Rock
Z = Scissors

"""

def main():
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\02\\puzzle.input") 
    
    silver(data)
    gold(data)
    return 0

import timeit    
time = timeit.timeit(main, number=1000)
print(f"{time*1000:.5f}ms")