#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int calories;
    int index;
} Elf;

void updateTopThree(Elf topThree[], int calories, int index) {
    if (calories > topThree[0].calories) {
        topThree[2] = topThree[1];
        topThree[1] = topThree[0];
        topThree[0].calories = calories;
        topThree[0].index = index;
    } else if (calories > topThree[1].calories) {
        topThree[2] = topThree[1];
        topThree[1].calories = calories;
        topThree[1].index = index;
    } else if (calories > topThree[2].calories) {
        topThree[2].calories = calories;
        topThree[2].index = index;
    }
}

int main(int argc, char *argv[]) {

    if (argc != 2) {
        printf("Usage: ./day1 filename\n");
        return 1;
    }

    FILE *file = fopen(argv[1], "r");
    if (file == NULL) {
        printf("Failed to open the file.\n");
        return 1;
    }

    Elf topThree[3] = { {0, -1}, {0, -1}, {0, -1} }; // Initialize top three Elves
    int currentCalories = 0; // Current Calories being calculated
    int index = 0; // Index of the current Elf
    char line[10]; // Assuming each line contains up to 10 characters

    while (fgets(line, sizeof(line), file)) {
        if (line[0] == '\n') {
            // Blank line, indicates the end of an Elf's inventory
            updateTopThree(topThree, currentCalories, index);
            currentCalories = 0;
            index++;
        } else {
            // Parse the line and add Calories to the current total
            int calories = atoi(line);
            currentCalories += calories;
        }
    }

    // Check if the last Elf's inventory is in the top three
    updateTopThree(topThree, currentCalories, index);

    fclose(file);

    int totalCalories = topThree[0].calories + topThree[1].calories + topThree[2].calories;
    printf("The top three Elves are carrying a total of %d Calories.\n", totalCalories);

    return 0;
}
