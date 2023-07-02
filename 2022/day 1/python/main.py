with open ("../day1.txt", "r") as file:
    data = file.read().splitlines()
    
elf_count = data.count('') + 1	

elf_calories = [0] * elf_count

current_elf = 0

for i, item in enumerate(data):
    if item != '':
        elf_calories[current_elf] += int(item)
    else:
        current_elf += 1

most_calories = 0
secondmost_calories = 0
thirdmost_calories = 0

for i, item in enumerate(elf_calories):
    if item > most_calories:
        secondmost_calories = most_calories
        most_calories = item
    elif item > thirdmost_calories and item < secondmost_calories:
        thirdmost_calories = item

print(most_calories)
print(most_calories + secondmost_calories + thirdmost_calories)