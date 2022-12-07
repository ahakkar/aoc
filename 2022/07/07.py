# -*- encoding: utf-8 -*-
'''
@File    :   07.py
@Time    :   07/12/2022, 08:23:00
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2022
@Desc    :   This was not easy
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

############################################   
class Node(object):
    def __init__(self, type:int, name:str, size:int = -1):
        self.type:int = type # 0 folder, 1 file
        self.name:str = name
        self.size:int = size
        self.parent:object = None;
        self.children:list = [];
        
    def add_parent(self, parent:object):
        self.parent = parent
    
    def add_child(self, child:object):
        self.children.append(child)

############################################     
class Parser():
    def __init__(self):
        self.first:object
        self.cur:object
        self.data:list = [] 
        self.dir_sizes:list = []
 
    def parse_data(self, data:list):
        for row in data:
            params = row.split(' ')   
             
            #commands
            if params[0] == '$':
                # change dir
                if params[1] == 'cd':
                    # root dir
                    if params[2] == '/':
                        self.first = Node(0, "/") # this should happen always first
                        self.cur = self.first
                        #print("create top node")
                        
                    # one dir up
                    elif params[2] == '..':
                        #print(self.cur.name)
                        if self.cur != self.first:
                            self.cur = self.cur.parent
                        
                    # one dir lower
                    else:
                        for val in self.cur.children:
                            if val.name == params[2]:
                                val.parent = self.cur
                                self.cur = val
                # list dir contents (does nothing here)
                elif params[1] == 'ls':
                    pass
                
            # list of data, first folder
            elif params[0] == 'dir':
                self.cur.add_child(Node(0, params[1]))
                #print("add dir", params[1])
            
            # otherwise should be files
            else:
                self.cur.add_child(Node(1, params[1], params[0]))
                #print("add file", params[1], params[0])                

    def traverse(self):
        type:str = ""          
        print("-", self.first.name, "(dir)") 
        self.print_children(self.first, 1)
        
    # this has indent bug somewhere
    def print_children(self, node:object, depth:int) -> int:
        cur_dir_size: int = 0
        for val in node.children:  
            type:str = ""      
            if (val.type == 0):
                type = "(dir)"
            else:
                type = f"(file, size={val.size})"  
                cur_dir_size += int(val.size)
                          
            #print("{}- {} {}".format('  ' * depth, val.name, type))
            
            if len(val.children) > 0:
                depth += 1
                cur_dir_size += self.print_children(val, depth)           
        
        self.dir_sizes.append(cur_dir_size)
        
        return cur_dir_size
    
    # i can 't think staright anymore so i made this func to find
    # and return the first value large enough
    def find_gold(self, diff:int):
        for val in self.dir_sizes:
            if val > diff:
                return val
        
    def print_dir_sizes(self):
        
        self.dir_sizes.sort()
       
        # for silver, gather dirs below size 100000
        total:int = 0               
        for val in self.dir_sizes:
            if val <= 100000:                
                total += val             
        
        #silver   1778099     
        print(total)  
        
        # for gold, find the smallest dir above 70 mil - largest value
        diff:int = 70000000-max(self.dir_sizes)        
        
        #gold   1623571     
        print(self.find_gold(30000000-diff))                             
        
############################################        

def main():
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\07\\puzzle.input") 
    bash = Parser()
    bash.parse_data(data)    
    bash.traverse()
    bash.print_dir_sizes()
    
    return 0

if __name__ == "__main__":
    main()
