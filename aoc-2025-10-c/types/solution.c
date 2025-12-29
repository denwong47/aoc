#include "solution.h"

/*
 * @brief Reset all counts in a solution to `0`.
 */
void empty_solution(Solution* solution) {
    for (USIZE index=0; index<solution->button_count; index++) {
        solution->presses[index] = 0;
    }
}

/*
 * @brief Create a new solution object to keep track of how many button presses are needed.
 */
Solution new_solution(USIZE button_count) {
    Solution solution;

    solution.button_count = button_count;
    solution.presses = (PRESSES)malloc(button_count*sizeof(PRESS_AMOUNT));

    empty_solution(&solution);

    return solution;
}

/*
 * @brief Create a new solution by parsing a string of numbers.
 *
 * Typically used in Unit tests.
 */
ExecutionStatus parse_solution_from_numbers(STRING input, Solution* solution) {
    USIZE count=0;
    NUMBER* array=(NUMBER*)malloc(MAX_BUTTONS * sizeof(NUMBER));

    ExecutionStatus status = parse_numbers(input, array, &count);

    if (status == SUCCESS) {
        if (solution->button_count == count) {
            for (USIZE index=0; index < count; index++) {
                if (array[index] > MAX_PRESSES) {
                    log_to_stderr(
                        ERROR,
                        "Incoming string has \x1b[31m\x1b[1m%u\x1b[0m at position \x1b[1m%u\x1b[22m, exceeding the limit of \x1b[1m%u\x1b[22m.",
                        array[index],
                        index,
                        MAX_PRESSES
                    );
                    status = PRESS_OVERFLOW;
                    break;
                }

                solution->presses[index] = (PRESS_AMOUNT)array[index];
            }
        } else {
            log_to_stderr(
                ERROR,
                "Solution has \x1b[1m%u\x1b[22m buttons, while incoming string has \x1b[31m\x1b[1m%u\x1b[0m.",
                solution->button_count,
                count
            );
            status = MISMATCHED_BUTTON_COUNT;
        }
    }

    free(array);
    return status;
}

/*
 * @brief Count the total number of presses in a `Solution`.
 */
PRESS_AMOUNT press_count(Solution* solution) {
    PRESS_AMOUNT count = 0;
    for (USIZE index=0; index<solution->button_count; index++) {
        count += solution->presses[index];
    }
    return count;
}

/*
 * @brief Add the `rhs` solution count to the `lhs`.
 *
 * If the button counts of both do not match, return `MISMATCHED_BUTTON_COUNT`.
 */
ExecutionStatus combined_solutions(Solution* lhs, Solution* rhs) {
    if (lhs->button_count != rhs->button_count) {
        log_to_stderr(
            ERROR,
            "Solutions have differing button counts: \x1b[1m%u\x1b[0m and \x1b[1m%u\x1b[0m.",
            lhs->button_count,
            rhs->button_count
        );
        return MISMATCHED_BUTTON_COUNT;
    }

    for (USIZE index=0; index < lhs->button_count; index++) {
        lhs->presses[index] += rhs->presses[index];
    }

    return SUCCESS;
}

/*
 * @brief Multiply all the presses of the given `Solution` by `amount`.
 *
 * There is no overflow protection in this function - be careful with its use.
 */
void multiply_solution(Solution* solution, USIZE amount) {
    for (USIZE index=0; index < solution->button_count; index++) {
        solution->presses[index] *= amount;
    }
}

/*
 * @brief Free a `Solution` from memory.
 */
void free_solution(Solution* solution) {
    free(solution->presses);
}
