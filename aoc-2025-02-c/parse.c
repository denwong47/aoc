#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include "types.c"

USIZE parse_input_from_stdin(RANGE_TYPE* found_ranges) {
    USIZE len = 0;

    char line[MAX_LINE_LENGTH];

    while (fgets(line, sizeof(line), stdin)) {
        // Set the new line character to NULL byte so that `strtok` will stop reading.
        line[strcspn(line, "\n")] = 0;
        char* segment = strtok(line, ",");
        while (segment != NULL) {
            RANGE_TYPE start, end;
            if (sscanf(segment, "%lu-%lu", &start, &end)) {
                found_ranges[len++] = start;
                found_ranges[len++] = end;
            }

            segment = strtok(NULL, ",");
        }
    }

    return len;
}

#ifdef UNIT_TEST
int main() {
    RANGE_TYPE *found_ranges = (RANGE_TYPE*)malloc(sizeof(RANGE_TYPE)*2 *MAX_RANGES_COUNT);

    USIZE count = parse_input_from_stdin(found_ranges);

    printf("Array size is %lu", count);
    for (USIZE i; i < count; i+=2) {
        printf("Range %lu: %lu-%lu\n", i, found_ranges[i], found_ranges[i+1]);
    }

    free(found_ranges);
    return 0;
}
#endif
