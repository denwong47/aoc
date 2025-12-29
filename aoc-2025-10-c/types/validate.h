#ifndef TYPES_VALIDATE_H
#define TYPES_VALIDATE_H

#include "common.h"
#include "solution.h"
#include "scenario.h"

ExecutionStatus compile_vector_from_solution(
    Scenario* scenario,
    Solution* solution,
    Vector* vector
);

#endif
