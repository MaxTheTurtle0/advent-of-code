import sys
from string import ascii_letters

# Checking if the user has entered a valid amount of arguments
if len(sys.argv) != 3:
    print("Usage: python main.py <1|2> <input file>")
    exit()

# Read input file
with open(sys.argv[2]) as f:
    data = f.readlines()

# Remove newlines
data = [line.strip("\n") for line in data]

# Setting sum of items to default value
sum_items = 0

# making sure the user has entered valid arguments
if sys.argv[1] == "1":
# Part 1

    for rucksacks in data:
        half = len(rucksacks) // 2

        right_compartment = rucksacks[half:]
        left_compartment = rucksacks[:half]

        for priority, item in enumerate(ascii_letters):
            if item in left_compartment and item in right_compartment:
                sum_items += priority + 1

    print(f"Sum of item priorities for groups: {sum_items}")
elif sys.argv[1] == "2":
# Part 2
    part2_data = []
    for i in range(0, len(data), 3):
            part2_data.append([data[i], data[i + 1], data[i + 2]])

    for i in range(len(part2_data)):  
            for priority, item in enumerate(ascii_letters):
                if item in part2_data[i][0] and item in part2_data[i][1] and item in part2_data[i][2]:
                    sum_items += priority + 1
                    break

    print(f"Sum of item priorities: {sum_items}")
else:
    print("Invalid argument \n Usage: python main.py <1|2> <input file>")
    exit()
