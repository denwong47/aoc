#ifndef COMPARE_H
#define COMPARE_H

#include "common.h"
#include "types.h"
#include "utils.h"

#ifdef UNIT_TEST
#include <stdbool.h>
#include <assert.h>
#endif

/// Compare two ranges, first by their start, then by their end
int compare_ranges(Range* lhs, Range* rhs);

/// Compare two ranges, first by their start, then by their end;
/// The signature of this function is meant to match the requirements
/// of `qsort`, which forces casting.
int _compare_ranges_for_qsort(const void* lhs, const void* rhs);

/// Sort all the ranges in a `Ranges` struct.
void sort_ranges(Ranges* ranges);

#ifdef UNIT_TEST
void assert_compare(RANGE_DEFINITION lhs_def, RANGE_DEFINITION rhs_def, int expected);
#endif

#endif
