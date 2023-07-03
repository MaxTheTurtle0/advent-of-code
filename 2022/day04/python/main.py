import sys 

# Checking if the user has entered a valid amount of arguments
if len(sys.argv) != 3:
    print("Usage: python main.py <1|2> <input file>")
    exit()

sum_pairs_full_contain = 0

# making sure the user has entered valid arguments
if sys.argv[1] == "1":
    # Part 1
    with open(sys.argv[2]) as f:
        content = f.readlines()
    content = [x.strip("\n").split(",") for x in content]
    for i in range(len(content)):
        pairs = [x.split("-") for x in content[i]]
        if int(pairs[0][0]) <= int(pairs[1][0]) and int(pairs[0][1]) >= int(pairs[1][1]):
            sum_pairs_full_contain += 1
        elif int(pairs[0][0]) >= int(pairs[1][0]) and int(pairs[0][1]) <= int(pairs[1][1]):
            sum_pairs_full_contain += 1
    print(f"In {sum_pairs_full_contain} assignments one range fully contains the other.")
elif sys.argv[1] == "2":
    # Part 2
    print("Part 2")
else:
    print("Invalid argument \n Usage: python main.py <1|2> <input file>")
    exit()
