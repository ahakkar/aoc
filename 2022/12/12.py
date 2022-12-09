

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

def silver(data:list):
    """
    param : puzzle input as list
    return: none
    """
    print("TBD")
    
def gold(data:list):
    """
    param : puzzle input as list
    return: none
    """
    print("TBD")



def main():
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\02\\simple.input") 
    
    silver(data)
    gold(data)
    return 0

if __name__ == "__main__":
    main()
