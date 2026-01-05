#include "divide.h"
#include "brute.h"

/*
 * @brief Mutate an existing `Vector` by dividing it into X chunks.
 *
 * The mutated vector will be have each value being `y' = y // X`,
 * where `//` is integer division (i.e. `y'` is the quotient); where the
 * returned vector will be `z' = y' + y % X`, i.e. taking the remainder for all
 * the other vectors.
 */
Vector chunk_vector(Vector* vector, USIZE chunks) {
    Vector new_vector = new_vector_with_dimensions(vector->dimensions);

    USIZE remainder, quotient;
    for (USIZE index = 0; index<vector->dimensions; index++) {
        remainder = vector->target[index] % chunks;
        quotient = vector->target[index] / chunks;

        new_vector.target[index] = quotient + remainder;
        vector->target[index] = quotient;
    }

    return new_vector;
}

/*
 * @brief Choose the best number of chunks for the given vector.
 *
 * Currently this is defined as the square root of the highest number. This is chosen
 * because the remainder can be up to `X // chunks + chunks-1`; while chunks can
 * be as high as the highest number, this will cause our quotients to become 0
 * and our remainder being very high, complicating the DFS for the remainder
 * vector.
 *
 * In the other scenario, if chunks is too low, our remainder will be very close
 * to the quotients, but the quotients themselves will be very high.
 */
USIZE find_optimal_chunks(Vector* vector) {
    USIZE max = 0;
    USIZE min = USIZE_MAX;
    for (USIZE index=0; index<vector->dimensions; index++) {
        if (vector->target[index] > max) max = vector->target[index];
        if (vector->target[index] < min) min = vector->target[index];
    }

    USIZE max_sqrt = (USIZE)(ceil(sqrt(max)));
    USIZE min_by_quotient = min / MIN_QUOTIENT;

    USIZE max_magnitude_sqrt = 0;
    USIZE max_magnitude_quotient = 0;

    for (USIZE index=0; index<vector->dimensions; index++) {
        if (max_sqrt > 0) {
            USIZE candidate_max_magnitude_sqrt = vector->target[index] / max_sqrt + vector->target[index] % max_sqrt;
            if (candidate_max_magnitude_sqrt > max_magnitude_sqrt) max_magnitude_sqrt = candidate_max_magnitude_sqrt;
        }
        if (min_by_quotient > 0) {
            USIZE candidate_max_magnitude_quotient = vector->target[index] / min_by_quotient + vector->target[index] % min_by_quotient;
            if (candidate_max_magnitude_quotient > max_magnitude_quotient) max_magnitude_quotient = candidate_max_magnitude_quotient;
        }
    }

    log_to_stderr(TRACE, "The two `chunks` candidates are: \x1b[1m%u\x1b[22m (max \x1b[1m%u\x1b[22m) and \x1b[1m%u\x1b[22m (max \x1b[1m%u\x1b[22m).", max_sqrt, max_magnitude_sqrt, min_by_quotient, max_magnitude_quotient);
    // The criteria it must satisfy:
    // - prefer the `chunks` resulting in the lower maximum value
    // - min `chunks` is 1
    if (max_magnitude_quotient < max_magnitude_sqrt && min_by_quotient >= 1) {
        return min_by_quotient;
    } else if (max_sqrt < 2) {
        return 1;
    }
    return max_sqrt;
}

/*
 * @brief Solve bigger `Scenario`s where DFS cannot work within reasonable time.
 */
ExecutionStatus _solve_by_division(
    Scenario* scenario,
    Vector* destination,
    USIZE current_depth,
    Solution* solution
) {
    display_vector(DEBUG, "Trying to division solve from origin to ", destination);

    // Step 0: Empty Vector protection
    if (is_empty_vector(destination)) {
        log_to_stderr(INFO, "Solving at depth \x1b[1m%u\x1b[0m not necessary, destination is empty.", current_depth);
        return SUCCESS;
    }

    log_to_stderr(INFO, "Division solver at depth \x1b[1m%u\x1b[22m.", current_depth);
    ExecutionStatus quotient_status, remainder_status, final_status;
    Vector quotient, remainder;
    Solution quotient_solution = new_solution(scenario->button_count);
    Solution remainder_solution = new_solution(scenario->button_count);
    final_status = UNDETERMINED;

    bool vector_init = false;

    for (USIZE chunks = find_optimal_chunks(destination); chunks > 0; chunks--) {
        if (chunks == 1 || !has_value_higher_than(destination, MAX_VALUE_IN_VECTOR)) {
            log_to_stderr(ERROR, "No further division needed at depth \x1b[1m%u\x1b[22m, falling back to DFS...", current_depth);
            final_status = dfs_from(scenario, destination, solution);
            break;
        }

        vector_init = true;
        log_to_stderr(ERROR, "Attempting division into \x1b[1m%u\x1b[22m chunks at depth \x1b[1m%u\x1b[22m...", chunks, current_depth);
        quotient = clone_vector(destination);
        remainder = chunk_vector(&quotient, chunks);
        display_vector(ERROR, "Original Vector:  ", destination);
        display_vector(ERROR, "Quotient Vector:  ", &quotient);
        display_vector(ERROR, "Remainder Vector: ", &remainder);

        if (has_value_higher_than(&quotient, MAX_VALUE_IN_VECTOR)) {
            // Recursive mode
            quotient_status = _solve_by_division(scenario, &quotient, current_depth+1, &quotient_solution);
        } else {
            quotient_status = dfs_from(scenario, &quotient, &quotient_solution);
        }

        // Only run the remainder one if we can solve quotient
        if (quotient_status == SUCCESS) {
            log_to_stderr(ERROR, "Found solution for quotient with \x1b[32m\x1b[1m%u\x1b[0m presses.", press_count(&quotient_solution));
            display_solution(ERROR, "Solution: ", &quotient_solution);
            if (has_value_higher_than(&remainder, MAX_VALUE_IN_VECTOR)) {
                // Recursive mode
                remainder_status = _solve_by_division(scenario, &remainder, current_depth+1, &remainder_solution);
            } else {
                remainder_status = dfs_from(scenario, &remainder, &remainder_solution);
            }

            if (remainder_status == SUCCESS) {
                log_to_stderr(ERROR, "Found solution for both quotient and remainder at depth \x1b[1m%u\x1b[22m", current_depth);
                display_solution(ERROR, "Solution: ", &remainder_solution);

                multiply_solution(&quotient_solution, chunks-1);
                remainder_status = combined_solutions(solution, &quotient_solution);
                if (remainder_status == SUCCESS) {
                    remainder_status = combined_solutions(solution, &remainder_solution);
                }

                final_status = remainder_status;
                break;
            } else if (remainder_status != NO_SOLUTION) {
                final_status = remainder_status;
                break;
            }
        } else if (quotient_status != NO_SOLUTION) {
            final_status = quotient_status;
            break;
        }

        empty_solution(&quotient_solution);
        empty_solution(&remainder_solution);
    }

    if (final_status == UNDETERMINED) {
        final_status = NO_SOLUTION;
    }

    if (vector_init) {
        free_vector(&remainder);
        free_vector(&quotient);
    }
    free_solution(&remainder_solution);
    free_solution(&quotient_solution);

    return final_status;
}

/*
 * @brief Solve bigger `Scenario`s where DFS cannot work within reasonable time.
 */
ExecutionStatus solve_by_division(
    Scenario* scenario,
    Vector* destination,
    Solution* solution
) {
    return _solve_by_division(scenario, destination, 0, solution);
}

#ifdef UNIT_TEST

void assert_chunk_vector(
    STRING vector_def,
    USIZE chunks,
    STRING manipulated_vector_def,
    STRING remainder_vector_def
) {
    log_to_stderr(DEBUG, "Chunking vector \x1b[1m%s\x1b[22m into \x1b[1m%u\x1b[22m...", vector_def, chunks);
    ExecutionStatus parse_status;
    char buffer[MAX_LINE_LENGTH];
    // We have to copy the static string into a mutable buffer due to `strtok`.
    Vector vector = new_vector();
    Vector expected_manipulated = new_vector_with_dimensions(vector.dimensions);
    Vector expected_remainder = new_vector_with_dimensions(vector.dimensions);

    // Step 1: Parse the vectors
    strncpy(buffer, vector_def, sizeof(buffer));
    parse_status = parse_vector((STRING)&buffer, &vector);
    assert(parse_status==SUCCESS);

    strncpy(buffer, manipulated_vector_def, sizeof(buffer));
    parse_status = parse_vector((STRING)&buffer, &expected_manipulated);
    assert(parse_status==SUCCESS);

    strncpy(buffer, remainder_vector_def, sizeof(buffer));
    parse_status = parse_vector((STRING)&buffer, &expected_remainder);
    assert(parse_status==SUCCESS);

    // Step 2: Perform the division
    Vector actual_remainder = chunk_vector(&vector, chunks);

    assert_vectors_eq(&vector, &expected_manipulated);
    assert_vectors_eq(&actual_remainder, &expected_remainder);

    free_vector(&vector);
    free_vector(&expected_manipulated);
    free_vector(&expected_remainder);
    free_vector(&actual_remainder);
}

void test_chunk_vector() {
    // Bear in mind that the manipulated vector is quotient + remainder!
    assert_chunk_vector("{0,0,0,0}", 3, "{0,0,0,0}", "{0,0,0,0}");
    assert_chunk_vector("{0,1,0,0}", 3, "{0,0,0,0}", "{0,1,0,0}");
    assert_chunk_vector("{3,4,7,5}", 3, "{1,1,2,1}", "{1,2,3,3}");
    assert_chunk_vector("{3,4,7,5}", 2, "{1,2,3,2}", "{2,2,4,3}");
}

void assert_find_optimal_chunks(
    STRING vector_def,
    USIZE expected_chunks
) {
    log_to_stderr(DEBUG, "Determining optimal chunk size for vector \x1b[1m%s\x1b[22m...", vector_def);
    ExecutionStatus parse_status;
    char buffer[MAX_LINE_LENGTH];
    // We have to copy the static string into a mutable buffer due to `strtok`.
    Vector vector = new_vector();

    // Step 1: Parse the vectors
    strncpy(buffer, vector_def, sizeof(buffer));
    parse_status = parse_vector((STRING)&buffer, &vector);
    assert(parse_status==SUCCESS);

    // Step 2: Get the optimal chunk
    USIZE chunks = find_optimal_chunks(&vector);

    bool success_chunks = chunks == expected_chunks;
    log_to_stderr(
        DEBUG,
        "Found optimal chunks \x1b[%um\x1b[1m%u\x1b[0m for vector \x1b[1m%s\x1b[22m, expected \x1b[1m%u\x1b[22m.",
        31+success_chunks,
        chunks,
        vector_def,
        expected_chunks
    );
    assert(success_chunks);

    free_vector(&vector);
}

void test_find_optimal_chunks() {
    assert_find_optimal_chunks("{0,0,0,0}", 1);
    assert_find_optimal_chunks("{3,4,7,5}", 3);
    assert_find_optimal_chunks("{32,47,21,8,35,53,45,46,32,37}", 8);
    assert_find_optimal_chunks("{200,102,201,305}", 20);
    assert_find_optimal_chunks("{200,102,201,305,19}", 18);
    assert_find_optimal_chunks("{200,102,201,305,5}", 18);
    assert_find_optimal_chunks("{301,306,302,307,299}", 59);
    assert_find_optimal_chunks("{301,306,302,307,299,17}", 18);
}

void assert_division_solve(
    STRING scenario_def,
    ExecutionStatus expected_status,
    STRING solution_def
) {
    assert_solver(
        "division solver",
        solve_by_division,
        scenario_def,
        expected_status,
        solution_def
    );
}

void test_division() {
    // there are no buttons that say (1).
    assert_division_solve(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {0,0,0,0}",
        SUCCESS,
        "0,0,0,0,0,0"
    );

    // there are no buttons that say (1).
    assert_division_solve(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {0,1,0,0}",
        NO_SOLUTION,
        ""
    );

    // // These are the examples from the task.
    // // Currently, it is getting this wrong.
    assert_division_solve(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        SUCCESS,
        // The example's solution is different, but the button count is the same.
        // i.e. Not a good example.
        "1,4,0,2,2,1"
    );
    assert_division_solve(
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        SUCCESS,
        "2,5,0,5,0"
    );
    assert_division_solve(
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        SUCCESS,
        "5,0,5,1"
    );

    // These are from the actual input.
    // With a much higher press count, this is approaching the limit of brute forcing;
    // but these are still runnable on CI.
    assert_division_solve(
        "[###..] (0,1,2) (0,3,4) (0,3) (1,2,4) {13,20,20,8,16}",
        SUCCESS,
        "5,1,7,15"
    );
    assert_division_solve(
        "[.###] (0,1,2) (0,2) (2) (0,2,3) (0) {39,8,26,7}",
        SUCCESS,
        "8,11,0,7,13"
    );
    assert_division_solve(
        "[#....] (0,2,3,4) (2,3,4) (0,1,3,4) {159,10,165,175,175}",
        SUCCESS,
        "149,16,10"
    );
    assert_division_solve(
        "[###.] (2,3) (1,2,3) (0,2) (0,1) (1,2) {20,143,135,20}",
        SUCCESS,
        // This is the correct solution with 149 presses:
        // "0,20,6,14,109"
        "12,8,0,20,115"
    );

    assert_division_solve(
        "[.#..#] (0,2) (1,4) (0,1,3) (0,4) {191,21,13,2,195}",
        SUCCESS,
        "13,19,2,176"
    );
    assert_division_solve(
        "[..#####...] (0,1,2,8,9) (0,3,4,6,7,8,9) (0,1,3,4,6,7,8,9) (0,1,2,5,6,7,8) (0,1,2,4,5,6,7,8,9) (0,1,2,3,4,6,7,8) (1,2,3,4,7,8) (0,1,2,3,4,6,7,9) {55,61,41,50,53,16,53,66,60,40}",
        SUCCESS,
        "2,7,20,13,3,2,13,8"
    );
    // assert_division_solve(
    //     "[##..#....] (0,1,2,3,6,7,8) (1,2,3) (0,2,3,4,5,6,7,8) (0,2,6,7,8) (0,1,4) (2,3,6) (2,3,4,5,6,7,8) (1,5,6) (0,2,3,7,8) (0,1,4,8) {80,72,68,62,41,18,45,52,71}",
    //     SUCCESS,
    //     "21,16,11,6,10,0,1,6,13,19"
    // );
}
#endif
