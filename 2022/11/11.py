# -*- encoding: utf-8 -*-
'''
@File    :   11.py
@Time    :   11/12/2022, 14:40:53
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2022
@Desc    :   TEMP
'''
class Jungle(object):
    def __init__(self):
        self.__monkeys:list = []        
        
    def add_monkey(self, id:int):
        self.__monkeys.append(Monkey(id))
    
    def add_item(self, selected_monkey:int, item_id:int):
        self.give_item_to_monkey(selected_monkey, Item(item_id))
    
    def give_item_to_monkey(self, selected_monkey:int, item:object):
        monkey = self.find_monkey(selected_monkey)
        monkey.catch_item(item)
        
    def turn(self):
        pass
    
    def find_monkey(self, monkey_id) -> object:
        for monkey in self.__monkeys:
            if monkey.id == monkey_id:
                return monkey
    
    def list_monkeys(self):
        print("Current list of animals living in the Jungle:\n")
        for monkey in self.__monkeys:
            print(f"Monkey {monkey.id} has the following items:")
            monkey.list_items()
            monkey.list_properties()
            print()
            
    def set_monkey_property(self, selected_monkey, oper:str, value:str, pos:int = -1):
        monkey = self.find_monkey(selected_monkey) 
        monkey.set_property(oper, value, pos)       
        
class Monkey(object):
    def __init__(self, given_id:int) -> None:
        self.id:int = given_id
        self.__items: list = []
        self.__operation:str = ""
        self.__test:list = [-1, -1, -1] # 0: divisible, 1: true monkey, 2: false monkey
        
    def inspect(self) -> None:
        pass
        
    def test(self) -> None:
        pass
    
    def throw(self) -> None:
        pass
    
    def catch_item(self, item:object) -> None:
        self.__items.append(item)
        
    def list_items(self):
        item_list:str = "  "
        for item in self.__items:            
            item_list += item.worry_level + ", "
        print(item_list[:-2])              
            
    def list_properties(self):
        print("  and following properties:")
        print(f"    {self.__operation}")
        print(f"    {self.__test}")
            
    def set_property(self, prop:str, value:str, pos:int):
        if prop == "oper":
            self.__operation = value.strip()
            #print(self.__operation)
        elif prop == "test":
            self.__test[pos] = int(value.strip())            
        
class Item(object):
    def __init__(self, initial_worry_level:int) -> None:
        #self.id:int = -1
        self.worry_level:int = initial_worry_level
        
class Simulation(object):
    def __init__(self):
        self.__data:list = []
        self.__sim:object
    
    def read_file(self, filename:str) -> list:
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
            
        self.__data = data
    
    def create_jungle(self):
        self.__sim = Jungle()
        
        selected_monkey = -1
        
        for row in self.__data:
            if row[0:6] == "Monkey":
                selected_monkey = row[7:].rstrip(":")
                self.__sim.add_monkey(selected_monkey)
            if row[2:16] == "Starting items":
                for item in row[17:].split(", "):                    
                    self.__sim.add_item(selected_monkey, item.strip())    
            if row[2:11] == "Operation":
                self.__sim.set_monkey_property(selected_monkey, "oper", row[12:])  
            if row[2:6] == "Test":
                self.__sim.set_monkey_property(selected_monkey, "test", row[20:], 0) 
            if row[4:11] == "If true":
                self.__sim.set_monkey_property(selected_monkey, "test", row[29:], 1)
            if row[4:12] == "If false":
                self.__sim.set_monkey_property(selected_monkey, "test", row[30:], 2)
             
        self.__sim.list_monkeys()
                
            
def main():
    sim = Simulation()
    sim.read_file("D:\\GDrive\\Prog\\aoc\\2022\\11\\simple.input") 
    sim.create_jungle()
    
    jungle = Jungle()
    return 0

if __name__ == "__main__":
    main()
