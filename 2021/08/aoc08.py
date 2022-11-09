# -*- encoding: utf-8 -*-
'''
@File    :   aoc08.py
@Time    :   09/11/2022, 21:20:53
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2021
@Desc    :   TEMP
'''


# -*- encoding: utf-8 -*-
'''
@File    :   aoc07.py
@Time    :   09/11/2022, 18:46:33
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
        param : use set() difference() to get diff between 069 and 235
        return: none
        """
        count:int = 0
        
        for row in self.data:
            parts = [x.strip().split(" ") for x in row.split('|')]
            #print(parts[1])
            for str in parts[0]:
                # sort string letters
                str = "".join([x for x in sorted(str)])
                if len(str) == 2:
                    print(f"1 = {str}")
                elif len(str) == 4:
                    print(f"4 = {str}")
                elif len(str) == 3:
                    print(f"7 = {str}")
                elif len(str) == 7:
                    print(f"8 = {str}")
                elif len(str) == 6:
                    print(f"0/6/9 = {str}")
                elif len(str) == 5:
                    print(f"2/3/5 = {str}")
                else:
                    print("!?!?!")  
            print("*" * 15) 
            for str in parts[1]:
                # sort string letters
                str = "".join([x for x in sorted(str)])
                if len(str) == 2:
                    print(f"1 = {str}")
                elif len(str) == 4:
                    print(f"4 = {str}")
                elif len(str) == 3:
                    print(f"7 = {str}")
                elif len(str) == 7:
                    print(f"8 = {str}")
                elif len(str) == 6:
                    print(f"0/6/9 = {str}")
                elif len(str) == 5:
                    print(f"2/3/5 = {str}")
                else:
                    print("!?!?!")  
            print("\n----------NEXT!! ------------\n")
     
        return count
        

def main():
    d = Display("simple.input")
    total = d.count_hard()
    """
    if total != 26:
        print(f"wrong amount! ({total})")
    else:
    """
    print("nums:", total)
    
    
    


main()

# one run takes 2855ms with 12700K..
#import timeit    
#time = timeit.timeit(main, number=1)
#print(f"{time*1000:.5f}ms")
