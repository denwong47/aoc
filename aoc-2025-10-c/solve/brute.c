#include "brute.h"
#include "common.h"

/*
 * @brief Perform a DFS from the current position.
 *
 * This does not actually guarantee the shortest solution to the problem;
 * but `rank_buttons_by_euclidean_distance` should do a reasonable job
 * at optimising that.
 */
ExecutionStatus _dfs_from(
    Scenario* scenario,
    Vector* current_position,
    Vector* destination,
    USIZE current_depth,
    Solution* solution
) {
    // Step 0: Empty Vector protection
    if (is_empty_vector(destination) && is_empty_vector(current_position)) {
        log_to_stderr(INFO, "DFS at depth \x1b[1m%u\x1b[0m not necessary, destination is empty.", current_depth);
        return SUCCESS;
    }

    log_to_stderr(INFO, "DFS at depth \x1b[1m%u\x1b[0m...", current_depth);
    ExecutionStatus status;
    ExecutionStatus final_status = NO_SOLUTION;
    Order order = new_order(scenario->button_count);

    // Step 1: Setup and rank the buttons we could be pressing
    status = rank_buttons_by_euclidean_distance(
        scenario,
        current_position,
        destination,
        &order
    );

    if (status!=SUCCESS) {
        free_order(&order);
        return status;
    }

    // Step 2: DFS through each candidate button.
    Button* button;
    for (USIZE index=0; index < order.count; index++) {
        // Safeguard against freak button IDs
        if (order.ids[index] >= scenario->button_count) {
            log_to_stderr(
                ERROR,
                "Button \x1b[31m\x1b[1m%d\x1b[0m is not a valid button in this scenario with \x1b[22m%d\x1b[0m buttons.",
                order.ids[index],
                scenario->button_count
            );
            return BUTTON_NOT_FOUND;
        }

        button = &scenario->buttons[order.ids[index]];

        // Step 3: Add the button on top of the current position, and see if we have a solution.
        solution->presses[order.ids[index]]++;
        status = add_to_vector(current_position, button);
        if (status!=SUCCESS) {
            // Dimensions mismatch, irrecoverable.
            final_status = status;
            break;
        }

        // If we have arrived at the solution, immediately return.
        if (are_vectors_eq(current_position, destination)) {
            final_status = SUCCESS;
            break;
        }

        if (current_depth >= MAX_PRESSES) {
            log_to_stderr(WARN, "Maximum depth of \x1b[1m%u\x1b[0m reached, stopping recursion.", MAX_PRESSES);
            status = NO_SOLUTION;
        } else {
            // We are not there yet, continue DFS...
            status = _dfs_from(
                scenario,
                current_position,
                destination,
                current_depth+1,
                solution
            );
        }

        // Step 4: Check if we have a solution from an inner depth.
        if (status==NO_SOLUTION) {
            // Oops, backtrack!
            solution->presses[order.ids[index]]--;
            status = subtract_from_vector(current_position, button);
        } else {
            // If any error or SUCCESS, just return.
            final_status = status;
            break;
        }
    }

    free_order(&order);
    if (final_status==NO_SOLUTION) {
        log_to_stderr(DEBUG, "DFS at depth \x1b[1m%u\x1b[0m found no solution, backtracking.", current_depth);
    } else if (final_status==SUCCESS) {
        log_to_stderr(INFO, "DFS at depth \x1b[1m%u\x1b[0m found a solution, passing back up the chain.", current_depth);
    }
    return final_status;
}


/*
 * @brief Perform a DFS from the current position.
 *
 * This does not actually guarantee the shortest solution to the problem;
 * but `rank_buttons_by_euclidean_distance` should do a reasonable job
 * at optimising that.
 *
 * `destination` needs to be provided explicitly, as one might need to find
 * an intermediate point instead of the scenario's destination.
 */
ExecutionStatus dfs_from(
    Scenario* scenario,
    Vector* destination,
    Solution* solution
) {
    Vector current_position = new_vector_with_dimensions(scenario->dimensions);
    empty_vector(&current_position);

    ExecutionStatus status = _dfs_from(
        scenario,
        &current_position,
        destination,
        0,
        solution
    );
    free_vector(&current_position);
    return status;
}

#ifdef UNIT_TEST
void assert_dfs_from(
    STRING scenario_def,
    ExecutionStatus expected_status,
    STRING solution_def
) {
    assert_solver(
        "DFS",
        dfs_from,
        scenario_def,
        expected_status,
        solution_def
    );
}

void test_brute() {
    // there are no buttons that say (1).
    assert_dfs_from(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {0,0,0,0}",
        SUCCESS,
        "0,0,0,0,0,0"
    );

    // there are no buttons that say (1).
    assert_dfs_from(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {0,1,0,0}",
        NO_SOLUTION,
        ""
    );

    // These are the examples from the task.
    assert_dfs_from(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        SUCCESS,
        // The example's solution is different, but the button count is the same.
        // i.e. Not a good example.
        "1,4,0,2,2,1"
    );
    assert_dfs_from(
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        SUCCESS,
        "2,5,0,5,0"
    );
    assert_dfs_from(
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        SUCCESS,
        "5,0,5,1"
    );
    // These are from the actual input.
    // With a much higher press count, this is approaching the limit of brute forcing;
    // but these are still runnable on CI.
    assert_dfs_from(
        "[###..] (0,1,2) (0,3,4) (0,3) (1,2,4) {13,20,20,8,16}",
        SUCCESS,
        "5,1,7,15"
    );
    assert_dfs_from(
        "[.###] (0,1,2) (0,2) (2) (0,2,3) (0) {39,8,26,7}",
        SUCCESS,
        "8,11,0,7,13"
    );
}
#endif
