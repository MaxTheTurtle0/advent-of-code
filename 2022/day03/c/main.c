#include <stdio.h>
#include <string.h>
#include <stdlib.h>

// strndup is not available on Windows so we need to implement it ourselves
char *strndup(const char *s, size_t n) {
    size_t len = strnlen(s, n);
    char *dup = malloc(len + 1);
    if (dup == NULL) {
        return NULL;
    }
    memcpy(dup, s, len);
    dup[len] = '\0';
    return dup;
}

char have_common_char(char *str1, char *str2) {
    for (int i = 0; i < strlen(str1); i++) {
        for (int j = 0; j < strlen(str2); j++) {
            if (str1[i] == str2[j]) {
                return str1[i] ;
            }
        }
    }
}

int main(int argc,char *argv[]) {

    if (argc != 3) {
        printf("Usage: ./day03 <1|2> <input file>\n");
        return 1;
    }

    FILE *f = fopen("../day03.txt", "r");

    char line[100];

    int sum_item_priorities = 0;

    if (f == NULL) {
        printf("Failed to open the file.\n");
        return 1;
    }

    if (strcmp(argv[1], "1") == 0) {
        while (fgets(line, sizeof(line), f)) {
            int length = strlen(line);

            int half = length / 2;

            char *left_compartment = strndup(line, half);

            char *right_compartment = strndup(line + half, half);

            char common_char = have_common_char(left_compartment, right_compartment);

            int priority = common_char - 'A' + 1;

            if (priority < 27) {
                priority += 26;
            } else if (priority > 32) {
                priority -= 32;
            }

            sum_item_priorities += priority;
        }
        printf("Sum of item priorities: %i\n", sum_item_priorities);
        fclose(f);
    } else if (strcmp(argv[1], "2") == 0) {
        printf("Part 2 is not implemented yet\n");
        fclose(f);
    } else {
        printf("Invalid argument \n Usage: ./day03 <1|2> <input file>");
        fclose(f);
        return 1;
    }

    return 0;
}