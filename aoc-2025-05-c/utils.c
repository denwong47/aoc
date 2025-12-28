#include "common.h"
#include "types.h"
#include "utils.h"

/// Try parsing a `Range` from a string such as `123-456`.
ExecutionStatus range_from_definition(RANGE_DEFINITION definition, Range* range) {
    BOUNDS start, end;
    if (sscanf(definition, "%lu-%lu", &start, &end) == 2) {
        range->start = start;
        range->end = end;
        return SUCCESS;
    } else {
        #ifdef VERBOSE
        printf("\x1b[31mERROR: \x1b[0m\x1b[1m\"%s\"\x1b[0m is not a correct definition.\n", definition);
        #endif
        return PARSE_FAILURE_INVALID_RANGE;
    }
}

/// Trim any trailing new lines
void rtrim_new_line(RANGE_DEFINITION buffer, USIZE length) {
    if (length > 0 && buffer[length-1] == '\n') {
        buffer[length-1] = '\0';
    }
}
