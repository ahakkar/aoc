# -*- encoding: utf-8 -*-
'''
@File    :   12.py
@Time    :   12/12/2022, 10:23:56
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
        
def print_map(heightmap):
    for row in heightmap:
        print(row)

def find_src(source, heightmap):
    # Find the Source "S"
    for y in range(len(heightmap)):
        for x in range(len(heightmap[y])):
            if heightmap[y][x] == 'S':
                print("src fnd")
                source.y = y
                source.x = x
                return source

def main():    
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\12\\simple.input") 
    heightmap:list = []
    for row in data:
        heightmap.append([*row])
        
    source = Node(0, 0, 0)
    source = find_src(source, heightmap)
    #print(source.x, source.y)
    
    visited = [[False for _ in range(len(heightmap[0]))] for _ in range(len(heightmap))]
    visited[source.x][source.y] = True
    
    queue = [source]

    is_index_valid = lambda list, index: -1 < index < len(list)
    
    while len(queue) > 0:
        source = queue.pop(0)
        y = source.y
        x = source.x
        dist = source.dist

        if (heightmap[y][x] == 'E'):
            print(source.dist)
            break
        
        # moving up
        if is_index_valid(heightmap, y-1) and visited[y][x]:
            queue.append(Node(y - 1, x, dist + 1))
            visited[y - 1][x] = True
 
        # moving down
        if is_index_valid(heightmap, y+1) and visited[y][x]:
            queue.append(Node(y + 1, x, dist + 1))
            visited[y + 1][x] = True
 
        # moving left
        if is_index_valid(heightmap[y], x-1) and visited[y][x]:
            queue.append(Node(y, x-1, dist + 1))
            visited[y][x-1] = True
 
        # moving right
        if is_index_valid(heightmap[y], x+1) and visited[y][x]:
            queue.append(Node(y, x+1, dist + 1))
            visited[y][x+1] = True
        
        #for i, j in [(0,1), (0,-1), (1,0), (-1,0)]:
        #    if is_index_valid(heightmap, y+i) and is_index_valid(heightmap[y+i], x+j):
        #        if not_visited[y][x]:
        #            queue.append(Node_item(source.y + i, source.x + j, source.dist + 1))
        #            not_visited[y+i][x+j] = False      

    return 0

if __name__ == "__main__":
    main()