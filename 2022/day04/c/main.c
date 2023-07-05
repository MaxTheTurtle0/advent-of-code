#include <stdio.h>
#include <string.h>

int main(int argc,char *argv[]) {

    // Check for the correct number of arguments
    if (argc != 3) {
        printf("Usage: ./day04 <1|2> <input file>\n");
        return 1;
    }

    if (strcmp(argv[1], "1") == 0) {
        printf("Part 1\n");
    } else if (strcmp(argv[1], "2") == 0) {
        printf("Part 2\n");
    } else {
        printf("Usage: ./day04 <1|2> <input file>\n");
        return 1;
    }

    return 0;
}