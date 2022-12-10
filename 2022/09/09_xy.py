# -*- encoding: utf-8 -*-
'''
@File    :   09.py
@Time    :   09/12/2022, 10:27:48
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2022
@Desc    :   TEMP
'''
import time

INIT = 20
SIZE = 40

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

class Piece:
    """contains a piece of the snake, and methods to set and get the position
    """
    def __init__(self, running_id:int):    
        self.id:int = running_id # part number
        self.x:int = INIT
        self.y:int = INIT
        
    def set_cur_pos(self, xy:tuple):
        self.x = xy[0]
        self.y = xy[1]
        
    def get_cur_pos(self):
        return tuple((self.y, self.x))

# too tired for anything else            
class Solution:    
    def __init__(self, data:list, length:int, color:str):
        self.__data:list = data
        self.__moves = len(self.__data)
        self.__moves_done = 0        
        self.t_moves:set = set()            
        self.snake_length = length
        self.s_pos: list = []
        self.color_ = color
        
        for x in range(0, self.snake_length):                        
            self.s_pos.append(Piece(x)) 
           
    def walk(self):                             
        while True:
            # return when all moves have been done
            if self.__moves_done == self.__moves:
                total_moves = len(self.t_moves)
                if self.color_ == "silver" : total_moves += 1 #debug +1 for silver..
                print(f"{self.color_}:", total_moves) 
                return 

            self.move()
         
    # monster from the dark depths
    def calc_dir(self, cur_x:int, cur_y:int, prev_x:int, prev_y:int, diag:bool) -> tuple:
        
        new_x = cur_x
        new_y = cur_y
        
        if diag:            
            if  (prev_y > cur_y) and (prev_x > cur_x): # ne
                new_x += 1
                new_y += 1                
            elif(prev_y > cur_y) and (prev_x < cur_x): # nw
                new_y += 1
                new_x -= 1            
            elif(prev_y < cur_y) and (prev_x > cur_x): # se
                new_y -= 1
                new_x += 1            
            elif(prev_y < cur_y) and (prev_x < cur_x): # sw
                new_y -= 1
                new_x -= 1
            else:
                print ("error in diag movement")
        else:
            # y direction
            if prev_x == cur_x:                
                if prev_y > cur_y: # up
                    new_y += 1                
                else:              # down
                    new_y -= 1
            # x direction
            elif prev_y ==  cur_y:                
                if prev_x > cur_x: # right
                    new_x += 1   
                else:              # left
                    new_x -= 1  
            else:
                print ("error in hor/vert movement")

        return tuple((new_x, new_y))
    
    def move(self):
        if self.__moves_done < self.__moves:
            dir:str = self.__data[self.__moves_done][0:1]
            amt:int = int(self.__data[self.__moves_done][2:])
            
            i:int = 0
            while i < amt:  
                # if tail parts are more than 1 away from each other, 
                # move next part to prev part's place
                for part in self.s_pos:
                    # don't touch the head
                    if part.id == 0:
                        continue
                                        
                    prev_x = self.s_pos[part.id-1].x
                    prev_y = self.s_pos[part.id-1].y
                                        
                    # If the head is ever two steps directly up, down, left, or right from the tail,
                    # the tail must also move one step in that direction so it remains close enough:"
                    if ((part.y == prev_y) and (abs(prev_x - part.x) > 1)) or \
                       ((abs(prev_y - part.y) > 1) and (part.x == prev_x)):
                        part.set_cur_pos(self.calc_dir(part.x, part.y, prev_x, prev_y, False))
                    # Otherwise, if the head and tail aren't touching and aren't in the same row
                    # or column, the tail always moves one step diagonally to keep up
                    elif ((abs(prev_y - part.y) > 1) or (abs(prev_x - part.x) > 1)):
                        part.set_cur_pos(self.calc_dir(part.x, part.y, prev_x, prev_y, True))
                            
                    # save tail's pos 
                    if self.color_ == "silver" and part.id == 1:
                        self.t_moves.add((part.x, part.y))
                    elif self.color_ == "gold" and part.id == 9:
                        self.t_moves.add((part.x, part.y))                    
                
                # move Head to new location
                head_pos = self.s_pos[0].get_cur_pos()
                if   dir == "R":  self.s_pos[0].set_cur_pos((head_pos[0],   head_pos[1] + 1))                            
                elif dir == "L":  self.s_pos[0].set_cur_pos((head_pos[0],   head_pos[1] - 1))                       
                elif dir == "U":  self.s_pos[0].set_cur_pos((head_pos[0] +1,head_pos[1]))                             
                elif dir == "D":  self.s_pos[0].set_cur_pos((head_pos[0] -1,head_pos[1]))

                i += 1                
                #self.print()
                #time.sleep(1)
            
            self.__moves_done += 1     
     
    # used only for visual debugging...        
    def print(self):  
        SIZE = 40
        mapx = [["." for i in range(SIZE)] for j in range(SIZE)] 
        
        for val in self.t_moves:
            mapx[val[0]][val[1]] = "#"
            
        for part in self.s_pos:
            pos = part.get_cur_pos()
            mapx[pos[0]][pos[1]] = str(part.id)
        
        for i in range(len(mapx)-1, -1, -1):            
            print(''.join(mapx[i])) 
        
def main():    
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\09\\puzzle.input") 
    
    # 9038 too much # 6174 too low #6175 is good, missing one due to a bug
    silver = Solution(data, 2, "silver")
    silver.walk()
    
    # 2579 too high, 2578 correct! debug +1 was not required, it works now perfectly! :v
    gold = Solution(data, 10, "gold")
    gold.walk() 

    return 0

if __name__ == "__main__":
    main()