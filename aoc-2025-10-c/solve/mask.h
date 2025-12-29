#ifndef SOLVE_MASK_H
#define SOLVE_MASK_H

#include "../types/mod.h"
#include "../utils/log.h"

ExecutionStatus bfs_for_mask(
    Scenario* scenario,
    Button* mask,
    Solution* solution,
    Vector* destination
);

#ifdef UNIT_TEST
#include "../parse/line.h"
void test_bfs_for_mask();
#endif
#endif
