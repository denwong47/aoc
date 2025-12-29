#include "numbers.h"

/*
 * @brief Internal function to parse a string of comma-separated numbers into an
 * array.
 *
 * The `input` string will be mutated by `strtok`; please ensure you do not need
 * the original string, and `input` is not a `static` string literal.
 */
ExecutionStatus parse_numbers(STRING input, NUMBER* array, USIZE* count) {
    log_to_stderr(TRACE, "Parsing input \x1b[1m\"%s\"\x1b[22m into numbers...", input);
    STRING segment = strtok(input, ",");
    USIZE read;
    NUMBER number;

    *count = 0;

    while (segment != NULL) {
        log_to_stderr(TRACE, "Parsing segment \x1b[1m\"%s\"\x1b[22m into a number...", segment);
        read = sscanf(segment, "%u", &number);

        if (read != 1) {
            log_to_stderr(ERROR, "Could not parse segment \x1b[1m\"%s\"\x1b[22m into a number.", segment);
            return PARSE_INVALID_NUMBER;
        }

        log_to_stderr(TRACE, "At position \x1b[1m%u\x1b[22m, found a number of \x1b[1m%u\x1b[22m.", *count, number);

        if (*count >= MAX_DIM) {
            log_to_stderr(ERROR, "Only supports upto \x1b[1m%u\x1b[22m dimensions, found at least \x1b[31m\x1b[1m%u\x1b[0m.", MAX_DIM, *count+1);
            return PARSE_DIMENSIONS_OUT_OF_RANGE;
        }

        array[*count] = number;
        (*count)++;

        // Find the next segment
        segment = strtok(NULL, ",");
    }

    return SUCCESS;
}
