# -*- encoding: utf-8 -*-
'''
@File    :   03b.py
@Time    :   07/11/2022, 17:18:51
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2021
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

def keep_at_col_i(data:list, pos:int, gas:str) -> str:
    """
    param : TODO
    return: none
    """    

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
    
          
    ones:int = numbers[pos].count('1')
    zeroes:int = numbers[pos].count('0')
    
    if gas == "o2":   
        if zeroes == ones:
            return '1'
        elif zeroes > ones:
            return '0'    
        else:
            return '1'
    
    if gas == "co2":
        if zeroes == ones:
            return '0'
        elif zeroes > ones:
            return '1'    
        else:
            return '0'
    
def main():
    import time
    start = time.time()
    
    data:list = read_file("input.txt")
    if len(data) == 0:
        return 0
    amt_rows = len(data)
    data_oxygen:list = data.copy()
    data_co2:list = data.copy()
    
    oxygen_rem:list = []
    co2_rem:list = []

    for i in range(len(data[0])):
        keep_o2:str = keep_at_col_i(data_oxygen, i, "o2")
        keep_co2:str = keep_at_col_i(data_co2, i, "co2")
        #print(f"o2 at {i}:", most_common_o2)
        #print(f"co2 at {i}:", keep_co2)
        
        # oxygen
        if len(data_oxygen) > 1:
            for j in range(len(data_oxygen)):
                if data_oxygen[j][i] == keep_o2:
                    oxygen_rem.append(data_oxygen[j])
                    
            data_oxygen = oxygen_rem.copy()
            oxygen_rem = []

        # co2
        if len(data_co2) > 1:
            for k in range(len(data_co2)):
                if data_co2[k][i] == keep_co2:                
                    co2_rem.append(data_co2[k])
                                
            data_co2 = co2_rem.copy()
            co2_rem = []
        
      
    # int() default base 10, binary to int uses base 2
    print("****************")
    print(data_oxygen)
    print(data_co2)  
    
    print(int(data_oxygen[0], 2))
    print(int(data_co2[0], 2))
    
    print(int(data_oxygen[0], 2) * int(data_co2[0], 2))
    
    # record end time
    end = time.time()
    
    # print the difference between start
    # and end time in milli. secs
    print("The time of execution of above program is :",
        (end-start) * 10**3, "ms")
    
def rec_deleter(numbers: dict, dataset:list):
    """
    param : TODO
    return: none
    """

if __name__ == "__main__":
    main()
