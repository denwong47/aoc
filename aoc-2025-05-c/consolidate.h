#ifndef CONSOLIDATE_H
#define CONSOLIDATE_H

#include "common.h"
#include "compare.h"
#include "types.h"
#include "range.h"
#include "utils.h"

/// Combine two given ranges. `lhs` must be of a lower sort order than `rhs`.
ExecutionStatus combine_ranges(Range* lhs, Range* rhs);
ExecutionStatus consolidate_ranges(Ranges* ranges);

#ifdef UNIT_TEST
void assert_combine_ranges(RANGE_DEFINITION lhs_def, RANGE_DEFINITION rhs_def, ExecutionStatus expected_status, BOUNDS expected_start, BOUNDS expected_end);
void assert_consolidate_ranges(const RANGE_DEFINITION range_input, const BOUNDS* expected);
#endif

#endif
