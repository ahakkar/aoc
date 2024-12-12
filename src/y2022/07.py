# -*- encoding: utf-8 -*-
'''
@File    :   07.py
@Time    :   07/12/2022, 08:23:00
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2022
@Desc    :   This was not easy. I wanted to make first child/sibling tree
             but could not make it this time.
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
        """parses commands read from row,
        or directories and files listed with ls

        :param list data: list of commands and dir/file rows
        """
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
        """
        Just prints the root dir.
        Calls print_children which prints dirs & calculates their size recursively
        """
        type:str = ""          
        print("-", self.first.name, "(dir)") 
        self.print_children(self.first, 1)
        
  
    def print_children(self, node:object, depth:int) -> int:
        """ 
        This has an indent bug somewhere in print feature.

        :param object node: item object (dir or file)
        :param int depth: how deep we are in dir tree
        :return int: current dir's and all subdir's total size
        """
        # current directory size must contain current dir files and all subdir's files
        cur_dir_size: int = 0
        for val in node.children:  
            type:str = ""      
            if (val.type == 0):
                type = "(dir)"
            else:
                type = f"(file, size={val.size})"  
                cur_dir_size += int(val.size)
                          
            print("{}- {} {}".format('  ' * depth, val.name, type))
            
            # recursively look for subdirs and add their size
            if len(val.children) > 0:
                depth += 1
                cur_dir_size += self.print_children(val, depth)           
        
        # after all subdir sizes are summed up, add current dir's total size to list
        self.dir_sizes.append(cur_dir_size)
        
        # return the current dir size to parent, so parent can add it to it's total dir size
        return cur_dir_size
    
    # Finds the largest folder which is enough to get free space above 30 mil
    def find_gold(self, diff:int):
        for val in self.dir_sizes:
            if val > diff:
                return val
        
    def print_dir_sizes(self):
        
        self.dir_sizes.sort()
       
        # for silver, gather dirs below size 100000 to a sum
        total:int = 0               
        for val in self.dir_sizes:
            if val <= 100000:                
                total += val             
        
        #silver   1778099     
        print(total)  
        
        # Disk has 70 mil total space. Update needs 30 mil free space.
        # Figure out how much space total is used, and how much more space
        # must be required to get to 30 mil free space.   
        
        #gold   1623571     
        print(self.find_gold(30000000-(70000000-max(self.dir_sizes) )))                             
        
############################################        

def main():
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\07\\puzzle.input") 
    bash = Parser()
    bash.parse_data(data)    
    bash.traverse()
    bash.print_dir_sizes()
    
    return 0

#if __name__ == "__main__":
#    main()

import timeit    
time = timeit.timeit(main, number=1000)
print(f"{time*1000:.5f}ms")