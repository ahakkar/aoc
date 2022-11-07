# -*- encoding: utf-8 -*-
'''
@File    :   01b.py
@Time    :   07/11/2022, 15:11:53
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
    try:
        with open (filename, "r") as read_file:
            data:list = read_file.readlines()
        read_file.close()
    except OSError:
        print("bad file name!")
        return
    
    int_data = []
    for num in data:
        int_data.append(int(num.strip()))
    
    return int_data

def main():
    data:list = read_file("input.txt")
    i:int = 0
    data_len = len(data)
    sums:list =[]   
    
    while data_len > 0:
        sums.append(sum(data[i:i+3]))
        i += 1
        data_len -= 1
    
    count:int = 0
    prev:int = 0
    
    for num in sums:        
        if prev == 0:
            prev = num
        else:
            num = int(num)                     
            if num > prev:
                count += 1
            prev = num
        
    print(count)      


if __name__ == "__main__":
    main()
