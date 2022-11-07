# -*- encoding: utf-8 -*-
'''
@File    :   03a.py
@Time    :   07/11/2022, 16:19:55
@Author  :   Antti Hakkarainen
@Student :   K79735
@Contact :   antti.i.hakkarainen@tuni.fi
@Course  :   COMP.CS.100 Ohjelmointi 1.
@Desc    :   TEMP
'''

def read_file(filename:str) -> dict:
    """
    param : TODO
    return: none
    """
    data:list = []
    
    try:
        with open (filename, "r") as read_file:
            data:list = read_file.read().splitlines()
        read_file.close()
    except OSError:
        print("bad file name!")
        return []
    
    return data

def main():
    data:list = read_file("input.txt")
    if len(data) == 0:
        return 0
    bits:int = len(data[0])
    numbers:dict = {}
    
    
    for row in data:
        i:int = 0
        while i < bits:
            if i not in numbers:
                numbers[i] = row[i]
            else:
                numbers[i] += row[i]
            i += 1
            
    gamma_rate:str = ""
    eps_rate:str = ""
       
    for key, val in numbers.items(): 
        zeroes:int = val.count('0')
        ones:int = val.count('1')
        
        if zeroes > ones:
            gamma_rate += '0'
            eps_rate += '1'
        else:
            gamma_rate += '1'
            eps_rate += '0'
            
    # int() default base 10, binary to int uses base 2
    print(int(gamma_rate,2) * int(eps_rate,2))

if __name__ == "__main__":
    main()
