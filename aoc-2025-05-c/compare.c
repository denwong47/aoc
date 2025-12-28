#include "compare.h"

#ifdef UNIT_TEST
#include <stdbool.h>
#include <assert.h>
#endif

/// Compare two ranges, first by their start, then by their end
int compare_ranges(Range* lhs, Range* rhs) {
    if (lhs->start > rhs->start) return 1;
    if (lhs->start < rhs->start) return -1;
    if (lhs->end > rhs->end) return 1;
    if (lhs->end < rhs->end) return -1;
    return 0;
}

/// Compare two ranges, first by their start, then by their end;
/// The signature of this function is meant to match the requirements
/// of `qsort`, which forces casting.
int _compare_ranges_for_qsort(const void* lhs, const void* rhs) {
    Range* lhs_casted = (Range*)lhs;
    Range* rhs_casted = (Range*)rhs;
    return compare_ranges(lhs_casted, rhs_casted);
}

/// Sort all the ranges in a `Ranges` struct.
void sort_ranges(Ranges* ranges) {
    #ifdef VERBOSE
    printf("\x1b[34mINFO:\x1b[0m Sorting %u ranges...\n", ranges->count);
    #endif
    qsort(ranges->data, ranges->count, sizeof(Range), _compare_ranges_for_qsort);

    #ifdef VERBOSE
    printf("\x1b[34mINFO:\x1b[0m Sorted %u ranges.\n", ranges->count);
    #endif
}

#ifdef UNIT_TEST
void assert_compare(RANGE_DEFINITION lhs_def, RANGE_DEFINITION rhs_def, int expected) {
    Range lhs;
    Range rhs;
    assert(range_from_definition(lhs_def, &lhs) == SUCCESS);
    assert(range_from_definition(rhs_def, &rhs) == SUCCESS);

    int actual = compare_ranges(&lhs, &rhs);
    unsigned int colour = 31 + (actual == expected);
    printf(
        "Comparing \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m to \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m, asserting \x1b[%dm\x1b[1m%d\x1b[0m to be \x1b[1m%d\x1b[0m\n",
        lhs.start, lhs.end, rhs.start, rhs.end, colour, actual, expected
    );
    assert(actual==expected);
}
#endif
