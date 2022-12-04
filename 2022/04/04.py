# -*- encoding: utf-8 -*-
'''
@File    :   04.py
@Time    :   04/12/2022, 15:22:18
@Author  :   Antti Hakkarainen
@Task    :   Advent of Code 2022
@Desc    :   TEMP
'''

def read_file(filename:str) -> list:
    """
    param : str, filename to read
    return: list, lines
    """
    
    data:list = []
    
    try:
        with open (filename, "r") as read_file:
            data: list = read_file.read().splitlines()
        read_file.close()
    except FileNotFoundError:
        print(f"Bad file name! {filename}")
        exit()
    except:
        print("SOS")
        exit()
        
    return data;

def silver_range(data:list):
    """
    # this is slower by doing it like this (tested aftewards)
    param : puzzle input as list
    return: none
    """
    total:int = 0
    for row in data:
        ran1, ran2 = row.split(',')
        a1, a2 = ran1.split('-')
        b1, b2 = ran2.split('-')
        a1 = int(a1)
        a2 = int(a2)
        b1 = int(b1)
        b2 = int(b2)    
        
        if (a1 in range(b1, b2+1)) and (a2 in range (b1, b2+1)):     
            total += 1            
        elif (b1 in range(a1, a2+1)) and (b2 in range (a1, a2+1)):     
            total += 1               
    # 456
    #print(total)  

def silver(data:list):
    """
    param : puzzle input as list
    return: none
    """
    total:int = 0
    for row in data:
        ran1, ran2 = row.split(',')
        a1, a2 = [int(x) for x in ran1.split('-')]
        b1, b2 = [int(x) for x in ran2.split('-')]   
        
        if (b1 <= a1 <= b2) and (b1 <= a2 <= b2): 
            total += 1            
        elif (a1 <= b1 <= a2) and (a1 <= b2 <= a2):                  
            total += 1
                           
    # 456
    print(total)    
    
def gold(data:list):
    """
    param : puzzle input as list
    return: none
    """
    total:int = 0
    for row in data:
        ran1, ran2 = row.split(',')
        a1, a2 = [int(x) for x in ran1.split('-')]
        b1, b2 = [int(x) for x in ran2.split('-')]           
        
        if (b1 <= a1 <= b2) or (b1 <= a2 <= b2):
            total += 1            
        elif (a1 <= b1 <= a2) or (a1 <= b2 <= a2):
            total += 1            
    
    # 808
    print(total)  
    
def combined(data:list):  
    s_total:int = 0
    g_total:int = 0
    for row in data:
        ran1, ran2 = row.split(',')
        a1, a2 = [int(x) for x in ran1.split('-')]
        b1, b2 = [int(x) for x in ran2.split('-')]   
        
        # silver
        if (b1 <= a1 <= b2) and (b1 <= a2 <= b2): 
            s_total += 1            
        elif (a1 <= b1 <= a2) and (a1 <= b2 <= a2):                  
            s_total += 1
        
        # gold
        if (b1 <= a1 <= b2) or (b1 <= a2 <= b2):
            g_total += 1            
        elif (a1 <= b1 <= a2) or (a1 <= b2 <= a2):
            g_total += 1    
            
    print(s_total)
    print(g_total) 

def main():
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\04\\puzzle.input") 
    
    #silver(data)
    #gold(data)
    combined(data)
    return 0

main()

# 840ms with 12700K, slow..
# 770ms by doing it in same loop with combined()
#import timeit    
#time = timeit.timeit(main, number=1000)
#print(f"{time*1000:.5f}ms")



