#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include "types.c"

JOLTAGE compile_joltage(char* buffer, USIZE total, USIZE digits) {
    USIZE found = 0;
    JOLTAGE joltage = 0;

    for (USIZE index=0; index < total; index++) {
        if (buffer[index] != BLANK) {
            joltage = joltage * 10 + (buffer[index] - '0');
            found++;

            if (found >= digits) {
                break;
            }
        }
    }
    return joltage;
}

char find_next_char(char* buffer, USIZE total, USIZE index) {
    for (USIZE current=index; current < total; current++) {
        if (buffer[current] != BLANK) {
            return buffer[current];
        }
    }

    return BLANK;
}

USIZE find_last_non_blank_index(char* buffer, USIZE index) {
    for (USIZE backtrack=index; backtrack>0; backtrack--) {
        if (backtrack==0 || buffer[backtrack] != BLANK) {
            return backtrack;
        }
    }
    // If we haven't found anything to backtrack to, just go to 0
    return 0;
}

JOLTAGE find_highest_joltage(char* buffer, USIZE total, USIZE digits) {
    USIZE remaining = total;
    USIZE index = 0;
    while (true) {
        char current_char = buffer[index];
        if (remaining <= digits) {
            #ifdef VERBOSE
            printf("Breaking the loop as only \x1b[1m%u\x1b[0m characters remaining.\n", remaining);
            #endif
            break;
        } else if (index >= total -1) {
            #ifdef VERBOSE
            printf("Breaking the loop as we are at the end of sentence.\n");
            #endif
            break;
        } else if (current_char == BLANK) {
            #ifdef VERBOSE
            printf("Advancing \x1b[1m%u\x1b[0m by \x1b[1m1\x1b[0m as the position is BLANK.\n", index);
            #endif
            index++;
        } else {
            char next_char = find_next_char(buffer, total, index + 1);

            if (current_char < next_char) {
                #ifdef VERBOSE
                printf(
                    "Marking '\x1b[1m%c\x1b[0m' at \x1b[1m%u\x1b[0m as BLANK, since it is smaller than '\x1b[1m%c\x1b[0m'.\n",
                    current_char, index, next_char
                );
                #endif
                buffer[index] = BLANK;
                remaining--;
                USIZE new_index = find_last_non_blank_index(buffer, index);
                #ifdef VERBOSE
                printf(
                    "Backtracked from \x1b[1m%u\x1b[0m to \x1b[1m%u\x1b[0m.\n",
                    index,
                    new_index
                );
                #endif
                index = new_index;
            } else {
                index++;
            }
        }
    }

    JOLTAGE joltage = compile_joltage(buffer, total, digits);
    #ifdef VERBOSE
    printf("Highest Joltage for \x1b[1m%d\x1b[0m digits: \x1b[32m%lu\x1b[0m\n", digits, joltage);
    #endif
    return joltage;
}

#ifdef UNIT_TEST
int main() {
    char peak2[] = "123456787654321";
    assert(find_highest_joltage(peak2, 15, 2)==87);
    char peak5[] = "123456787654321";
    assert(find_highest_joltage(peak5, 15, 5)==87654);
    char descending2[] = "987654321111111";
    assert(find_highest_joltage(descending2, 15, 2)==98);
    char descending5[] = "987654321111111";
    assert(find_highest_joltage(descending5, 15, 5)==98765);
    char ascending2[] = "1234567899999999";
    assert(find_highest_joltage(ascending2, 15, 2)==99);
    char ascending5[] = "1234567899999999";
    assert(find_highest_joltage(ascending5, 15, 5)==99999);
    char peaks2[] = "321232343210014";
    assert(find_highest_joltage(peaks2, 15, 2)==44);
    char peaks5[] = "321232343210014";
    assert(find_highest_joltage(peaks5, 15, 5)==43214);
    char actual2[] = "7455337345554393449454442744452533444624555444444525654744644442462265544584444244243377662874573954";
    assert(find_highest_joltage(actual2, 100, 2)==99);
    char actual12[] = "7455337345554393449454442744452533444624555444444525654744644442462265544584444244243377662874573954";
    assert(find_highest_joltage(actual12, 100, 12)==998874573954);
}
#endif
