# -*- encoding: utf-8 -*-
'''
@File    :   aoc08.py
@Time    :   09/11/2022, 21:20:53
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2021
@Desc    :   TEMP
'''
class Display:
    
    def __init__(self, filename:str):
        """
        param : TODO
        return: none
        """
        self.data:list = []
        self.read_file(filename)
        
        
    def read_file(self, filename:str) -> None:
        """
        param : str, filename to read
        return: None
        """
        
        try:
            with open (filename, "r") as read_file:
                self.data: list = read_file.read().splitlines()
            read_file.close()
        except OSError:
            print("Bad file name!")
            exit()
            
    def count_easy(self) -> int:
        """
        param : TODO
        return: none
        """
        count:int = 0
        
        for row in self.data:
            parts = [x.strip().split(" ") for x in row.split('|')]
            #print(parts[1])
            for str in parts[1]:
                if len(str) in (2,4,3,7):
                    count += 1
                    
        return count
    
    def count_hard(self) -> int:
        """        
        return: none
        """
        count:int = 0
                       
        for row in self.data:
            parts = [x.strip().split(" ") for x in row.split('|')]
            
            keys:dict = self.solve_keys(parts[0])
            print(keys)                
            
            break
     
        return count
    
    def solve_keys(self, row:list) -> dict:
        """
        1 = be
        8 = abcdefg
        4 = bceg

        0/6/9 = bcdefg 9?
        0/6/9 = acdefg 6?
        0/6/9 = abdefg 0? (koska c puuttuu)

        2/3/5 = cdefg 5?
        2/3/5 = bcdef 3?
        2/3/5 = abcdf 2?

        7 = bde
        
         dddd
        g    b
        g    b
        cccc  
        a    e
        a    e
         ffff

        :param list row: _description_
        :return dict: _description_
        """
        keys:dict = {}
        set_069 = set()
        set_235 = set()
            
        # sort string letters
        for str in row:
            str = "".join([x for x in sorted(str)])
            
            if len(str) == 2:
                keys[1] = str
            elif len(str) == 4:
                keys[4] = str
            elif len(str) == 3:
                keys[7] = str
            elif len(str) == 7:
                keys[8] = str
            elif len(str) == 6:  
                set_069.add(str) 
            elif len(str) == 5:
                set_235.add(str)
            else:
                print("!?!?!") 
        
       # use set() difference() to get diff between 069 and 235
        print(set_069, set_235)
        for val in set_235:
            print(val)
        
        exit()       
        return keys
        

def main():
    #d = Display("simple.input")
    #total = d.count_hard()
    """
    if total != 26:
        print(f"wrong amount! ({total})")
    else:
    """
    #print("nums:", total)
    
    # {1: 'be', 8: 'abcdefg', '069': 'abdefg', 4: 'bceg', '235': 'abcdf', 7: 'bde'}
    
    setti = {'cdefg', 'abcdf', 'bcdef'}
    setti2 = {'bcdefg', 'abdefg', 'acdefg'}
    
    keys = {1: 'be'}
    
    one_ch = {}
    for ch in keys[1]:
        one_ch[ch] = 1
    
    five_ch = {}
    for val in setti2:
        for ch in val:
            if ch in five_ch:
                five_ch[ch] += 1
            else:
                five_ch.update({ch: 1})
                
    six_ch = {}
    for val in setti:
        for ch in val:
            if ch in six_ch:
                six_ch[ch] += 1
            else:
                six_ch.update({ch: 1})
    
    # Vitosessa on yks sama kun 0/6/9.
    # Kakkosessa puuttuu toinen joka on ykkösessä, jäljelle jää 3.
    
    # figure out value 5
    for key, val in six_ch.items():              
        if val == 1 and key in five_ch:            
            if five_ch[key] == 3:
                print("jee löytyi 5", key)
                for s in setti:
                    if s.find(key) >= 0:                       
                        keys[5] = s # this is no 5 str                             
    setti.remove(keys[5]) 
    
    # figure out value 3
    for val in setti:
        hits = 0
        for ch in val:
            if ch in one_ch:
                hits += 1
        if hits == 2:
            keys[3] = val # this is no 3 str            
    
    # 2 is the remaining one
    setti.remove(keys[3]) 
    keys[2] = setti.pop() # pop() returns last item from set
    
    # Nollasta puuttuu yks mikä on 6/9 molemmilla.
    # Kuutosessa puuttuu toinen mikä on ykkösessä. Ysi jää jäljelle.
    print(setti2)
    print(six_ch)
    
    diff = set('bcdefg').difference(set("abdefg"))
    for val in setti2:
        
    
        
    print(keys)              

            
    
    
    


main()

# one run takes 2855ms with 12700K..
#import timeit    
#time = timeit.timeit(main, number=1)
#print(f"{time*1000:.5f}ms")
