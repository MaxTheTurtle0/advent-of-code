import sys

draw = 3
win = 6

score = 0

if len(sys.argv) != 2:
    print("Invalid number of arguments. \n Usage: python main.py <1|2>")
    exit()
elif sys.argv[1] == "1":

    with open("../day02.txt", "r") as f:
        rounds = f.readlines()
        rounds = [line.strip("\n") for line in rounds]

    outcomes = {
        "A X": draw + 1, "A Y": win + 2, "A Z": 3,
        "B X": 1, "B Y": draw + 2, "B Z": win + 3,
        "C X": win + 1, "C Y": 2, "C Z": draw + 3
    }

    for round in rounds:
        score += outcomes[round]
elif sys.argv[1] == "2":
        
    wins = {
        "A": 2, "B": 3, "C": 1
    }        

    losses = {
        "A": 3, "B": 1, "C": 2
    }

    draws = {
        "A": 1, "B": 2, "C": 3
    }
        

    with open("../day2.txt", "r") as f:
        rounds = f.readlines()

    for round in rounds:
        if round[2] == "X":
            score += losses[round[0]]
        elif round[2] == "Y":
            score += draw + draws[round[0]]
        else:
            score += wins[round[0]] + win
else:
    print("Invalid argument \n Usage: python main.py [1/2]")
    exit()


print(f"Score: {score}")
