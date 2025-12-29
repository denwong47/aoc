#include "vector.h"
#include "common.h"
#include "status.h"

/*
 * @brief Create a new button with the default capacity.
 */
Button new_button() {
    Button button;
    button.dimensions = 0;
    button.capacity = MAX_DIM;
    button.effect = (EFFECT)malloc(button.capacity * sizeof(EFFECT_AMOUNT));

    return button;
}

/*
 * @brief Print the vector at the chosen log level with the supplied prefix.
 */
void display_button(LogLevel level, STRING prefix, Button* button) {
    if (should_log(level)) {
        log_to_stderr_with_sep_without_ln(level, "%s", prefix);
        write_to_stderr("\x1b[1m{");
        for (USIZE index=0; index<button->dimensions; index++) {
            write_to_stderr("%u", button->effect[index]);
            if (index+1<button->dimensions) {
                write_to_stderr(",");
            }
        }
        write_to_stderr("}\x1b[22m\n");
    }
}

/*
 * @brief Check if two `Button`s are equal, and short circuit if otherwise.
 */
bool are_buttons_eq(
    Button* lhs,
    Button* rhs
) {
    bool is_equal, success_target;
    for (USIZE index=0; index < lhs->dimensions; index++) {
        if (lhs->effect[index] != rhs->effect[index]) return false;
    }
    return true;
}

/*
 * @brief Returns `true` if a vector consists only of `0` values.
 */
bool is_empty_button(Button* button) {
    for (USIZE index=0; index < button->dimensions; index++) {
        if (button->effect[index] > 0) {
            return false;
        }
    }
    return true;
}

/*
 * @brief Create a new empty vector with predefined dimensions.
 */
Vector new_vector_with_dimensions(USIZE dimensions) {
    Vector vector;
    vector.dimensions = dimensions;
    vector.capacity = dimensions;
    vector.target = (TARGET)malloc(vector.capacity * sizeof(TARGET_AMOUNT));

    return vector;
}

/*
 * @brief Create a new vector with the default capacity.
 */
Vector new_vector() {
    return new_vector_with_dimensions(MAX_DIM);
}

/*
 * @brief Reset a vector to all `0`.
 */
void empty_vector(Vector* vector) {
    for (USIZE index=0; index < vector->dimensions; index++) {
        vector->target[index] = 0;
    }
}

/*
 * @brief Clone a vector in memory.
 */
Vector clone_vector(Vector* vector) {
    Vector cloned = new_vector_with_dimensions(vector->dimensions);
    empty_vector(&cloned);
    combine_vectors(&cloned, vector);
    return cloned;
}

/*
 * @brief Print the vector at the chosen log level with the supplied prefix.
 */
void display_vector(LogLevel level, STRING prefix, Vector* vector) {
    if (should_log(level)) {
        log_to_stderr_with_sep_without_ln(level, "%s", prefix);
        write_to_stderr("\x1b[1m{");
        for (USIZE index=0; index<vector->dimensions; index++) {
            write_to_stderr("%u", vector->target[index]);
            if (index+1<vector->dimensions) {
                write_to_stderr(",");
            }
        }
        write_to_stderr("}\x1b[22m\n");
    }
}

/*
 * @brief Create a new vector from a `Button`.
 */
Vector vector_from_button(Button* button) {
    Vector vector = new_vector_with_dimensions(button->dimensions);
    for (USIZE index=0; index < vector.dimensions; index++) {
        vector.target[index] = (TARGET_AMOUNT)button->effect[index];
    }
    return vector;
}

/*
 * @brief Check if a vector matches a mask.
 */
ExecutionStatus is_vector_matching_mask(Vector* vector, Button* mask) {
    if (vector->dimensions != mask->dimensions) {
        return MISMATCHED_DIMENSIONS;
    }

    for (USIZE index=0; index<vector->dimensions; index++) {
        if (vector->target[index] % 2 != mask->effect[index]) {
            return NO_SOLUTION;
        }
    }

    return SUCCESS;
}

/*
 * @brief Add a button press to the vector.
 */
ExecutionStatus add_to_vector(Vector* vector, Button* button_press) {
    if (vector->dimensions != button_press->dimensions) {
        log_to_stderr(
            ERROR,
            "Vector and pressed button have differing dimensions: \x1b[1m%u\x1b[0m and \x1b[1m%u\x1b[0m.",
            vector->dimensions,
            button_press->dimensions
        );
        return MISMATCHED_DIMENSIONS;
    }

    for (USIZE index=0; index < vector->dimensions; index++) {
        vector->target[index] += button_press->effect[index];
    }

    return SUCCESS;
}

/*
 * @brief Add a uniform scalar amount to each dimension of the vector, up to the destination.
 *
 * This is a specific function to solve `[.##.]` by repeatedly adding `2` to the
 * whole vector, then try solving it.
 *
 * @deprecated Turns out we may not need this function; this may be removed.
 */
ExecutionStatus add_scalar_to_vector_upto(Vector* vector, TARGET_AMOUNT amount, Vector* limit) {
    if (limit != NULL && vector->dimensions != limit->dimensions) {
        log_to_stderr(
            ERROR,
            "Vector and limit have differing dimensions: \x1b[1m%u\x1b[0m and \x1b[1m%u\x1b[0m.",
            vector->dimensions,
            limit->dimensions
        );
        return MISMATCHED_DIMENSIONS;
    }

    if (limit != NULL) {
        // Limit mode - check upper bounds before adding.
        // Step 1: Check if any values overflowed
        USIZE* new_values = (USIZE*)malloc(vector->dimensions * sizeof(USIZE));
        for (USIZE index=0; index < vector->dimensions; index++) {
            new_values[index] = vector->target[index] + amount;
            if (new_values[index] > limit->target[index]) {
                log_to_stderr(
                    WARN,
                    "At position \x1b[1m%u\x1b[22m, \x1b[31m\x1b[1m%u\x1b[22m + \x1b[1m%u\x1b[0m > \x1b[1m%u\x1b[22m, aborting scalar addition.",
                    index,
                    vector->target[index],
                    amount,
                    limit->target[index]
                );
                free(new_values);
                return VECTOR_OVERFLOW;
            }
        }

        // Step 2: Commit to the changes
        for (USIZE index=0; index < vector->dimensions; index++)
            vector->target[index] = new_values[index];

        free(new_values);
    } else {
        // No limit mode.

        for (USIZE index=0; index < vector->dimensions; index++) {
            vector->target[index] += amount;
        }
    }

    return SUCCESS;
}

/*
 * @brief Remove a button press from the vector.
 *
 * There is underflow protection, but it does not guarantee that this button was
 * indeed pressed to achieve this `Vector` in the first place.
 */
ExecutionStatus subtract_from_vector(Vector* vector, Button* button_press) {
    if (vector->dimensions != button_press->dimensions) {
        log_to_stderr(
            ERROR,
            "Vector and pressed button have differing dimensions: \x1b[1m%u\x1b[0m and \x1b[1m%u\x1b[0m.",
            vector->dimensions,
            button_press->dimensions
        );
        return MISMATCHED_DIMENSIONS;
    }
    for (USIZE index=0; index < vector->dimensions; index++) {
        if (button_press->effect[index] > vector->target[index]) {
            log_to_stderr(
                ERROR,
                "Vector underflowed at position \x1b[1m%u\x1b[22m: \x1b[31m\x1b[1m%u\x1b[0m - \x1b[1m%u\x1b[22m < 0.",
                index,
                vector->target[index],
                button_press->effect[index]
            );
            return VECTOR_UNDERFLOW;
        }
        vector->target[index] -= button_press->effect[index];
    }
    return SUCCESS;
}

/*
 * @brief Combine the second vector into the first one.
 */
ExecutionStatus combine_vectors(Vector* lhs, Vector* rhs) {
    if (lhs->dimensions != rhs->dimensions) {
        log_to_stderr(
            ERROR,
            "Vectors have differing dimensions: \x1b[1m%u\x1b[0m and \x1b[1m%u\x1b[0m.",
            lhs->dimensions,
            rhs->dimensions
        );
        return MISMATCHED_DIMENSIONS;
    }

    for (USIZE index=0; index < lhs->dimensions; index++) {
        lhs->target[index] += rhs->target[index];
    }

    return SUCCESS;
}

/*
 * @brief Combine the second vector into the first one.
 */
ExecutionStatus subtract_vectors(Vector* lhs, Vector* rhs) {
    if (lhs->dimensions != rhs->dimensions) {
        log_to_stderr(
            ERROR,
            "Vectors have differing dimensions: \x1b[1m%u\x1b[0m and \x1b[1m%u\x1b[0m.",
            lhs->dimensions,
            rhs->dimensions
        );
        return MISMATCHED_DIMENSIONS;
    }

    // Step 1: Check that the subtraction is fully valid
    for (USIZE index=0; index < lhs->dimensions; index++) {
        if (rhs->target[index] > lhs->target[index]) {
            log_to_stderr(
                ERROR,
                "Vector underflowed at position \x1b[1m%u\x1b[22m: \x1b[31m\x1b[1m%u\x1b[0m - \x1b[1m%u\x1b[22m < 0.",
                index,
                lhs->target[index],
                rhs->target[index]
            );
            return VECTOR_UNDERFLOW;
        }
    }

    // Step 2: Commit to the subtraction
    for (USIZE index=0; index < lhs->dimensions; index++) {
        lhs->target[index] -= rhs->target[index];
    }
    return SUCCESS;
}

/*
 * @brief Integer divide all values in this `Vector` by a factor.
 *
 * If the existing value is indivisible by `factor`, return `INDIVISIBLE_VALUE`,
 * keeping the vector unchanged.
 */
ExecutionStatus divide_vector_by_scalar(Vector* vector, USIZE factor) {
    // Step 1: Check divisibility
    for (USIZE index=0; index < vector->dimensions; index++) {
        if (vector->target[index] % factor != 0) {
            log_to_stderr(
                ERROR,
                "Could not divide \x1b[31m\x1b[1m%u\x1b[0m by \x1b[1m%u\x1b[0m.",
                vector->target[index],
                factor
            );
            return INDIVISIBLE_VALUE;
        }
    }

    // Step 2: Perform division
    for (USIZE index=0; index < vector->dimensions; index++) {
        vector->target[index] /= factor;
    }

    return SUCCESS;
}

/*
 * @brief Create a new `Vector` by skimming `0` or `1` from an existing `Vector`
 * to make all values even.
 *
 * Contrary to a previous version of this function, this will no longer mutates
 * the `vector`.
 */
Button skim_vector_to_even(Vector* vector) {
    Button skimmed = new_button();
    skimmed.dimensions = vector->dimensions;

    TARGET_AMOUNT skimmed_amount;
    for (USIZE index=0; index < vector->dimensions; index++) {
        // BALANCE_FACTOR MUST BE 2 due to `skimmed` being a `Button` here
        skimmed_amount = vector->target[index] % BALANCE_FACTOR;
        // vector->target[index] -= skimmed_amount;
        skimmed.effect[index] = skimmed_amount;
    }

    return skimmed;
}

/*
 * @brief For each feature, attempt to move `X` from `lhs` to `rhs`.
 *
 * It will not perform any move if the feature has less than `X` left.
 * If nothing was moved, return `BALANCING_IMPOSSIBLE`.
 */
ExecutionStatus balance_vectors_by_amount(
    Vector* lhs,
    Vector* rhs,
    TARGET_AMOUNT amount
) {
    if (lhs->dimensions != rhs->dimensions) {
        log_to_stderr(
            ERROR,
            "LHS and RHS have differing dimensions: \x1b[1m%u\x1b[0m and \x1b[1m%u\x1b[0m.",
            lhs->dimensions,
            rhs->dimensions
        );
        return MISMATCHED_DIMENSIONS;
    }

    TARGET_AMOUNT moved=0;
    for (USIZE index=0; index<lhs->dimensions; index++) {
        if (lhs->target[index] >= amount) {
            moved += amount;
            lhs->target[index] -= amount;
            rhs->target[index] += amount;
        }
    }

    if (moved <= 0) {
        return BALANCING_IMPOSSIBLE;
    }
    return SUCCESS;
}

/*
 * @brief The squared euclidean distance of the vector.
 *
 * This is the sum of the squared lengths of each dimension - basically euclidean distance
 * without performing the final square root.
 *
 * Typically used for heuristics or comparison.
 */
DISTANCE square_euclidean_length_of_vector(Vector* vector) {
    DISTANCE distance=0;
    for (USIZE index=0; index < vector->dimensions; index++) {
        distance += vector->target[index]*vector->target[index];
    }

    return distance;
}

/*
 * @brief Returns `true` if a vector consists only of `0` values.
 */
bool is_empty_vector(Vector* vector) {
    for (USIZE index=0; index < vector->dimensions; index++) {
        if (vector->target[index] > 0) {
            return false;
        }
    }
    return true;
}

/*
 * @brief Check if two `Vector`s are equal, and short circuit if otherwise.
 */
bool are_vectors_eq(
    Vector* lhs,
    Vector* rhs
) {
    bool is_equal, success_target;
    for (USIZE index=0; index < lhs->dimensions; index++) {
        if (lhs->target[index] != rhs->target[index]) return false;
    }
    return true;
}

/*
 * @brief Compare two vectors, by their euclidean distances.
 */
int compare_vectors(Vector* lhs, Vector* rhs) {
    DISTANCE lhs_dis = square_euclidean_length_of_vector(lhs);
    DISTANCE rhs_dis = square_euclidean_length_of_vector(rhs);

    if (lhs_dis > rhs_dis) return 1;
    if (lhs_dis < rhs_dis) return -1;
    return 0;
}

/*
 * @brief Compare two vectors, by their euclidean distances.
 *
 * The signature of this function is meant to match the requirements
 * of `qsort`, which forces casting.
 */
int compare_vectors_for_qsort(const void* lhs, const void* rhs) {
    Vector* lhs_casted = (Vector*)lhs;
    Vector* rhs_casted = (Vector*)rhs;
    return compare_vectors(lhs_casted, rhs_casted);
}

/*
 * @brief Free a `Vector` from memory.
 */
void free_button(Button* button) {
    free(button->effect);
}

/*
 * @brief Free a `Vector` from memory.
 */
void free_vector(Vector* vector) {
    free(vector->target);
}

// ======================================================================================================
// UNIT TEST


#ifdef UNIT_TEST

void assert_vectors_eq(
    Vector* lhs,
    Vector* rhs
) {
    bool success, success_target;
    success=true;
    for (USIZE index=0; index < lhs->dimensions; index++) {
        success_target = lhs->target[index] == rhs->target[index];
        log_to_stderr(
            DEBUG,
            "At position \x1b[1m%u\x1b[22m, found \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
            index,
            31+success_target,
            lhs->target[index],
            rhs->target[index]
        );

        if (!success_target) {
            success = false;
        }
    }
    assert(success);
}

void assert_vector_op(
    ExecutionStatus (* operation)(Vector*, Vector*),
    STRING lhs_definition,
    STRING rhs_definition,
    ExecutionStatus expected_status,
    STRING expected_definition
) {
    ExecutionStatus parse_status;
    char buffer[MAX_LINE_LENGTH];
    // We have to copy the static string into a mutable buffer due to `strtok`.
    Vector lhs = new_vector_with_dimensions(5);
    Vector rhs = new_vector();
    Vector expected = new_vector();

    strncpy(buffer, lhs_definition, sizeof(buffer));
    parse_status = parse_vector((STRING)&buffer, &lhs);
    assert(parse_status==SUCCESS);

    strncpy(buffer, rhs_definition, sizeof(buffer));
    parse_status = parse_vector((STRING)&buffer, &rhs);
    assert(parse_status==SUCCESS);

    // Actual tests

    ExecutionStatus actual_status = operation(&lhs, &rhs);

    bool success_status = actual_status == expected_status;
    log_to_stderr(
        DEBUG,
        "Operating on \x1b[1m\"%s\"\x1b[22m and \x1b[1m\"%s\"\x1b[22m, got status \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
        lhs_definition,
        rhs_definition,
        31+success_status,
        actual_status,
        expected_status
    );
    assert(success_status);

    if (expected_status != SUCCESS) {
        return;
    }

    strncpy(buffer, expected_definition, sizeof(buffer));
    parse_status = parse_vector((STRING)&buffer, &expected);
    assert(parse_status==SUCCESS);

    assert_vectors_eq(&lhs, &expected);

    free(lhs.target);
    free(rhs.target);
    free(expected.target);

}

void test_vector_ops() {
    assert_vector_op(
        combine_vectors,
        "{1,2,3,4,5}",
        "{0,1,4,9,16}",
        SUCCESS,
        "{1,3,7,13,21}"
    );
    assert_vector_op(
        combine_vectors,
        "{1,2,3,4,5}",
        "{0,1,4,9}",
        MISMATCHED_DIMENSIONS,
        ""
    );
    assert_vector_op(
        subtract_vectors,
        "{1,2,3,4,5}",
        "{0,1,3,2,3}",
        SUCCESS,
        "{1,1,0,2,2}"
    );
    assert_vector_op(
        subtract_vectors,
        "{1,2,3,4,5}",
        "{0,1,3,2}",
        MISMATCHED_DIMENSIONS,
        ""
    );
    assert_vector_op(
        subtract_vectors,
        "{1,2,3,4,5}",
        "{0,1,3,4,6}",
        VECTOR_UNDERFLOW,
        ""
    );
    assert_vector_op(
        subtract_vectors,
        "{1,2,3,4,5}",
        "{1,2,3,4,5}",
        SUCCESS,
        "{0,0,0,0,0}"
    );
}

void assert_divide_vector_by_scalar(
    STRING vector_def,
    USIZE factor,
    ExecutionStatus expected_status,
    STRING expected_def
) {
    char buffer[MAX_LINE_LENGTH];
    // We have to copy the static string into a mutable buffer due to `strtok`.
    Vector vector = new_vector();

    // Step 1: Parse the vector
    strncpy(buffer, vector_def, sizeof(buffer));
    ExecutionStatus parse_status = parse_vector((STRING)&buffer, &vector);
    assert(parse_status==SUCCESS);

    // Step 2: Perform division
    ExecutionStatus actual_status = divide_vector_by_scalar(&vector, factor);

    // Step 3: Assert the status
    bool success_status = actual_status == expected_status;
    log_to_stderr(
        DEBUG,
        "Dividing \x1b[1m%s\x1b[22m by \x1b[1m%u\x1b[22m, got status \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
        vector_def,
        factor,
        31+success_status,
        actual_status,
        expected_status
    );
    assert(success_status);

    if (expected_status == SUCCESS) {
        Vector expected = new_vector();
        strncpy(buffer, expected_def, sizeof(buffer));
        parse_status = parse_vector((STRING)&buffer, &expected);
        assert(parse_status==SUCCESS);

        // Step 4: Check each element.
        assert_vectors_eq(
            &vector,
            &expected
        );

        free_vector(&expected);
    }
    free_vector(&vector);
}

void test_divide_vector_by_scalar(){
    assert_divide_vector_by_scalar(
        "{8,2,6,4,10}",
        2,
        SUCCESS,
        "{4,1,3,2,5}"
    );

    assert_divide_vector_by_scalar(
        "{3,6,3,9,18}",
        3,
        SUCCESS,
        "{1,2,1,3,6}"
    );

    assert_divide_vector_by_scalar(
        "{0,0,0,0}",
        3,
        SUCCESS,
        "{0,0,0,0}"
    );

    assert_divide_vector_by_scalar(
        "{4,6,8,7,2}",
        2,
        INDIVISIBLE_VALUE,
        ""
    );
}

void assert_add_scalar_to_vector_upto(
    STRING vector_def,
    USIZE amount,
    STRING destination_def,
    ExecutionStatus expected_status,
    STRING expected_def
) {
    ExecutionStatus parse_status;
    char buffer[MAX_LINE_LENGTH];
    // We have to copy the static string into a mutable buffer due to `strtok`.
    Vector vector = new_vector();
    Vector destination = new_vector();

    Vector* destination_ref;

    // Step 1: Parse the vectors
    strncpy(buffer, vector_def, sizeof(buffer));
    parse_status = parse_vector((STRING)&buffer, &vector);
    assert(parse_status==SUCCESS);

    if (destination_def != NULL) {
        strncpy(buffer, destination_def, sizeof(buffer));
        parse_status = parse_vector((STRING)&buffer, &destination);
        assert(parse_status==SUCCESS);

        destination_ref = &destination;
    } else {
        destination_ref = NULL;
    }

    // Step 2: Perform the addition
    ExecutionStatus actual_status = add_scalar_to_vector_upto(&vector, amount, destination_ref);
    free(destination.target);

    // Step 3: Assert the status
    bool success_status = actual_status == expected_status;
    log_to_stderr(
        DEBUG,
        "Adding \x1b[1m%u\x1b[22m to \x1b[1m%s\x1b[22m, got status \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
        amount,
        vector_def,
        31+success_status,
        actual_status,
        expected_status
    );
    assert(success_status);

    if (expected_status != SUCCESS) {
        free(vector.target);
        return;
    }

    // Step 4: If there are things to assert, then parse the expected vector
    Vector expected = new_vector();
    strncpy(buffer, expected_def, sizeof(buffer));
    parse_status = parse_vector((STRING)&buffer, &expected);
    assert(parse_status==SUCCESS);

    // Step 5: Check each element.
    assert_vectors_eq(
        &vector,
        &expected
    );

    free(vector.target);
    free(expected.target);
}

void test_add_scalar_to_vector_upto() {
    assert_add_scalar_to_vector_upto(
        "{1,2,3,4,5}",
        2,
        NULL,
        SUCCESS,
        "{3,4,5,6,7}"
    );

    assert_add_scalar_to_vector_upto(
        "{1,2,3,4,5}",
        2,
        "{4,5,6,7,8}",
        SUCCESS,
        "{3,4,5,6,7}"
    );

    assert_add_scalar_to_vector_upto(
        "{1,2,3,4,5}",
        2,
        "{3,4,5,6,7}",
        SUCCESS,
        "{3,4,5,6,7}"
    );

    assert_add_scalar_to_vector_upto(
        "{1,2,3,4,5}",
        2,
        "{3,4,5,5,7}",
        VECTOR_OVERFLOW,
        NULL
    );
}

void assert_skim_vector_to_even(
    STRING vector_def,
    STRING skimmed_def,
    STRING expected_def
) {
    ExecutionStatus parse_status;
    char buffer[MAX_LINE_LENGTH];
    // We have to copy the static string into a mutable buffer due to `strtok`.

    // Step 1: Parse the vectors
    Vector vector = new_vector();
    strncpy(buffer, vector_def, sizeof(buffer));
    parse_status = parse_vector((STRING)&buffer, &vector);
    assert(parse_status==SUCCESS);

    Vector skimmed = new_vector();
    strncpy(buffer, skimmed_def, sizeof(buffer));
    parse_status = parse_vector((STRING)&buffer, &skimmed);
    assert(parse_status==SUCCESS);

    Vector expected = new_vector();
    strncpy(buffer, expected_def, sizeof(buffer));
    parse_status = parse_vector((STRING)&buffer, &expected);
    assert(parse_status==SUCCESS);

    // Step 2: Perform the skim
    Button actual = skim_vector_to_even(&vector);
    Vector actual_vector = vector_from_button(&actual);

    // Step 3: Check each element.
    log_to_stderr(
        DEBUG,
        "Checking actual output against \x1b[1m%s\x1b[22m...",
        skimmed_def
    );
    assert_vectors_eq(
        &actual_vector,
        &skimmed
    );

    // // Deprecated: We are no longer mutating the existing vectors.
    // log_to_stderr(
    //     DEBUG,
    //     "Checking \x1b[1m%s\x1b[22m against \x1b[1m%s\x1b[22m...",
    //     vector_def,
    //     expected_def
    // );
    // assert_vectors_eq(
    //     &vector,
    //     &expected
    // );

    free(actual.effect);
    free_vector(&vector);
    free_vector(&skimmed);
    free_vector(&expected);
}

void test_skim_vector_to_even() {
    assert_skim_vector_to_even(
        "{0,1,2,3,4,5,6,7,8,9}",
        "{0,1,0,1,0,1,0,1,0,1}",
        "{0,0,2,2,4,4,6,6,8,8}"
    );

    assert_skim_vector_to_even(
        "{4,6,12,8,12,0,2}",
        "{0,0,0,0,0,0,0}",
        "{4,6,12,8,12,0,2}"
    );

    assert_skim_vector_to_even(
        "{7,1,3,13,5,9}",
        "{1,1,1,1,1,1}",
        "{6,0,2,12,4,8}"
    );
}

void assert_balance_vectors_by_amount(
    STRING lhs_def,
    STRING rhs_def,
    USIZE amount,
    ExecutionStatus expected_status,
    STRING expected_lhs_def,
    STRING expected_rhs_def
) {
    ExecutionStatus parse_status;
    char buffer[MAX_LINE_LENGTH];
    // We have to copy the static string into a mutable buffer due to `strtok`.
    Vector lhs = new_vector();
    Vector rhs = new_vector();

    // Step 1: Parse the vectors
    strncpy(buffer, lhs_def, sizeof(buffer));
    parse_status = parse_vector((STRING)&buffer, &lhs);
    assert(parse_status==SUCCESS);

    strncpy(buffer, rhs_def, sizeof(buffer));
    parse_status = parse_vector((STRING)&buffer, &rhs);
    assert(parse_status==SUCCESS);

    // Step 2: Perform the addition
    ExecutionStatus actual_status = balance_vectors_by_amount(&lhs, &rhs, amount);

    // Step 3: Assert the status
    bool success_status = actual_status == expected_status;
    log_to_stderr(
        DEBUG,
        "Balaning \x1b[1m%u\x1b[22m from \x1b[1m%s\x1b[22m to \x1b[1m%s\x1b[22m, got status \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
        amount,
        lhs_def,
        rhs_def,
        31+success_status,
        actual_status,
        expected_status
    );
    assert(success_status);

    if (expected_status == SUCCESS) {
        // Step 4: If there are things to assert, then parse the expected vector
        Vector expected_lhs = new_vector();
        strncpy(buffer, expected_lhs_def, sizeof(buffer));
        parse_status = parse_vector((STRING)&buffer, &expected_lhs);
        assert(parse_status==SUCCESS);

        Vector expected_rhs = new_vector();
        strncpy(buffer, expected_rhs_def, sizeof(buffer));
        parse_status = parse_vector((STRING)&buffer, &expected_rhs);
        assert(parse_status==SUCCESS);

        // Step 5: Check each element.
        log_to_stderr(
            DEBUG,
            "Asserting LHS is as expected."
        );
        assert_vectors_eq(
            &lhs,
            &expected_lhs
        );
        log_to_stderr(
            DEBUG,
            "Asserting RHS is as expected."
        );
        assert_vectors_eq(
            &rhs,
            &expected_rhs
        );

        free(expected_lhs.target);
        free(expected_rhs.target);
    }
    free(lhs.target);
    free(rhs.target);
}

void test_balance_vectors_by_amount() {
    assert_balance_vectors_by_amount(
        "{3,4,5,6,7}",
        "{4,3,2,1,0}",
        2,
        SUCCESS,
        "{1,2,3,4,5}",
        "{6,5,4,3,2}"
    );

    assert_balance_vectors_by_amount(
        "{2,0,4,3,12}",
        "{1,2,3,4,5}",
        2,
        SUCCESS,
        "{0,0,2,1,10}",
        // Position 1 did not add.
        "{3,2,5,6,7}"
    );

    assert_balance_vectors_by_amount(
        "{2,0,4,3,12}",
        "{1,2,3,4,5}",
        13,
        BALANCING_IMPOSSIBLE,
        "",
        ""
    );
}
#endif
