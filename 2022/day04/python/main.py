import sys 

# Checking if the user has entered a valid amount of arguments
if len(sys.argv) != 3:
    print("Usage: python main.py <1|2> <input file>")
    exit()

print("This is going to be day04 in python")

# making sure the user has entered valid arguments
if sys.argv[1] == "1":
    # Part 1
    print("Part 1")
elif sys.argv[1] == "2":
    # Part 2
    print("Part 2")
else:
    print("Invalid argument \n Usage: python main.py <1|2> <input file>")
    exit()
