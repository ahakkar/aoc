# -*- encoding: utf-8 -*-
'''
@File    :   11.py
@Time    :   11/12/2022, 14:40:53
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2022
@Desc    :   TEMP
'''

import math
from collections import deque
class Jungle(object):
    def __init__(self):
        self.__monkeys:list = []        
        
    def add_monkey(self, id:int):
        self.__monkeys.append(Monkey(id))
    
    def add_item(self, selected_monkey:int, item_id:int):
        self.give_item_to_monkey((Item(item_id), selected_monkey))
        
    def play_keep_away(self):
        for monkey in self.__monkeys: 
            for _ in range(0, len(monkey.items())):         
                give = monkey.inspect_item()    
                #print(f"give item {give[0].worry_level()} to monkey {give[1]}")        
                self.give_item_to_monkey(give)
                
    def give_item_to_monkey(self, item_and_monkey_id:tuple):
        #print(item_and_monkey_id)
        monkey = self.find_monkey(item_and_monkey_id[1])        
        monkey.catch_item(item_and_monkey_id[0])
    
    def find_monkey(self, monkey_id) -> object:
        #print("trying to find monkey", monkey_id, type(monkey_id))
        for monkey in self.__monkeys:
            if monkey.id == int(monkey_id):
                return monkey
            
        print("vituiks meni, apinaa ei löytynyt")
        exit(1)
    
    def list_monkeys(self):
        print("Current list of animals living in the Jungle:\n")
        for monkey in self.__monkeys:
            if len(monkey.items()) > 0:
                print(f"Monkey {monkey.id} has the following items:")
                monkey.list_items()
            else:
                print(f"Monkey {monkey.id} has no items.")            
            monkey.list_properties()
            print()
            
    def list_inspections(self):
        insp:list = []
        for monkey in self.__monkeys:
            insp.append((monkey.id, monkey.inspections()))
            
        return insp
            
    def set_monkey_property(self, selected_monkey, oper:str, value:str, pos:int = -1):
        monkey = self.find_monkey(selected_monkey) 
        monkey.set_property(oper, value, pos)       
        
class Monkey(object):
    def __init__(self, given_id:int) -> None:
        self.id:int = int(given_id)
        self.__items: deque = deque([])
        self.__operation:str = ""
        self.__test:list = [-1, -1, -1] # 0: divisible, 1: true monkey, 2: false monkey
        self.__items_inspected:int = 0
        
    def catch_item(self, item:object):
        print("Monkey", self.id, "caught item", item.worry_level())
        self.__items.append(item)
        
    def items(self):
        return self.__items
        
    def inspect_item(self) -> int:
        item = self.__items.popleft()
        #print("Monkey:", self.id)
        operator = self.__operation[10:11] 
        factor = self.__operation[12:]
        
        #print(operator, factor)
                
        current = item.worry_level()
        #print("item before inspection:", current)
        
        if operator == "*":
            if factor == "old":
                current *= current
            else:                
                current *= int(factor)
        elif operator == "+":
            if factor == "old":
                current += current
            else:                
                current += int(factor)
            
        #print("item after inspection:", current)
        current = math.floor(current/3)
        #print("monkey gets bored, divided by 3:", current)            
          
        item.set_worry_level(current)
        self.__items_inspected += 1        

        return (item, self.test(item))
            
    def inspections(self):
        return self.__items_inspected
    
    # 0: divisible, 1: true monkey, 2: false monkey    
    def test(self, item:object) -> None:
        wl = item.worry_level()        
        if wl%self.__test[0] == 0:
            return int(self.__test[1])
        return int(self.__test[2])            
    
    def catch_item(self, item:object) -> None:
        self.__items.append(item)
        
    def list_items(self):
        item_list:str = "  "
        for item in self.__items:            
            item_list += str(item.worry_level()) + ", "
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
        self.__worry_level:int = int(initial_worry_level)
        
    def set_worry_level(self, worry_level):
        self.__worry_level = int(worry_level)
        
    def worry_level(self):
        return self.__worry_level
        
class Simulation(object):
    def __init__(self):
        self.__data:list = []
        self.__sim:object
        self.__turn:int = 0
    
    def read_file(self, filename:str) -> list:        
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
        
        print("Monkey status at turn 0:")     
        self.__sim.list_monkeys()
        
    def advance_turn(self):
        self.__turn += 1
        print("Simulating turn", self.__turn)
        self.__sim.play_keep_away()
        #self.__sim.list_monkeys()
        
    def level_of_monkey_business(self):
        insp = self.__sim.list_inspections()
        most_active_monkeys:list = []
        for val in insp:
            most_active_monkeys.append(val[1])
        
        most_active_monkeys.sort(reverse=True)
        
        print(f"The level of monkey business after {self.__turn} rounds is:")
        print(most_active_monkeys[0] * most_active_monkeys[1])
        
                
            
def main():
    sim = Simulation()
    sim.read_file("D:\\GDrive\\Prog\\aoc\\2022\\11\\puzzle.input") 
    sim.create_jungle()
    for _ in range(0, 20):
        sim.advance_turn()
    sim.level_of_monkey_business()    

    return 0

if __name__ == "__main__":
    main()
