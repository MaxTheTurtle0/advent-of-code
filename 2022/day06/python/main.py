import sys

# Checking if the user has entered a valid amount of arguments
if len(sys.argv) != 3:
    print("Usage: python main.py <1|2> <input file>")
    exit()

with open(sys.argv[2]) as f:
    content = f.readlines()

def get_first_marker(marker_length: int) -> int | None:
    for i in range(len(content[0]) - marker_length - 1):
        chars = content[0][i:i + marker_length]
        if len(set(chars)) == marker_length:
            return i + marker_length

if sys.argv[1] == "1":
    print(f"Part 2: {get_first_marker(4)}")
elif sys.argv[1] == "2":
    print(f"Part 1: {get_first_marker(14)}")
else:
    print("Invalid argument \n Usage: python main.py <1|2> <input file>")
    exit()