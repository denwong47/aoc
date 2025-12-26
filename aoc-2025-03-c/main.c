#include "types.c"
#include "parse.c"
#include "func.c"
#include <string.h>
#include <stdlib.h>

int main() {
    INPUT_LINE buffer = (INPUT_LINE)malloc(MAX_LINE_LENGTH*sizeof(char));
    INPUT_LINE buffer2 = (INPUT_LINE)malloc(MAX_LINE_LENGTH*sizeof(char));
    USIZE read;

    JOLTAGE joltage_2 = 0;
    JOLTAGE joltage_12 = 0;
    while ((read = read_line(buffer)) != 0) {
        strcpy(buffer2, buffer);
        joltage_2 += find_highest_joltage(buffer, read-1, 2);
        joltage_12 += find_highest_joltage(buffer2, read-1, 12);
    }

    printf("Part 1 Total Joltage: \x1b[32m%lu\x1b[0m\n", joltage_2);
    printf("Part 2 Total Joltage: \x1b[32m%lu\x1b[0m\n", joltage_12);
}
