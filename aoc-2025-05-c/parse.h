#ifndef PARSE_H
#define PARSE_H

#include "common.h"
#include "utils.h"
#include "types.h"

/// Read a number from `stdin`, and replace `number` with the new value.
/// Returns a status indicating the result of the operation.
ExecutionStatus read_number_from_stdin(BOUNDS* number);

#endif
