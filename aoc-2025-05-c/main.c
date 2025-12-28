#include "consolidate.h"
#include "types.h"
#include "range.h"
#include "func.h"

int main() {
    ExecutionStatus status;
    Ranges ranges = new_ranges();
    if ((status = add_ranges_from_stdin(&ranges)) != SUCCESS) {
        return status;
    }
    sort_ranges(&ranges);
    consolidate_ranges(&ranges);

    // Part 1
    COUNT count = count_of_numbers_from_stdin_within_ranges(&ranges);
    printf("Count of all ingredient IDs within the ranges: \x1b[32m%lu\x1b[0m\n", count);

    // Part 2
    COUNT total_valid = count_numbers_within_ranges(&ranges);
    printf("Count of all valid ingredient IDs within the ranges: \x1b[32m%lu\x1b[0m\n", total_valid);

    free(ranges.data);
}
