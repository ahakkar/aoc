# -*- encoding: utf-8 -*-
'''
@File    :   02b.py
@Time    :   07/11/2022, 15:55:40
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
            data:list = read_file.read().splitlines()
        read_file.close()
    except OSError:
        print("bad file name!")
        return
    
    return data

def main():
    data:list = read_file("input.txt")
    y_pos:int = 0
    z_pos:int = 0
    aim:int = 0
    
    for row in data:
        cmd, val = row.split(' ')
        val = int(val)
        match cmd:
            case "forward":
                y_pos += val
                z_pos += val * aim                
            case "up":                
                aim -= val                
            case "down":                
                aim += val                
            case _:
                print("error, wrong cmd")
                break
    
    print(y_pos*z_pos)
    
if __name__ == "__main__":
    main()
