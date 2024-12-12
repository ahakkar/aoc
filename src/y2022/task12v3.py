"""
Anew go with A* algorithm learned from Tiraka1 in fall 2023
"""

import heapq # priority queue
from enum import Enum, auto
from collections import namedtuple

# possible states of a node for pathfinding
class NodeState(Enum):
    """_summary_ Enum for node states

    :param _type_ Enum: _description_
    """
    UNVISITED = auto()
    VISITED = auto()
    EXPLORED = auto()
    
Coord = namedtuple("Coord", ["x", "y"])

class Node:
    def __init__(self, coord, height):
        self.coord = coord
        self.height = height
        self.parent = None
        self.routes = []  # Adjacent nodes
        self.state = NodeState.UNVISITED
        self.g = float('inf') # cost of the cheapest path from the start node to n currently known
        self.h = 0            # heuristic cost estimate from n to the goal
        self.f = float('inf') # current best guess as to how short a path from start to finish can be if it goes through n
    
    def __lt__(self, other):
        return self.f < other.f  # This will allow us to compare two nodes based on f score

    # pretty print
    def __str__(self) -> str:
        return f"Node: {self.coord} height: {self.height} routes: {self.routes}"


def main():
    """_summary_ runs the program
    """
    graph = Graph()
    graph.create_graph_from_2d_array("puzzle.input")
    start = graph.getStartCoord()
    end = graph.getEndCoord()
    
    #graph.printGraph()
    
    # use Coord (0,12) for part B (eyeballed to be the closest a coordinate)
    path:list = a_star_search(graph.getGraph(), start, end, manhattan_distance)
    print(f"Path: {len(path)-1}")
    #print(path)
    
    

def manhattan_distance(a:Coord, b:Coord) -> int:
    return abs(a.x - b.x) + abs(a.y - b.y)


def a_star_search(graph, start: Coord, goal: Coord, heuristic) -> list:
    open_set:list = []
    # Priority queue, sorted by f score, with start node
    heapq.heappush(open_set, (graph[start].f, start))  

    graph[start].g = 0
    graph[start].f = heuristic(start, goal)

    while open_set:
        # Take the node with lowest f score
        current_f, current_coord = heapq.heappop(open_set)
        current_node = graph[current_coord]
        
        if current_coord == goal:
            return reconstruct_path(graph, start, goal)
        
        current_node.state = NodeState.EXPLORED

        for neighbor_coord in current_node.routes:
            neighbor_node = graph[neighbor_coord]
            if neighbor_node.state == NodeState.EXPLORED:
                continue
            
            # Assuming each step to a neighbor costs 1
            tentative_g_score = current_node.g + 1  

            if tentative_g_score < neighbor_node.g:
                neighbor_node.parent = current_coord
                neighbor_node.g = tentative_g_score
                neighbor_node.h = heuristic(neighbor_coord, goal)
                neighbor_node.f = neighbor_node.g + neighbor_node.h

                if neighbor_node.state == NodeState.UNVISITED:
                    heapq.heappush(open_set, (neighbor_node.f, neighbor_coord))
                    neighbor_node.state = NodeState.VISITED

    # If the open_set is empty and goal was not reached, return None
    return None  


def reconstruct_path(graph, start: Coord, goal: Coord) -> list:
    path = []
    current = goal
    while current != start:
        path.append(current)
        current = graph[current].parent
    path.append(start)
    path.reverse()  # Reverse the path to go from start to goal
    return path


class Graph:
    """_summary_ Graph class for A* algorithm
    """
    def __init__(self):
        self._start = None  # Initialize start
        self._end = None    # Initialize end
        self._max_width = 0
        self._max_height = 0
        self._graph:dict[Coord, Node] = {}
        
        
    def printGraph(self):
        """_summary_ prints the graph
        """
        for key, value in self._graph.items():
            print(key, value)
          
            
    def getGraph(self) -> dict[tuple, tuple]:
        """_summary_ returns the graph

        :return dict[tuple, tuple]: _description_
        """
        return self._graph
    
    
    def getStartCoord(self) -> Coord:
        """_summary_ returns the start coordinate

        :return tuple: _description_
        """
        return self._start
    
    def getEndCoord(self) -> Coord:
        """_summary_ returns the end coordinate

        :return tuple: _description_
        """
        return self._end
        
        
    def read_file(self, filename: str) -> list:
        """
        Reads lines from a given filename.
        """
        try:
            with open(filename, "r", encoding="utf-8") as read_file:
                return read_file.read().splitlines()
        except FileNotFoundError:
            print(f"Bad file name! {filename}")

        return []


    def create_graph_from_2d_array(self, filename: str):
        """_summary_ creates a graph from a 2d array

        :param str filename: _description_
        """
        data = self.read_file(filename)
        self._max_height = len(data)
        self._max_width = len(data[0])        

        for y, row in enumerate(data):
            for x, char in enumerate(row):
                if char == "S":
                    self._start = Coord(x, y)
                    char = "a"
                elif char == "E":
                    self._end = Coord(x, y)
                    char = "z"

                coord = Coord(x,y)
                node = Node(coord,  ord(char) - 96)
                self._graph[coord] = node
       
        for key, value in self._graph.items():
            #print(key, value)
            adjacentPositions = self.getAdjacentPositions(key)
            for pos in adjacentPositions:
                #print(f"height at key{key}: {value[0]}, height at pos {pos} {graph[pos][0]}")
                if self.isValidEdge(value.height, self._graph[pos].height):      
                    value.routes.append(pos)     
                    
        # set start node's g&f score to 0
        #self._graph[self._start].g = 0
        #self._graph[self._start].f = 0


    def getAdjacentPositions(self, coord:Coord) -> list:
        """_summary_
        Returns a list of valid positions adjacent to the given 
        position, checking for boundaries

        :param tuple key: _description_
        :return list: _description_
        """
        positions:list = []
        
        # legal directions
        moves = [(0, -1), (0, 1), (-1, 0), (1, 0)]
        
        for dx, dy in moves:
            newCoord = Coord(coord.x + dx, coord.y + dy)   
            # boundary check
            if 0 <= newCoord.x < self._max_width and 0 <= newCoord.y < self._max_height:
                positions.append(newCoord)
        
        
        return positions


    def isValidEdge(self, h1:int, h2:int) -> bool:
        if h1 == h2:    # same height
            return True
        if h1+1 == h2:  # one step up
            return True
        if h2 < h1:     # any number of steps down
            return True
        return False  
    
    def isValidEdge2(self, h1: int, h2: int) -> bool:
        # Allow any number of steps up
        if h2 >= h1:
            return True
        # Allow at most one step down
        if h1 - h2 == 1:
            return True
        # Disallow more than one step down
        return False
    


if __name__ == "__main__":
    import timeit    
    time = timeit.timeit(main, number=1)
    print(f"{time*1000:.5f}ms")    
