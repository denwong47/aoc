#include "validate.h"

/*
 * @brief Build a vector using the buttons from a `Scenario`.
 *
 * Typically used to validate the solution indeed can arrive
 * at the destination.
 */
ExecutionStatus compile_vector_from_solution(
    Scenario* scenario,
    Solution* solution,
    Vector* vector
) {
    // Step 1: Validate inputs.
    if (solution->button_count != scenario->button_count) {
        log_to_stderr(
            ERROR,
            "Solution has \x1b[31m\x1b[1m%u\x1b[0m buttons, while scenario has \x1b[1m%u\x1b[0m.",
            solution->button_count,
            scenario->button_count
        );
        return MISMATCHED_BUTTON_COUNT;
    }

    if (vector->capacity < scenario->dimensions) {
        log_to_stderr(
            ERROR,
            "Scenario has \x1b[31m\x1b[1m%u\x1b[0m dimensions, while vector only has \x1b[1m%u\x1b[0m capacity.",
            scenario->dimensions,
            vector->capacity
        );
        return INSUFFICIENT_CAPACITY;
    }

    // Step 2: Empty the Vector first
    for (USIZE index=0; index < scenario->dimensions; index++) {
        vector->target[index] = 0;
    }
    vector->dimensions = scenario->dimensions;

    // Step 3: Accumulate the button presses.
    for (USIZE button_index=0; button_index < solution->button_count; button_index++) {
        for (USIZE index=0; index < scenario->dimensions; index++) {
            USIZE effect = scenario->buttons[button_index].effect[index];
            USIZE press_count = solution->presses[button_index];
            vector->target[index] += press_count * effect;
        }
    }

    return SUCCESS;
}
