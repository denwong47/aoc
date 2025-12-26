#include "types.c"
#include "parse.c"
#include "func.c"
#include <stdlib.h>

int main() {
    RANGE_TYPE *found_ranges = (RANGE_TYPE*)malloc(sizeof(RANGE_TYPE)*2 *MAX_RANGES_COUNT);
    USIZE count = parse_input_from_stdin(found_ranges);
    printf("Found \x1b[1m%lu\x1b[0m numbers.\n", count);

    RANGE_TYPE total_part1 = 0;
    RANGE_TYPE total_part2 = 0;
    for (USIZE i=0; i < count; i+=2) {
        USIZE start = found_ranges[i];
        USIZE end = found_ranges[i+1];

        total_part1 += sum_invalids_in_range(start, end, 2);
        total_part2 += sum_invalids_in_range(start, end, 0);
    }

    printf("The total of all invalid IDs (repeats == 2) is \x1b[1m\x1b[31m%lu\x1b[0m.\n", total_part1);
    printf("The total of all invalid IDs (repeats == any) is \x1b[1m\x1b[31m%lu\x1b[0m.\n", total_part2);
    free(found_ranges);
}
