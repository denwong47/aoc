#ifndef SOLVE_BRUTE_H
#define SOLVE_BRUTE_H

#include "../types/mod.h"
#include "../utils/numbers.h"
// #include "libc_accumulative_hash_set.h"
#include "../../c-bindings/c-accumulative-hash-set/dylib/libc_accumulative_hash_set.h"
#include "common.h"

ExecutionStatus dfs_from(
    Scenario* scenario,
    Vector* destination,
    Solution* solution
);

#ifdef UNIT_TEST
void test_brute();
#endif

#endif
