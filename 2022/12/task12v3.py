"""
Anew go with A* algorithm learned from Tiraka1 in fall 2023
"""

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
Node = namedtuple("Node", ["coord", "height", "routes", "g", "h", "f"])

def main():
    """_summary_ runs the program
    """
    graph = Graph()
    graph.create_graph_from_2d_array("simple.input")
    graph.printGraph()

class Graph:
    """_summary_ Graph class for A* algorithm
    """
    def __init__(self):
        self._start = None  # Initialize start
        self._end = None    # Initialize end
        self._max_width = 0
        self._max_height = 0
        self._graph:dict[tuple, tuple] = {}
        
    def printGraph(self):
        """_summary_ prints the graph
        """
        for key, value in self._graph.items():
            print(key, value)
        
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
                    self._start = (x, y)
                    char = "a"
                elif char == "E":
                    self._end = (x, y)
                    char = "z"

                self._graph[(y, x)] = (ord(char) - 96, [])
       
        for key, value in self._graph.items():
            #print(key, value)
            adjacentPositions = self.getAdjacentPositions(key)
            for pos in adjacentPositions:
                #print(f"height at key{key}: {value[0]}, height at pos {pos} {graph[pos][0]}")
                if self.isValidEdge(value[0], self._graph[pos][0]):      
                    value[1].append(pos)     

    def getAdjacentPositions(self, key:tuple) -> list:
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
            new_x, new_y = key[0] + dx, key[1] + dy
            
            # boundary check
            if 0 <= new_x < self._max_height and 0 <= new_y < self._max_width:
                positions.append((new_x, new_y))
        
        
        return positions

    def isValidEdge(self, h1:int, h2:int) -> bool:
        if h1 == h2:    # same height
            return True
        if h1+1 == h2:  # one step up
            return True
        if h2 > h1:     # any number of steps down
            return True
        return False
    
    def manhattan_distance(self, x1:int, y1:int, x2:int, y2:int) -> int:
        return abs(x1 - x2) + abs(y1 - y2)


if __name__ == "__main__":
    main()
