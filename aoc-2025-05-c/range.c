#include "range.h"

/// Create a new `Ranges` object.
Ranges new_ranges() {
    USIZE capacity = MAX_RANGES;
    Range *data = (Range*)malloc(capacity * sizeof(Range));
    Ranges ranges;

    ranges.data = data;
    ranges.capacity = capacity;
    ranges.count = 0;

    return ranges;
}

/// Add a `Range` to the `Ranges`.
ExecutionStatus add_to_ranges(Range range, Ranges* ranges) {
    if (ranges->count < ranges->capacity) {
        ranges->data[ranges->count] = range;
        ranges->count++;
        return SUCCESS;
    }

    #ifdef VERBOSE
    printf("\x1b[31mERROR:\x1b[0m Ranges is full, could not add \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m\n", range.start, range.end);
    #endif

    return PARSE_FAILURE_RANGES_FULL;
}

/// Read lines from a string until we hit EOF.
/// Typically used for Unit tests only, as the actual `main` simply reads from `stdin`.
ExecutionStatus add_ranges_from_text(char* pattern, Ranges* ranges) {
    char* segment = strtok(pattern, ",");
    ExecutionStatus status;

    while (segment != NULL) {
        Range range;
        if ((status = range_from_definition(segment, &range)) != SUCCESS) {
            printf("\x1b[31mERROR:\x1b[0m Failed to parse due to error \x1b[1m%d\x1b[0m.\n", status);
            return status;
        }
        if ((status = add_to_ranges(range, ranges)) != SUCCESS) {
            printf("\x1b[31mERROR:\x1b[0m Failed to add range to ranges due to error \x1b[1m%d\x1b[0m.\n", status);
            return status;
        }

        segment = strtok(NULL, ",");
    }

    return SUCCESS;
}

/// Check if a number is within any of the `Range` in a `Ranges`.
/// This requires the `ranges` to be sorted. If not, a RANGES_NOT_SORTED will be returned.
/// This safeguard requires a small amount of copying, which might have a performance impact,
/// but prevents a lot of undefined behaviours.
ExecutionStatus is_within_ranges(BOUNDS number, Ranges* ranges) {
    Range last_seen;
    Range* range;
    range_from_definition("0-0", &last_seen);

    for (USIZE index=0; index < ranges->count; index++) {
        range = &ranges->data[index];
        if (compare_ranges(&last_seen, range) > 0) {
            #ifdef VERBOSE
            printf(
                "\x1b[31mERROR:\x1b[0m Ranges are not sorted, \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m > \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m.\n",
                last_seen.start,
                last_seen.end,
                range->start,
                range->end
            );
            #endif
            return RANGES_NOT_SORTED;
        }

        if (number >= range->start && number <= range->end) {
            #ifdef VERBOSE
            printf(
                "\x1b[36mDEBUG:\x1b[0m Number \x1b[1m%lu\x1b[0m is within \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m.\n",
                number,
                range->start,
                range->end
            );
            #endif
            return SUCCESS;
        }
        last_seen = *range;
    }

    return NOT_IN_RANGES;
}

/// Read lines on stdin until we hit a PARSE_FAILURE_INVALID_RANGE.
ExecutionStatus add_ranges_from_stdin(Ranges* ranges) {
    size_t len = MAX_LINE_LENGTH;
    size_t read;
    ExecutionStatus status;
    RANGE_DEFINITION buffer = (RANGE_DEFINITION)malloc(sizeof(char) * MAX_LINE_LENGTH);

    while ((read = getline(&buffer, &len, stdin)) != -1) {
        Range range;
        rtrim_new_line(buffer, read);
        status = range_from_definition(buffer, &range);
        // Replace any new line characters with NULL byte

        if (status == PARSE_FAILURE_INVALID_RANGE) {
            #ifdef VERBOSE
            printf("\x1b[34mINFO:\x1b[0m Encountered a line without range, breaking: \x1b[1m\"%s\"\x1b[0m\n", buffer);
            #endif
            break;
        }

        status = add_to_ranges(range, ranges);
        if (status != SUCCESS) {
            return status;
        } else {
            #ifdef VERBOSE
            printf("\x1b[34mINFO:\x1b[0m Added range of \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m to ranges; there are currently \x1b[1m%u\x1b[0m ranges.\n", range.start, range.end, ranges->count);
            #endif
        }
    }

    free(buffer);
    return SUCCESS;
}

#ifdef UNIT_TEST
void assert_sort_ranges(const RANGE_DEFINITION range_input, const BOUNDS* expected) {
    printf(
        "Sorting ranges from \x1b[1m%s\x1b[0m...\n",
        range_input
    );
    char buffer[MAX_LINE_LENGTH];
    // We have to copy the static string into a mutable buffer due to `strtok`.
    strncpy(buffer, range_input, sizeof(buffer));

    Ranges ranges = new_ranges();
    add_ranges_from_text((RANGE_DEFINITION)&buffer, &ranges);
    sort_ranges(&ranges);

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

void assert_in_ranges(const RANGE_DEFINITION range_input, const BOUNDS number, const ExecutionStatus expected) {
    char buffer[MAX_LINE_LENGTH];
    ExecutionStatus status;
    // We have to copy the static string into a mutable buffer due to `strtok`.
    strncpy(buffer, range_input, sizeof(buffer));

    Ranges ranges = new_ranges();
    if ((status = add_ranges_from_text((RANGE_DEFINITION)&buffer, &ranges)) != SUCCESS) {
        printf(
            "\x1b[31mERROR:\x1b[0m Could not add ranges from text, error \x1b[1m%u\x1b[0m.\n",
            status
        );
    }

    ExecutionStatus actual = is_within_ranges(number, &ranges);
    bool success = actual == expected;

    printf(
        "Number \x1b[1m%lu\x1b[0m in \x1b[1m\"%s\"\x1b[0m is \x1b[%um\x1b[1m%d\x1b[0m, asserting to be \x1b[1m%d\x1b[0m.\n",
        number,
        range_input,
        31 + (unsigned short)success,
        actual,
        expected
    );
    assert(success);
    free(ranges.data);
}
#endif
