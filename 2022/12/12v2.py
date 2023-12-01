# -*- encoding: utf-8 -*-
'''
@File    :   12.py
@Time    :   12/12/2022, 10:23:56
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2022
@Desc    :   TEMP
'''

from collections import deque

def read_file(filename:str) -> list:
    """
    param : str, filename to read
    return: list, lines
    """
    
    data:list = []
    
    try:
        with open (filename, "r") as read_file:
            data = read_file.read().splitlines()
        read_file.close()
    except FileNotFoundError:
        print(f"Bad file name! {filename}")
        exit()
    except Exception:
        print("SOS")
        exit()
        
    return data;

class Node:
    def __init__(self, y, x, dist):
        self.y = y
        self.x = x
        self.dist = dist
        
class Grid:
    def __init__(self, data):
        self.heightmap = []
        self.generate_heightmap(data)
        self.width = len(self.heightmap[0])
        self.height = len(self.heightmap)
        #self.print_map(self.heightmap)
        
        self.start = self.find_loc("S")
        self.goal = self.find_loc("E")
        
    def find_loc(self,  search):    
        for y in range(0, self.height):
            for x in range(0, self.width):
                #print(y, x)
                if self.heightmap[y][x] == search: 
                    return Node(y, x, 0)
                
    def is_index_valid(self, list, index):        
        if -1 < index < len(list): 
            return True
        return False
    
    def generate_heightmap(self, data):
        for row in data:
            self.heightmap.append([*row])    
              
    def print_map(self, heightmap):
        for row in heightmap:
            print(row)
            
    def bfs(self, start):
        queue = deque([start])
        seen = set(start)
        print(queue, seen)
        print(type(queue), print(type(queue[0])))
        while queue:
            path = queue.popleft()
            print("path:", path)
            y = path[0]
            x = path[1]
            if self.heightmap[y][x] == "E":
                return path
            for x2, y2 in ((x+1,y), (x-1,y), (x,y+1), (x,y-1)):
                if 0 <= x2 < self.width and 0 <= y2 < self.height and (x2, y2) not in seen:
                    queue.append(path + [(x2, y2)])
                    seen.add((x2, y2))
            
    def visited(self):
        #visited = [[False for _ in range(len(heightmap[0]))] for _ in range(len(heightmap))]
        #visited[source.x][source.y] = True
        pass

def main():    
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\12\\simple.input") 
    src = Grid(data)

    print("S coords:", src.start.x, src.start.y)
    print("E coords:", src.goal.x, src.goal.y)
    print("map size:", src.width, src.height)
    
    src.bfs(tuple((src.start.y, src.start.x)))
    

    
    #queue = [source]

    is_index_valid = lambda list, index: -1 < index < len(list)
    


    return 0

if __name__ == "__main__":
    main()