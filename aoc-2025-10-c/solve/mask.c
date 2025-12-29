#include "mask.h"

/*
 * @brief Check whether a combination matches the mask.
 *
 * Returns `SUCCESS` if the combination matches, otherwise
 * returns `NO_SOLUTION`.
 *
 * @note This function requires a temporary vector
 * to avoid excessive Heap allocation.
 */
ExecutionStatus is_combination_matching_mask(
    Scenario* scenario,
    Button* mask,
    USIZE* combination,
    USIZE button_count,
    Vector* temp_vector
) {
    if (temp_vector->dimensions != scenario->dimensions) {
        log_to_stderr(
            ERROR,
            "Temporary Vector  has \x1b[31m\x1b[1m%u\x1b[0m dimensions but scenario has \x1b[1m%u\x1b[22m.",
            temp_vector->dimensions,
            scenario->dimensions
        );
        return MISMATCHED_DIMENSIONS;
    }

    empty_vector(temp_vector);
    for (USIZE index=0; index < button_count; index++) {
        if (combination[index] >= scenario->button_count) {
            log_to_stderr(
                ERROR,
                "Combination requires button #\x1b[31m\x1b[1m%u\x1b[0m but scenario only has \x1b[1m%u\x1b[22m buttons.",
                combination[index],
                scenario->button_count
            );
            return BUTTON_NOT_FOUND;
        }
        add_to_vector(temp_vector, &scenario->buttons[combination[index]]);
    }

    return is_vector_matching_mask(temp_vector, mask);
}

/*
 * @brief Try to solve a mask with a predetermined number of button presses.
 *
 * A mask will always only require a max press of 1 per button; as the buttons
 * operates in "toggle mode", i.e. the next press reverses the last.
 *
 * This greatly reduces the complexity of the problem, and we can safely brute
 * force this by never recycling any buttons.
 *
 * Ensure your `Solution` is empty before calling this function; it does not
 * reset it.
 */
ExecutionStatus solve_mask_using_solution_size(
    Scenario* scenario,
    Button* mask,
    USIZE button_count,
    Solution* solution,
    Vector* temp_vector
) {
    CombinationIterator iterator = iter_button_combinations(scenario, button_count);
    USIZE* combination;
    ExecutionStatus status = NO_SOLUTION;

    while ((combination = next_button_combination(&iterator)) != NULL) {
        status = is_combination_matching_mask(
            scenario, mask, combination, button_count, temp_vector
        );

        if (status == SUCCESS) {
            log_to_stderr_with_sep_without_ln(INFO, "Found solution of \x1b[1m%u\x1b[22m Buttons ", button_count);
            for (USIZE index=0; index<button_count; index++) {
                solution->presses[combination[index]]++;
                write_to_stderr("\x1b[1m#%d\x1b[22m", combination[index]);
                if (index+1<button_count) {
                    write_to_stderr("+");
                }
            }
            write_to_stderr("\n");
            break;
        } else if (status != NO_SOLUTION) {
            break;
        }
        // if we have no solution, keep looking.
    }

    if (status == NO_SOLUTION) {
        log_to_stderr(INFO, "Found no solution for mask using \x1b[1m%u\x1b[22m buttons.", button_count);
    }

    return status;
}

/*
 * @brief Brute force to solve a mask.
 *
 * `destination` will be wiped; and it must h
 */
ExecutionStatus bfs_for_mask(
    Scenario* scenario,
    Button* mask,
    Solution* solution,
    Vector* destination
) {
    if (destination->dimensions != scenario->dimensions) {
        log_to_stderr(ERROR, "Destination has \x1b[31m\x1b[1m%u\x1b[0m dimensions, but scenario has \x1b[1m%u\x1b[22.", destination->dimensions, scenario->dimensions);
        return MISMATCHED_DIMENSIONS;
    }
    if (solution->button_count != scenario->button_count) {
        return MISMATCHED_BUTTON_COUNT;
    }

    ExecutionStatus status = NO_SOLUTION;
    for (USIZE button_count=1; button_count<scenario->button_count; button_count++) {
        status = solve_mask_using_solution_size(scenario, mask, button_count, solution, destination);
        if (status != NO_SOLUTION) {
            break;
        }
    }

    return status;
}

// ======================================================================================================
// UNIT TEST

#ifdef UNIT_TEST
void assert_bfs_for_mask(
    STRING scenario_def,
    ExecutionStatus expected_status,
    STRING solution_def
) {
    log_to_stderr(INFO, "Running a BFS Mask test for \x1b[1m\"%s\"\x1b[22m.", scenario_def);

    Scenario scenario = new_scenario();

    ExecutionStatus parse_status = parse_line(scenario_def, &scenario);
    assert(parse_status==SUCCESS);

    Solution solution = new_solution(scenario.button_count);
    Vector actual_destination = new_vector_with_dimensions(scenario.dimensions);

    ExecutionStatus actual_status = bfs_for_mask(&scenario, &scenario.indicator, &solution, &actual_destination);

    if (actual_status == SUCCESS) {
        display_vector(INFO, "Solution arrived at ", &actual_destination);

        free_vector(&actual_destination);
    }

    bool success_status = actual_status == expected_status;
    log_to_stderr(
        DEBUG,
        "BFS Mask on \x1b[1m\"%s\"\x1b[22m got status \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
        scenario_def,
        31+success_status,
        actual_status,
        expected_status
    );
    assert(success_status);

    // Check the actual solution
    if (actual_status == SUCCESS) {
        // Step 2: Assert that the solution is the same.
        char buffer[MAX_LINE_LENGTH];
        // We have to copy the static string into a mutable buffer due to `strtok`.
        strncpy(buffer, solution_def, sizeof(buffer));

        Solution expected_solution = new_solution(scenario.button_count);

        parse_status = parse_solution_from_numbers(buffer, &expected_solution);
        assert(parse_status==SUCCESS);

        log_to_stderr(DEBUG, "Validating solution against scenario...");
        PRESS_AMOUNT actual_press_count = press_count(&solution);
        PRESS_AMOUNT expected_press_count = press_count(&expected_solution);
        bool success_press_count = actual_press_count == expected_press_count;
        log_to_stderr(
            DEBUG,
            "This solution required \x1b[%um\x1b[1m%u\x1b[0m presses, expecting \x1b[1m%u\x1b[22m.",
            31+success_press_count,
            actual_press_count,
            expected_press_count
        );

        bool success_all_presses=true;
        bool success_press;
        for (USIZE index=0; index<solution.button_count; index++) {
            success_press = expected_solution.presses[index] == solution.presses[index];

            LogLevel level;
            if (success_press) {
                level = DEBUG;
            } else {
                level = ERROR;
            }
            log_to_stderr(
                level,
                "At position \x1b[1m%u\x1b[22m, found \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
                index,
                31+success_press,
                solution.presses[index],
                expected_solution.presses[index]
            );
            if (!success_press) {
                success_all_presses=false;
            }
        }

        free_solution(&expected_solution);
        assert(success_all_presses);
    }

    free_scenario(&scenario);
    free_solution(&solution);
}

void test_bfs_for_mask() {
    assert_bfs_for_mask(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        SUCCESS,
        "0,1,0,1,0,0"
    );
    assert_bfs_for_mask(
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        SUCCESS,
        "0,0,1,1,1"
    );
    assert_bfs_for_mask(
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        SUCCESS,
        "0,1,1,0"
    );
}
#endif
