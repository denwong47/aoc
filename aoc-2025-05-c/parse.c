#include "parse.h"

/// Read a number from `stdin`, and replace `number` with the new value.
/// Returns a status indicating the result of the operation.
ExecutionStatus read_number_from_stdin(BOUNDS* number) {
    RANGE_DEFINITION buffer = (RANGE_DEFINITION)malloc(MAX_LINE_LENGTH * sizeof(char));
    size_t len=MAX_LINE_LENGTH;
    size_t read=getline(&buffer, &len, stdin);
    ExecutionStatus result;

    if (read == -1) {
        result = PARSE_FAILURE_EMPTY_LINE;
    } else {
        rtrim_new_line(buffer, read);

        if (sscanf(buffer, "%lu", number) == 1) {
            #ifdef VERBOSE
            printf("\x1b[35mINFO:\x1b[0m Found \x1b[1m\"%lu\"\x1b[0m in stdin.\n", *number);
            #endif
            result = SUCCESS;
        } else {
            #ifdef VERBOSE
            printf("\x1b[31mERROR:\x1b[0m Could not parse \x1b[1m\"%s\"\x1b[0m into a number.\n", buffer);
            #endif
            result = PARSE_FAILURE_NOT_A_NUMBER;
        }
    }

    free(buffer);
    return result;
}
