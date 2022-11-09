# -*- encoding: utf-8 -*-
'''
@File    :   04a.py
@Time    :   07/11/2022, 23:53:00
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2021
@Desc    :   Day 4 B : Bingo
             First tried to do this with functions.
'''

class Bingo:
    
    def __init__(self, filename:str) -> None:
        self.data: list 
        self.cards: dict = {}
        self.read_file(filename)       
         
        # add bingo numbers to a list. split string and convert to int in 1 line
        self.bingo_nums: list = [int(i) for i in self.data[0].split(',')]
        self.data.pop(0)
        
        self.cards_not_won: dict = {}
        
    def get_data(self) -> list:
        return self.data
    
    def get_cards(self) -> dict:
        return self.cards
    
    def get_bingo_nums(self) -> list:
        return self.bingo_nums
    
    def get_amount_of_cards(self) -> int:
        return len(self.cards)

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
    
    def add_cards_to_dict(self) -> None:
        # add cards to dict, 0 = no match, 1 = match
        i: int = 0
        for row in self.data:
            nums = row.strip().split(' ')    
            if len(row) != 14:
                i += 1        
            else:  
                for num in nums:
                    # avoid adding empty keys
                    if not self.cards.get(i):
                        self.cards[i] = {}                        
                    if num != "":
                        self.cards[i][int(num)] = 0

    def update_card(self, card_id:int, bingo_num:int) -> None:                  
        # look for key with called bingo number                  
        result = self.cards[card_id].get(bingo_num)        
        
        # if key is found, update value
        if result != None:
            self.cards[card_id][bingo_num] = 1  
            
    def check_card(self, card_id:int) -> bool:                  
        """
        look for BINGO from one card
        param : int, card id to check
        return: bool, BINGO y/n?
        """                   
                 
        #horizontal bingo
        count = 0
        i = 0
        for val in self.cards[card_id].items():
            if val[1] == 1:
                count += 1
            if i == 4:
                if count == 5:                    
                    return True
                count = 0
                i = 0
                continue
            i += 1  
            
        #vertical bingo
        i = 0   
        keys: list = list(self.cards[card_id].keys())           
        
        for val in self.cards[card_id].items():
            if i > 4: break
            if val[1] == 1:
                if self.cards[card_id][keys[i+5]] == 1 and \
                   self.cards[card_id][keys[i+10]] == 1 and \
                   self.cards[card_id][keys[i+15]] == 1 and \
                   self.cards[card_id][keys[i+20]] == 1:                       
                       return True
            i += 1        
               
        return False
     
    def print_card(self, card_id:int, last_num: int) -> None:
        """
        Start by finding the sum of all unmarked numbers on that board; in this
        case, the sum is 188. Then, multiply that sum by the number that was just
        called when the board won, 24, to get the final score, 188 * 24 = 4512. 
        """
        tmp = 0
        sum = 0
        
        # val[0]: key, val[1]: value
        for val in self.cards[card_id].items():
            print(f"{val[1]} ", end= "")            
            if val[1] == 0: sum += val[0]
            
            # print newline after 5 nums
            tmp += 1            
            if tmp % 5 == 0: print()
        
        # winning number:    
        print("Winning number:", sum * last_num) 

def main() -> int:
    # initialize bingo and load the data
    bingo = Bingo("input.txt")
    #bingo = Bingo("short.input")
    bingo.get_data()
    bingo.add_cards_to_dict()    
    amt_of_cards:int = bingo.get_amount_of_cards()    
    won_cards:list = []
    won_last:int = -1
    
    # loop through each card with each number and check for bingo
    for num in bingo.get_bingo_nums(): 
        for i in range (1, amt_of_cards+1):
            # don't recheck already won cards
            if i in won_cards:
                continue 
            
            # check only cards which have not won bingo yet     
            bingo.update_card(i, num)
            if bingo.check_card(i):
                if i not in won_cards:
                    won_cards.append(i)
                    won_last = i                                       
    
    print("card which won last:", won_last)
    print("amount of won cards:", len(won_cards))  
    bingo.print_card(won_last, num)        
    
    return 0    

from time import time
start = time()

main()
  
# record end time
end = time()

# print the difference between start
# and end time in milli. secs
print("The time of execution of above program is:",
    (end-start) * 10**3, "ms")   

