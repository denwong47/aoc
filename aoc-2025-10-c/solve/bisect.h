#ifndef SOLVE_BISECT_H
#define SOLVE_BISECT_H

#include "../types/mod.h"
#include "common.h"
#include "mask.h"

ExecutionStatus solve_by_bisection(
    Scenario* scenario,
    Vector* destination,
    Solution* solution
);

#ifdef UNIT_TEST
void test_bisection();
#endif
#endif
