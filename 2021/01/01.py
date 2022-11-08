# -*- encoding: utf-8 -*-
'''
@File    :   01.py
@Time    :   07/11/2022, 15:05:29
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2021
@Desc    :   TEMP
'''

def read_file(filename:str) -> dict:
    """
    param : TODO
    return: none
    """
    try:
        with open (filename, "r") as read_file:
            data:list = read_file.readlines()  
        read_file.close()
    except OSError:
        print("bad file name!")
        return
    
    return data

def main():
    data:list = read_file("input.txt")
    count:int = 0
    prev:int = 0
    
    for num in data:
        num = int(num.strip())  
        if prev == 0:
            prev = num
        else:
            num = int(num)                     
            if num > prev:
                count += 1
            prev = num
        
    print(count)
    
    return 0

if __name__ == "__main__":
    main()
