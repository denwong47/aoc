#include "consolidate.h"
#include "range.h"
#include "types.h"

/// Combine two given ranges. `lhs` must be of a lower sort order than `rhs`.
ExecutionStatus combine_ranges(Range* lhs, Range* rhs) {
    if (compare_ranges(lhs, rhs) == 1) {
        #ifdef VERBOSE
        printf(
            "\x1b[31mERROR:\x1b[0m Input ranges are not sorted, \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m > \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m.\n",
            lhs->start,
            lhs->end,
            rhs->start,
            rhs->end
        );
        #endif
        return RANGES_NOT_SORTED;
    }

    if (lhs->end >= rhs->start) {
        BOUNDS new_start, new_end;
        if (lhs->start <= rhs->start) {
            new_start = lhs->start;
        } else {
            new_start = rhs->start;
        }
        if (lhs->end >= rhs->end) {
            new_end = lhs->end;
        } else {
            new_end = rhs->end;
        }

        lhs->start = new_start;
        lhs->end = new_end;

        return SUCCESS;
    }

    return RANGES_NOT_OVERLAPPING;
}

/// Combine any overlapping ranges.
ExecutionStatus consolidate_ranges(Ranges* ranges) {
    if (ranges->count <= 1) {
        #ifdef VERBOSE
        printf("\x1b[33mWARNING:\x1b[0m The provided ranges <=1 range items. Have you added to it via `stdin` or `text` yet?");
        #endif
        return SUCCESS;
    }

    Ranges consolidated_ranges = new_ranges();
    // Initialize `current` with the first item.
    Range* staged = &ranges->data[0];
    Range* current;
    ExecutionStatus combine_status;

    for (USIZE index=1; index < ranges->count; index++) {
        current = &ranges->data[index];

        switch (combine_status = combine_ranges(staged, current)) {
        case SUCCESS:
            #ifdef VERBOSE
            printf(
                "\x1b[35mINFO:\x1b[0m Combined new Range: \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m, ditching \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m.\n",
                staged->start,
                staged->end,
                current->start,
                current->end
            );
            #endif
            break;
        case RANGES_NOT_OVERLAPPING:
            #ifdef VERBOSE
            printf(
                "\x1b[35mINFO:\x1b[0m Ranges do not overlap: \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m and \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m.\n",
                staged->start,
                staged->end,
                current->start,
                current->end
            );
            #endif
            if ((combine_status = add_to_ranges(*staged, &consolidated_ranges)) != SUCCESS) {
                return combine_status;
            }
            staged = current;
            break;
        default:
            #ifdef VERBOSE
            printf(
                "\x1b[35mERROR:\x1b[0m Unexpected status code when combining \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m and \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m: \x1b[31m%u\x1b[0m\n",
                staged->start,
                staged->end,
                current->start,
                current->end,
                combine_status
            );
            #endif
            if ((combine_status = add_to_ranges(*staged, &consolidated_ranges)) != SUCCESS) {
                return combine_status;
            }
            break;
        }
    }

    if ((combine_status = add_to_ranges(*staged, &consolidated_ranges)) != SUCCESS) {
        return combine_status;
    }

    free(ranges->data);
    // Replace the entire ranges
    *ranges = consolidated_ranges;

    return SUCCESS;
}

#ifdef UNIT_TEST
void assert_combine_ranges(RANGE_DEFINITION lhs_def, RANGE_DEFINITION rhs_def, ExecutionStatus expected_status, BOUNDS expected_start, BOUNDS expected_end) {
    Range lhs, rhs;
    assert(range_from_definition(lhs_def, &lhs) == SUCCESS);
    assert(range_from_definition(rhs_def, &rhs) == SUCCESS);
    BOUNDS original_start = lhs.start;
    BOUNDS original_end = lhs.end;

    ExecutionStatus actual_status = combine_ranges(&lhs, &rhs);
    bool success_start, success_end, success_status;

    success_status = actual_status == expected_status;
    printf(
        "Upon combining \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m with \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m, found status \x1b[%um\x1b[1m%u\x1b[0m, expected \x1b[1m%u\x1b[0m.\n",
        original_start,
        original_end,
        rhs.start,
        rhs.end,
        31+(unsigned short)success_status,
        actual_status,
        expected_status
    );
    assert(success_status);
    if (actual_status == SUCCESS || actual_status == RANGES_NOT_OVERLAPPING) {
        success_start = lhs.start == expected_start;
        success_end = lhs.end == expected_end;
        printf("Upon combining \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m with \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m, we got \x1b[%um\x1b[1m%lu\x1b[0m-\x1b[%um\x1b[1m%lu\x1b[0m, expected \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m.\n",
            original_start,
            original_end,
            rhs.start,
            rhs.end,
            31+(unsigned short)success_start,
            lhs.start,
            31+(unsigned short)success_end,
            lhs.end,
            expected_start,
            expected_end
        );

        assert(success_start && success_end);
    }
}


void assert_consolidate_ranges(const RANGE_DEFINITION range_input, const BOUNDS* expected) {
    printf(
        "Consolidating ranges from \x1b[1m%s\x1b[0m...\n",
        range_input
    );
    char buffer[MAX_LINE_LENGTH];
    // We have to copy the static string into a mutable buffer due to `strtok`.
    strncpy(buffer, range_input, sizeof(buffer));

    Ranges ranges = new_ranges();
    add_ranges_from_text((RANGE_DEFINITION)&buffer, &ranges);
    sort_ranges(&ranges);
    consolidate_ranges(&ranges);

    Range* range;
    BOUNDS expected_start, expected_end;
    bool success_start, success_end;
    for (USIZE index=0; index < ranges.count; index++) {
        range = &ranges.data[index];
        expected_start = expected[index*2];
        expected_end = expected[index*2+1];
        success_start = range->start == expected_start;
        success_end = range->end == expected_end;
        printf(
            "Asserting \x1b[%um\x1b[1m%lu\x1b[0m-\x1b[%um\x1b[1m%lu\x1b[0m to be \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m.\n",
            31+(unsigned short)success_start,
            range->start,
            31+(unsigned short)success_end,
            range->end,
            expected_start,
            expected_end
        );
        assert(success_start);
        assert(success_end);
    }
    free(ranges.data);
}

#endif
