# -*- encoding: utf-8 -*-
'''
@File    :   04a.py
@Time    :   07/11/2022, 22:08:23
@Author  :   Antti Hakkarainen
@Contact :   a.hakkarainen@iki.fi
@Task    :   Advent of Code 2021
@Desc    :   Day 4 A : Bingo
'''

import time
start = time.time()

def read_file(filename:str) -> dict:
    """
    param : TODO
    return: none
    """
    data:list = []
    
    try:
        with open (filename, "r") as read_file:
            data:list = read_file.read().splitlines()
        read_file.close()
    except OSError:
        print("Bad file name!")
        return data
    
    return data

def check_bingo(card:dict) -> bool:
    """
    param : TODO
    return: none
    """
    
    #hor
    count = 0
    i = 0
    for val in card.items():
        if val[1] == 1:
            count += 1
        if i == 4:
            if count == 5:
                return True
            count = 0
            i = 0
            continue
        i += 1
        
    #vert
    i = 0   
    # find out this is needed only for part B     
    
    return False

data: list = read_file("input.txt")
bingo: list = data[0].split(',')
data.pop(0)

cards: dict = {}

# add cards to dict, 0 = no match, 1 = match
i: int = 0
for row in data:
    nums = row.strip().split(' ')    
    if len(row) != 14:
        i += 1        
    else:  
        for num in nums:
            # avoid adding empty keys
            if not cards.get(i):
                cards[i] = {}
            if num != "":
                cards[i][int(num)] = 0
                
print(cards[74])
          
# go through all bingo numbers
won = False
won_card = -1
won_num = -1

for num in bingo: 
    if won:
        break
    
    i = 1
    num = int(num)
    # go through all bingo cards
    while i < len(cards)+1:
        result = cards[i].get(num)
        if result != None:
            cards[i][num] = 1
            if check_bingo(cards[i]):
                print("BINGO!")
                won = True
                won_card = i
                won_num = num
                break
        i += 1   

# Start by finding the sum of all unmarked numbers on
# that board; in this case, the sum is 188. Then,
# multiply that sum by the number that was just
# called when the board won, 24, to get the final score, 
# 188 * 24 = 4512.
    
tmp = 0
sum = 0
for val in cards[won_card].items():
    print(f"{val[1]} ", end= "")
    if val[1] == 0:
        sum += val[0]
    tmp += 1
    if tmp % 5 == 0:
        print()
        
print(sum*won_num)
    
# record end time
end = time.time()

# print the difference between start
# and end time in milli. secs
print("The time of execution of above program is:",
    (end-start) * 10**3, "ms")
