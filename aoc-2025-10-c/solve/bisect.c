#include "bisect.h"
#include "brute.h"

/*
 * @brief Solve bigger `Scenario`s where DFS cannot work within reasonable time.
 */
ExecutionStatus _solve_by_bisection(
    Scenario* scenario,
    Vector* destination,
    USIZE current_depth,
    Solution* solution
) {
    display_vector(DEBUG, "Trying to bisect solve from origin to ", destination);

    // Step 0: Empty Vector protection
    if (is_empty_vector(destination)) {
        log_to_stderr(INFO, "Solving at depth \x1b[1m%u\x1b[0m not necessary, destination is empty.", current_depth);
        return SUCCESS;
    }

    log_to_stderr(INFO, "Bisection solver at depth \x1b[1m%u\x1b[22m.", current_depth);
    // Step 1: Skim the vector into two
    ExecutionStatus status, final_status;
    final_status = UNDETERMINED;
    // Note to self: `skim_vector_to_even` no longer mutates destination.
    Button skimmed = skim_vector_to_even(destination);

    // Step 2: If we have skimmed something, try solving it.
    if (!is_empty_button(&skimmed)) {
        Solution skimmed_solution = new_solution(scenario->button_count);
        Vector skimmed_solution_vector = new_vector_with_dimensions(scenario->dimensions);

        log_to_stderr(
            DEBUG,
            "Attempting to find a mask solution at depth \x1b[1m%u\x1b[0m.",
            current_depth
        );
        display_button(DEBUG, "Skimmed mask: ", &skimmed);
        display_vector(DEBUG, "Destination vector: ", destination);
        status = bfs_for_mask(scenario, &skimmed, &skimmed_solution, &skimmed_solution_vector);

        if (status == SUCCESS) {
            display_button(INFO, "BFS found a solution for skimmed mask: ", &skimmed);
            display_vector(INFO, "The solution is for a vector of ", &skimmed_solution_vector);
            status = subtract_vectors(destination, &skimmed_solution_vector);
            if (status == VECTOR_UNDERFLOW) {
                display_vector(WARN, "This won't work however, as it will underflow the destination of ", destination);
                display_vector(INFO, "Falling back to DFS to solve ", destination);

                status = dfs_from(scenario, destination, solution);
                // We will not need to look at `destination` any further; this status is final.
                final_status = status;
            } else if (status == SUCCESS) {
                status = combined_solutions(solution, &skimmed_solution);
            }
        } else if (status == NO_SOLUTION) {
            // FIXME What do we do????
        }

        free_solution(&skimmed_solution);
        free_vector(&skimmed_solution_vector);
    }

    // Step 3: If we haven't got a final status, we should bisect the remaining vector.
    if (final_status == UNDETERMINED) {
        if (!is_empty_vector(destination)) {
            display_vector(INFO, "Attempting to bisect ", destination);
            Solution bisected_solution = new_solution(scenario->button_count);

            // Now that we are sure our `destination` is integer divisible by `BALANCE_FACTOR`, we can start dividing it.
            status = divide_vector_by_scalar(destination, BALANCE_FACTOR);
            if (status==SUCCESS) {
                status = _solve_by_bisection(
                    scenario,
                    destination,
                    current_depth+1,
                    &bisected_solution
                );

                if (status==SUCCESS){
                    log_to_stderr(INFO, "Found a solution for bisected vector at depth \x1b[1m%u\x1b[0m.", current_depth);
                    // Scale the solution back up
                    multiply_solution(&bisected_solution, BALANCE_FACTOR);

                    final_status = combined_solutions(solution, &bisected_solution);
                } else {
                    final_status = status;
                }
            } else {
                final_status = status;
            }

            free_solution(&bisected_solution);
        } else {
            log_to_stderr(DEBUG, "No need to run DFS on destination, its empty.");
            final_status = SUCCESS;
        }
    }

    free_button(&skimmed);
    log_to_stderr(INFO, "Bisection solver finishing depth \x1b[1m%u\x1b[22m with status \x1b[1m%u\x1b[22m.", current_depth, final_status);
    return final_status;
}

/*
 * @brief Solve bigger `Scenario`s where DFS cannot work within reasonable time.
 */
ExecutionStatus solve_by_bisection(
    Scenario* scenario,
    Vector* destination,
    Solution* solution
) {
    return _solve_by_bisection(scenario, destination, 0, solution);
}

#ifdef UNIT_TEST
void assert_bisect_solve(
    STRING scenario_def,
    ExecutionStatus expected_status,
    STRING solution_def
) {
    assert_solver(
        "Bisect solver",
        solve_by_bisection,
        scenario_def,
        expected_status,
        solution_def
    );
}

void test_bisection() {
    // there are no buttons that say (1).
    assert_bisect_solve(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {0,0,0,0}",
        SUCCESS,
        "0,0,0,0,0,0"
    );

    // there are no buttons that say (1).
    assert_bisect_solve(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {0,1,0,0}",
        NO_SOLUTION,
        ""
    );

    // These are the examples from the task.
    assert_bisect_solve(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        SUCCESS,
        // The example's solution is different, but the button count is the same.
        // i.e. Not a good example.
        "1,2,0,4,0,3"
    );
    assert_bisect_solve(
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        SUCCESS,
        "2,5,0,5,0"
    );


    // // THESE DO NOT WORK - see README.md for why.
    // assert_bisect_solve(
    //     "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    //     SUCCESS,
    //     "5,0,5,1"
    // );
    // These are from the actual input.
    // With a much higher press count, this is approaching the limit of brute forcing;
    // but these are still runnable on CI.
    // assert_bisect_solve(
    //     "[###..] (0,1,2) (0,3,4) (0,3) (1,2,4) {13,20,20,8,16}",
    //     SUCCESS,
    //     "5,1,7,15"
    // );
    // assert_bisect_solve(
    //     "[.###] (0,1,2) (0,2) (2) (0,2,3) (0) {39,8,26,7}",
    //     SUCCESS,
    //     "8,11,0,7,13"
    // );
}
#endif
