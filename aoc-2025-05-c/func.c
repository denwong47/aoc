#include "func.h"
#include "consolidate.h"

/// Sum up all the numbers from `stdin` that are within the supplied `Ranges`.
COUNT count_of_numbers_from_stdin_within_ranges(Ranges *ranges) {
    COUNT count=0;
    BOUNDS number;
    ExecutionStatus status;

    while ((status = read_number_from_stdin(&number)) == SUCCESS) {
        if (is_within_ranges(number, ranges) == SUCCESS) {
            count++;
            #ifdef VERBOSE
            printf(
                "\x1b[35mINFO:\x1b[0m Added \x1b[1m%lu\x1b[0m to the new count of \x1b[1m%lu\x1b[0m.\n",
                number,
                count
            );
            #endif
        } else {
            #ifdef VERBOSE
            printf(
                "\x1b[36mDEBUG:\x1b[0m \x1b[1m%lu\x1b[0m is not within any of the ranges.\n",
                number
            );
            #endif
        }
    }

    return count;
}

/// Sum up all the numbers from `stdin` that are within the supplied `Ranges`.
COUNT count_numbers_within_ranges(Ranges *ranges) {
    consolidate_ranges(ranges);
    COUNT total=0;

    Range* range;

    for (USIZE index=0; index < ranges->count; index++) {
        range = &ranges->data[index];
        total += (range->end - range->start +1);
    }

    return total;
}
