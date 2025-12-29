#include "common.h"

/*
 * @brief Calculate the squared euclidean distance of the resultant vector of a combination.
 *
 * This does not mutate `vector`; it only calculates the resultant distance. Useful for
 * calculating heuristics before commiting to the next step of a DFS.
 */
ExecutionStatus square_euclidean_length_of_vector_with_button_to_target(Vector* vector, Button* button, Vector* destination, DISTANCE* distance) {
    if (vector->dimensions != button->dimensions || vector->dimensions != destination->dimensions) {
        log_to_stderr(
            ERROR,
            "Vector, button or destination have differing dimensions: \x1b[1m%u\x1b[0m vs \x1b[1m%u\x1b[0m vs \x1b[1m%u\x1b[0m.",
            vector->dimensions,
            button->dimensions,
            destination->dimensions
        );
        return MISMATCHED_DIMENSIONS;
    }

    *distance = 0;
    DISTANCE combined;
    for (USIZE index=0; index < vector->dimensions; index++) {
        combined = vector->target[index] + button->effect[index];
        if (combined > destination->target[index]) {
            log_to_stderr(
                TRACE,
                "Distance underflowed: \x1b[31m\x1b[1m%u\x1b[22m + \x1b[1m%u\x1b[0m > \x1b[1m%u\x1b[0m.",
                vector->target[index],
                button->effect[index],
                destination->target[index]
            );
            return DISTANCE_UNDERFLOW;
        }
        combined -= destination->target[index];
        (*distance)+=(combined * combined);
    }

    return SUCCESS;
}

/*
 * Internal function to compare two `ButtonComparator`s.
 */
int compare_button_comparators(const void *lhs, const void *rhs) {
    const DISTANCE lhs_distance = ((const ButtonComparator *)lhs)->distance;
    const DISTANCE rhs_distance = ((const ButtonComparator *)rhs)->distance;

    if (lhs_distance < rhs_distance) return -1;
    if (lhs_distance > rhs_distance) return 1;
    return 0;
}

/*
 * @brief Order the button IDs in ascending distances.
 *
 * This trusts that `distances` is of a sufficient length from upstream.
 */
void sort_button_ids_by_provided_distances(
    Order* order,
    DISTANCE* distances
) {
    ButtonComparator* comparators = (ButtonComparator*)malloc(order->count * sizeof(ButtonComparator));

    // Step 1: build the Comparators
    for (USIZE index=0; index < order->count; index++) {
        ButtonComparator comparator;
        comparator.button_id = order->ids[index];
        comparator.distance = distances[comparator.button_id];
        comparators[index] = comparator;
    }

    // Step 2: sort them
    qsort(comparators, order->count, sizeof(ButtonComparator), compare_button_comparators);

    // Step 3: rewrite the button IDs
    for (USIZE index=0; index < order->count; index++) {
        order->ids[index] = comparators[index].button_id;
    }

    free(comparators);
}

/*
 * @brief Sort the available buttons by the remaining vector to the target.
 *
 * This resets anything currently in `order`; it treats the existing object
 * only as a memory allocation.
 *
 * `desination` is explicitly supplied here, as we may not be aiming for the overall
 * target upon bisection. `scenario->vector` is _NOT_ used.
 */
ExecutionStatus rank_buttons_by_euclidean_distance(
    Scenario* scenario,
    Vector* current_position,
    Vector* destination,
    Order* order
) {
    order->count = 0;
    if (order->capacity < scenario->button_count) {
        log_to_stderr(ERROR, "Insufficient capacity in `Order` object to record the \x1b[1m%u\x1b[0m button options.", scenario->button_count);
        return INSUFFICIENT_CAPACITY;
    }

    DISTANCE* distances = (DISTANCE*)malloc(scenario->button_count * sizeof(DISTANCE));
    Vector new_vector;
    ExecutionStatus status;

    // Step 1: Populate `distances` and `order->ids`
    for (USIZE button_index=0; button_index < scenario->button_count; button_index++) {
        status = square_euclidean_length_of_vector_with_button_to_target(
            current_position,
            &scenario->buttons[button_index],
            destination,
            &distances[button_index]
        );
        if (status == SUCCESS) {
            log_to_stderr(TRACE, "Button \x1b[1m%u\x1b[0m have a remaining sq-euclid distance of \x1b[32m%lu\x1b[0m. Adding as candidate.", button_index, distances[button_index]);
            order->ids[order->count++] = button_index;
        } else if (status == DISTANCE_UNDERFLOW) {
            log_to_stderr(TRACE, "Button \x1b[1m%u\x1b[0m would have exceeded destination ignoring.", button_index);
        } else {
            free(distances);
            return status;
        }
    }

    // Step 2: If there are any candidates at all, sort them by the distances.
    if (order->count > 0) {
        sort_button_ids_by_provided_distances(order, distances);
    }

    free(distances);
    return SUCCESS;
}

// ======================================================================================================
// UNIT TEST


#ifdef UNIT_TEST

void assert_square_euclidean_length_of_vector_with_button_to_target(
    STRING vector_def,
    STRING button_def,
    STRING destination_def,
    ExecutionStatus expected_status,
    DISTANCE expected_distance
) {
    ExecutionStatus parse_status;
    char buffer[MAX_LINE_LENGTH];
    // We have to copy the static string into a mutable buffer due to `strtok`.
    Vector vector = new_vector_with_dimensions(3);
    Button button = new_button();
    Vector destination = new_vector();

    strncpy(buffer, vector_def, sizeof(buffer));
    parse_status = parse_vector((STRING)&buffer, &vector);
    assert(parse_status==SUCCESS);

    strncpy(buffer, button_def, sizeof(buffer));
    parse_status = parse_button((STRING)&buffer, &button, vector.dimensions);
    assert(parse_status==SUCCESS);

    strncpy(buffer, destination_def, sizeof(buffer));
    parse_status = parse_vector((STRING)&buffer, &destination);
    assert(parse_status==SUCCESS);

    // Actual tests
    DISTANCE distance;
    ExecutionStatus actual_status = square_euclidean_length_of_vector_with_button_to_target(
        &vector,
        &button,
        &destination,
        &distance
    );

    bool success_status = actual_status == expected_status;
    log_to_stderr(
        DEBUG,
        "Operating on \x1b[1m%s\x1b[22m + \x1b[1m%s\x1b[22m -> \x1b[1m%s\x1b[22m, got status \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
        vector_def,
        button_def,
        destination_def,
        31+success_status,
        actual_status,
        expected_status
    );
    assert(success_status);

    if (expected_status != SUCCESS) {
        return;
    }

    bool success_distance = distance == expected_distance;
    log_to_stderr(
        DEBUG,
        "From \x1b[1m%s\x1b[22m + \x1b[1m%s\x1b[22m to \x1b[1m%s\x1b[22m, got distance \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
        vector_def,
        button_def,
        destination_def,
        31+success_status,
        distance,
        expected_distance
    );
    assert(success_distance);
}

void assert_sort_button_ids_by_provided_distances(
    STRING order_def,
    STRING distance_def,
    USIZE* expected_order
) {
    log_to_stderr(
        DEBUG,
        "Sorting buttons \x1b[1m[%s]\x1b[22m by distances of \x1b[1m{%s}\x1b[22m.",
        order_def,
        distance_def
    );
    ExecutionStatus parse_status;
    char buffer[MAX_LINE_LENGTH];

    // Step 1: Parse the button IDs.
    strncpy(buffer, order_def, sizeof(buffer));
    USIZE order_count;
    NUMBER* numbers = (NUMBER*)malloc(MAX_DIM * sizeof(NUMBER));
    parse_status = parse_numbers(buffer, numbers, &order_count);
    assert(parse_status == SUCCESS);

    // Step 2: Setup the `Order` struct.
    Order order = new_order(order_count);
    for (USIZE index=0; index < order_count; index++) {
        order.ids[index] = (USIZE)numbers[index];
    }
    order.count = order_count;

    // Step 3: Parse the distances.
    strncpy(buffer, distance_def, sizeof(buffer));
    USIZE distance_count;
    parse_status = parse_numbers(buffer, numbers, &distance_count);
    assert(parse_status == SUCCESS);

    DISTANCE* distances = (DISTANCE*)malloc(distance_count * sizeof(DISTANCE));
    for (USIZE index=0; index < distance_count; index++) {
        distances[index] = (DISTANCE)numbers[index];
    }

    // Step 4: Order the IDs.
    sort_button_ids_by_provided_distances(&order, distances);

    // Step 5: Assert the IDs.
    for (USIZE index=0; index < order.count; index++) {
        USIZE actual_id = order.ids[index];
        USIZE expected_id = expected_order[index];
        bool success_id = actual_id == expected_id;

        log_to_stderr(
            DEBUG,
            "At position \x1b[1m%u\x1b[0m, found Button #\x1b[%um\x1b[1m%u\x1b[0m; expected #\x1b[1m%u\x1b[22m.",
            index,
            31+success_id,
            actual_id,
            expected_id
        );
        assert(success_id);
    }

    free(order.ids);
    free(distances);
    free(numbers);
}

void assert_rank_buttons_by_euclidean_distance(
    STRING scenario_def,
    STRING current_position_def,
    STRING destination_def,
    ExecutionStatus expected_status,
    USIZE expected_count,
    USIZE* expected_order
) {
    log_to_stderr(
        DEBUG,
        "Ranking buttons from \x1b[1m%s\x1b[22m by distances from \x1b[1m%s\x1b[22m to \x1b[1m%s\x1b[22m.",
        scenario_def,
        current_position_def,
        destination_def
    );

    // Step 1: Parse the Scenario.
    Scenario scenario = new_scenario();
    ExecutionStatus parse_status = parse_line(scenario_def, &scenario);
    assert(parse_status == SUCCESS);

    // Step 2: Parse the vectors.
    char buffer[MAX_LINE_LENGTH];
    Vector current_position = new_vector();
    strncpy(buffer, current_position_def, sizeof(buffer));
    parse_status = parse_vector(buffer, &current_position);
    assert(parse_status == SUCCESS);

    Vector destination = new_vector();
    strncpy(buffer, destination_def, sizeof(buffer));
    parse_status = parse_vector(buffer, &destination);
    assert(parse_status == SUCCESS);

    // Step 3: Rank the buttons.
    ExecutionStatus actual_status;
    // This _could_ use scenario.button_count, but we want to test INSUFFICIENT_CAPACITY
    Order order = new_order(MAX_BUTTONS);
    actual_status = rank_buttons_by_euclidean_distance(&scenario, &current_position, &destination, &order);

    // Step 4: Assert the status.
    bool success_status = actual_status == expected_status;
    log_to_stderr(
        DEBUG,
        "Ranking buttons from \x1b[1m%s\x1b[22m to \x1b[1m%s\x1b[22m, got status \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
        current_position_def,
        destination_def,
        31+success_status,
        actual_status,
        expected_status
    );
    assert(success_status);

    if (expected_status != SUCCESS) {
        return;
    }

    // Step 5: Assert the count.
    bool success_count = order.count == expected_count;
    log_to_stderr(
        DEBUG,
        "Ranking buttons from \x1b[1m%s\x1b[22m to \x1b[1m%s\x1b[22m, found \x1b[%um\x1b[1m%u\x1b[0m buttons, expecting \x1b[1m%u\x1b[22m.",
        current_position_def,
        destination_def,
        31+success_count,
        order.count,
        expected_count
    );
    assert(success_count);

    // Step 6: Assert the order.
    bool success_number;
    for (USIZE index=0; index < order.count; index++) {
        success_number = order.ids[index] == expected_order[index];
        log_to_stderr(
            DEBUG,
            "At position \x1b[1m%u\x1b[22m, found Button #\x1b[%um\x1b[1m%u\x1b[0m, expecting #\x1b[1m%u\x1b[22m.",
            index,
            31+success_number,
            order.ids[index],
            expected_order[index]
        );
        assert(success_number);
    }

    free_scenario(&scenario);
    free(current_position.target);
    free(destination.target);
    free(order.ids);
}

int test_euclidean_lengths() {
    assert_square_euclidean_length_of_vector_with_button_to_target(
        "{1,2,3}",
        "(0,2)",
        "{3,3,6}",
        SUCCESS,
        1 + 1 + 2*2
    );
    assert_square_euclidean_length_of_vector_with_button_to_target(
        "{1,2,3}",
        "(1)",
        "{3,3,6}",
        SUCCESS,
        2*2 + 0 + 3*3
    );
    assert_square_euclidean_length_of_vector_with_button_to_target(
        "{1,2,3}",
        "(1)",
        "{3,3,6,4}",
        MISMATCHED_DIMENSIONS,
        0
    );
    assert_square_euclidean_length_of_vector_with_button_to_target(
        "{1,2,3}",
        "(1)",
        "{3,2,6}",
        DISTANCE_UNDERFLOW,
        0
    );
    assert_square_euclidean_length_of_vector_with_button_to_target(
        "{1,2,3}",
        "(1)",
        "{1,3,3}",
        SUCCESS,
        0
    );
    return 0;
}

int test_sort_button_ids() {
    USIZE expected_descending[] = {5,4,3,2,1,0};
    assert_sort_button_ids_by_provided_distances(
        "0,1,2,3,4,5",
        "55,44,33,22,11,0",
        expected_descending
    );

    USIZE expected_ascending[] = {0,1,2,3,4,5};
    assert_sort_button_ids_by_provided_distances(
        "0,1,2,3,4,5",
        "0,11,22,33,44,55",
        expected_ascending
    );

    USIZE expected_subset[] = {3,2,0,5};
    assert_sort_button_ids_by_provided_distances(
        "5,3,0,2", // Order does not matter
        "50,0,20,10,60,80",
        expected_subset
    );

    USIZE expected_nothing[] = {};
    assert_sort_button_ids_by_provided_distances(
        "",
        "50,0,20,10,60,80",
        expected_nothing
    );

    USIZE expected_one[] = {5};
    assert_sort_button_ids_by_provided_distances(
        "5",
        "50,0,20,10,60,80",
        expected_one
    );

    return 0;
}

int test_rank_buttons_by_euclidean_distance() {
    USIZE* invalid_array;

    USIZE example_1_from_origin_to_target[] = {1,3,5,0,4,2};
    assert_rank_buttons_by_euclidean_distance(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        "{0,0,0,0}",
        "{3,5,4,7}",
        SUCCESS,
        sizeof(example_1_from_origin_to_target) / sizeof(USIZE),
        example_1_from_origin_to_target
    );

    USIZE example_1_from_origin_to_close_neighbour[] = {3,0,2};
    assert_rank_buttons_by_euclidean_distance(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        "{0,0,0,0}",
        "{0,0,1,1}",
        SUCCESS,
        sizeof(example_1_from_origin_to_close_neighbour) / sizeof(USIZE),
        example_1_from_origin_to_close_neighbour
    );

    USIZE example_1_from_middle_to_target[] = {1,3,0,2};
    assert_rank_buttons_by_euclidean_distance(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        "{3,3,2,4}",
        "{3,5,4,7}", // Actual {0,2,2,3}
        SUCCESS,
        sizeof(example_1_from_middle_to_target) / sizeof(USIZE),
        example_1_from_middle_to_target
    );
    return 0;
}

void assert_solver(
    STRING solver_name,
    ExecutionStatus (* solver)(Scenario*, Vector*, Solution*),
    STRING scenario_def,
    ExecutionStatus expected_status,
    STRING solution_def
) {
    log_to_stderr(INFO, "Running an assertion test using \x1b[1m%s\x1b[22m.", solver_name);

    Scenario scenario = new_scenario();

    ExecutionStatus parse_status = parse_line(scenario_def, &scenario);
    assert(parse_status==SUCCESS);

    Vector destination = clone_vector(&scenario.vector);
    display_vector(INFO, "Destination: ", &destination);

    Solution solution = new_solution(scenario.button_count);

    ExecutionStatus actual_status = solver(&scenario, &destination, &solution);

    if (actual_status == SUCCESS) {
        // Step 1: Assert that it actually arrives at the same place.
        Vector actual_destination = new_vector_with_dimensions(scenario.dimensions);
        actual_status = compile_vector_from_solution(&scenario, &solution, &actual_destination);
        assert(actual_status == SUCCESS);

        assert_vectors_eq(&actual_destination, &scenario.vector);
        display_vector(INFO, "Solution correctly arrived at ", &actual_destination);

        free_vector(&actual_destination);
    }

    bool success_status = actual_status == expected_status;
    log_to_stderr(
        DEBUG,
        "%s on \x1b[1m\"%s\"\x1b[22m got status \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
        solver_name,
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
#endif
