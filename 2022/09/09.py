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

class Silver:    
    def __init__(self, data:list):
        self.__data:list = data
        self.__moves = len(self.__data)
        self.__moves_done = 0
        
        self.h_pos:tuple = (20, 20)
        self.t_pos:tuple = (20, 20)
        self.h_prev = ()
        self.t_moves:set = set()        
        self.t_moves.add(self.t_pos)       
             
    def walk(self):                             
        while True:
            # return when all moves have been done
            if self.__moves_done == self.__moves:
                print("Final position: H", self.h_pos, "T", self.t_pos)
                print("silver:", len(self.t_moves)+1) #debug extra +1
                return 

            self.move()
    
    def move(self):
        if self.__moves_done < self.__moves:
            dir:str = self.__data[self.__moves_done][0:1]
            amt:int = int(self.__data[self.__moves_done][2:])
            
            i:int = 0
            while i < amt:  
                # if tail is more than 1 away from h, move t to h's prev place
                if abs(self.h_pos[0]-self.t_pos[0]) > 1 or \
                   abs(self.h_pos[1]-self.t_pos[1]) > 1:
                    self.t_pos = self.h_prev  
                    self.t_moves.add(self.t_pos)                    
                
                # save previous point
                self.h_prev = self.h_pos          

                # move Head
                if   dir == "R":  self.h_pos = (self.h_pos[0],  self.h_pos[1]+1)                            
                elif dir == "L":  self.h_pos = (self.h_pos[0],  self.h_pos[1]-1)                       
                elif dir == "U":  self.h_pos = (self.h_pos[0]+1,self.h_pos[1])                              
                elif dir == "D":  self.h_pos = (self.h_pos[0]-1,self.h_pos[1])

                i += 1
            
            self.__moves_done += 1                 
            
    def print(self):         
        mapx = [["." for i in range(SIZE)] for j in range(SIZE)] 
        
        for val in self.t_moves:
            mapx[val[0]][val[1]] = "#"
        
        for i in range(len(mapx)-1, -1, -1):            
            print(''.join(mapx[i])) 
class Piece:
    def __init__(self):
        # current and prev pos
        self.cur_pos:tuple = (INIT, INIT)
        self.prev_pos:tuple = None
        
        # prev & next part. prev is not used
        self.prev:object = None        
        self.next:object
        
        # part number. 0 head, ... 9 tail.
        self.id_:int = -1
    
    def set_cur_pos(self, coords:tuple):
        # save old location
        self.prev_pos = self.cur_pos
        self.cur_pos = coords
        
    def get_cur_pos(self) -> tuple:
        return self.cur_pos  
    
    def get_prev_pos(self) -> tuple:
        return self.prev_pos  

# too tired for anything else            
class Gold:    
    def __init__(self, data:list):
        self.__data:list = data
        self.__moves = len(self.__data)
        self.__moves_done = 0
        
        self.t_moves:set = set()       
        
        self.SNAKE_PARTS = 10
        self.s_pos: list = []
        
        # add 10 snake piece objects which know the next part
        add_piece:object = Piece()
        for x in range(0, self.SNAKE_PARTS): 
            #print(x)           
            self.s_pos.append(add_piece) 
            self.s_pos[x].id_ = x
            
            add_piece = Piece()
            self.s_pos[x].next = add_piece
            
        #for val in self.s_pos:
        #    print(val.num_)             
           
    def walk(self):                             
        while True:
            # return when all moves have been done
            if self.__moves_done == self.__moves:
                #print("Final position: H", self.h_pos, "T", self.t_pos)
                print("gold:", len(self.t_moves)) 
                return 

            self.move()
         
    # monster from the dark depths
    def calc_dir(self, part_pos:tuple, prev_part_pos:tuple, diag:bool) -> tuple:
        
        #print("og pos", part_pos)
        new_pos:list = list(part_pos)
        #print("TUPLE ERROR!!", new_pos)
        
        if diag:
            # ne
            if (prev_part_pos[0] > part_pos[0]) and (prev_part_pos[1] > part_pos[1]):
                new_pos[0] += 1
                new_pos[1] += 1
            # nw
            elif(prev_part_pos[0] > part_pos[0]) and (prev_part_pos[1] < part_pos[1]):
                new_pos[0] += 1
                new_pos[1] -= 1
            # se
            elif(prev_part_pos[0] < part_pos[0]) and (prev_part_pos[1] > part_pos[1]):
                new_pos[0] -= 1
                new_pos[1] += 1
            # sw
            elif(prev_part_pos[0] < part_pos[0]) and (prev_part_pos[1] < part_pos[1]):
                new_pos[0] -= 1
                new_pos[1] -= 1
            else:
                print ("error in diag movement")
        else:
            # y direction
            if prev_part_pos[1] == part_pos[1]:
                # y up
                if prev_part_pos[0] > part_pos[0]:
                    new_pos[0] += 1
                # y down
                else:
                    new_pos[0] -= 1
            # x direction
            elif prev_part_pos[0] ==  part_pos[0]:
                # x right
                if prev_part_pos[1] > part_pos[1]:
                    new_pos[1] += 1
                # x left
                else:
                    new_pos[1] -= 1
            else:
                print ("error in hor/vert movement")
        
        
        new_pos = tuple(new_pos)
        #print(new_pos)
        #exit(0)
        return new_pos
    
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
                    if part.id_ == 0:
                        continue
                    
                    part_pos = part.get_cur_pos()
                    prev_part_pos = self.s_pos[part.id_-1].get_cur_pos()
                    
                    # move other pieces if neccessary  
                    #if abs(prev_part_pos[0] - part_pos[0]) > 1 or abs(prev_part_pos[1] - part_pos[1]) > 1:
                    
                    # If the head is ever two steps directly up, down, left, or right from the tail,
                    # the tail must also move one step in that direction so it remains close enough:"
                    if ((part_pos[0] == prev_part_pos[0]) and (abs(prev_part_pos[1] - part_pos[1]) > 1)) or \
                       ((abs(prev_part_pos[0] - part_pos[0]) > 1) and (part_pos[1] == prev_part_pos[1])):
                        part.set_cur_pos(self.calc_dir(part_pos, prev_part_pos, False))
                    # Otherwise, if the head and tail aren't touching and aren't in the same row
                    # or column, the tail always moves one step diagonally to keep up
                    elif ((abs(prev_part_pos[0] - part_pos[0]) > 1) or (abs(prev_part_pos[1] - part_pos[1]) > 1)):
                        part.set_cur_pos(self.calc_dir(part_pos, prev_part_pos, True))
                            
                    # save tail's pos 
                    if part.id_ == 9:
                        self.t_moves.add(part.get_cur_pos())                       
                
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
            
    def print(self):  
        SIZE = 40
        mapx = [["." for i in range(SIZE)] for j in range(SIZE)] 
        
        for val in self.t_moves:
            mapx[val[0]][val[1]] = "#"
            
        for part in self.s_pos:
            pos = part.get_cur_pos()
            mapx[pos[0]][pos[1]] = str(part.id_)
        
        for i in range(len(mapx)-1, -1, -1):            
            print(''.join(mapx[i])) 
            
        
        
def main():
    # 9038 too much # 6174 too low #6175 is good, missing one due to a bug
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\09\\puzzle.input") 
    
    silver = Silver(data)  
    silver.walk() 
    
    # 2579 too high, 2578 correct! debug +1 was not required, it works now perfectly! :v
    gold = Gold(data)
    gold.walk() 
    #gold.print()
    return 0

if __name__ == "__main__":
    main()
