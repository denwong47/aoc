#ifndef RANGE_H
#define RANGE_H

#include "common.h"
#include "compare.h"
#include "utils.h"
#include "types.h"

/// Create a new `Ranges` object.
Ranges new_ranges();

/// Add a `Range` to the `Ranges`.
ExecutionStatus add_to_ranges(Range range, Ranges* ranges);

/// Read lines from a string until we hit EOF.
/// Typically used for Unit tests only, as the actual `main` simply reads from `stdin`.
ExecutionStatus add_ranges_from_text(char* pattern, Ranges* ranges);

/// Check if a number is within any of the `Range` in a `Ranges`.
/// This requires the `ranges` to be sorted. If not, a RANGES_NOT_SORTED will be returned.
/// This safeguard requires a small amount of copying, which might have a performance impact,
/// but prevents a lot of undefined behaviours.
ExecutionStatus is_within_ranges(BOUNDS number, Ranges* ranges);

/// Read lines on stdin until we hit a PARSE_FAILURE_INVALID_RANGE.
ExecutionStatus add_ranges_from_stdin(Ranges* ranges);

#ifdef UNIT_TEST

void assert_sort_ranges(const RANGE_DEFINITION range_input, const BOUNDS* expected);
void assert_in_ranges(const RANGE_DEFINITION range_input, const BOUNDS number, const ExecutionStatus expected);

#endif

#endif
