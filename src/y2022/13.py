def read_file(filename:str) -> list:
    """
    param : str, filename to read
    return: list, lines
    """
    
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
        
    return data;

class Solution(object):
    def __init__(self, data:list):
        self.__data:list = data

    def silver(self):
        pass
    
    def gold(self):
        pass


def main():
    data:list = read_file("D:\\GDrive\\Prog\\aoc\\2022\\11\\simple.input") 
    
    sol = Solution(data)
    return 0

if __name__ == "__main__":
    main()