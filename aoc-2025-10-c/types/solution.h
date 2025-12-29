#ifndef TYPES_SOLUTION_H
#define TYPES_SOLUTION_H

#include "common.h"
#include "status.h"
#include "vector.h"
#include "../utils/log.h"
#include "../utils/numbers.h"

typedef struct {
    PRESSES presses;
    USIZE button_count;
} Solution;

void empty_solution(Solution* solution);
Solution new_solution(USIZE button_count);
ExecutionStatus parse_solution_from_numbers(STRING input, Solution* solution);
ExecutionStatus combined_solutions(Solution* lhs, Solution* rhs);
PRESS_AMOUNT press_count(Solution* solution);
void multiply_solution(Solution* solution, USIZE amount);
void free_solution(Solution* solution);

#endif
