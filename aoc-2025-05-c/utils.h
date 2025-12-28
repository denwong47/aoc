#ifndef UTILS_H
#define UTILS_H
#include "types.h"

/// Try parsing a `Range` from a string such as `123-456`.
ExecutionStatus range_from_definition(RANGE_DEFINITION definition, Range* range);

/// Trim any trailing new lines
void rtrim_new_line(RANGE_DEFINITION buffer, USIZE length);

#endif
