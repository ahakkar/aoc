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
Node = namedtuple("Node", ["coord", "height", "routes", "state", "h", "f"])


def main():
    """_summary_ runs the program
    """
    graph = Graph()
    graph.create_graph_from_2d_array("simple.input")
    #graph.printGraph()
    

def manhattan_distance(a:Coord, b:Coord) -> int:
    return abs(a.x - b.x) + abs(a.y - b.y)

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

                coord = Coord(x,y)
                node = Node(coord, ord(char) - 96, [], NodeState.UNVISITED, 0, 0)
                self._graph[coord] = node
       
        for key, value in self._graph.items():
            #print(key, value)
            adjacentPositions = self.getAdjacentPositions(key)
            for pos in adjacentPositions:
                #print(f"height at key{key}: {value[0]}, height at pos {pos} {graph[pos][0]}")
                if self.isValidEdge(value.height, self._graph[pos].height):      
                    value.routes.append(pos)     


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
        if h2 > h1:     # any number of steps down
            return True
        return False  
    


if __name__ == "__main__":
    main()
