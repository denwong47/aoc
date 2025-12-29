#include "bracket.h"

/**
 * @brief Internal function to compare integers.
 */
int usize_comp(const void *a, const void *b) {
    return (*(USIZE *)a - *(USIZE *)b);
}

/*
 * @brief Internal function to parse a string of `[.##..#]` `0` and `1`s.
 */
ExecutionStatus parse_indicators(STRING input, Button* indicators) {
    USIZE string_length = strlen(input);
    if (string_length <= 1) {
        log_to_stderr(ERROR, "Insufficient string buffer of length \x1b[1m%u\x1b[22m supplied to `parse_indicators`.", string_length);
        return PARSE_EMPTY_BUFFER;
    } else if (input[0] != '[') {
        log_to_stderr(ERROR, "Indicator string is malformed, expected '[': \x1b[1m\"%s\"\x1b[22m", input);
        return PARSE_INVALID_BRACKETS;
    }

    for (USIZE index=0; index < string_length; index++) {
        switch (input[index]) {
            case '[':
                if (index != 0) {
                    log_to_stderr(ERROR, "Open square bracket found in invalid location: \x1b[1m\"%s\"\x1b[22m.", input);
                    return PARSE_INVALID_BRACKETS;
                }
                break;
            case ']':
                log_to_stderr(TRACE, "Found closing square bracket, concluding Indicators of \x1b[1m%u\x1b[22m dimensions.", indicators->dimensions);
                return SUCCESS;
            case EMPTY_CHAR:
            case FULL_CHAR:
                log_to_stderr(TRACE, "Found \x1b[1m'%c'\x1b[0m at position \x1b[1m%u\x1b[22m.", input[index], indicators->dimensions);
                if (indicators->dimensions >= indicators->capacity) {
                    log_to_stderr(ERROR, "Only supports upto \x1b[1m%u\x1b[22m dimensions, found at least \x1b[31m\x1b[1m%u\x1b[0m.", MAX_DIM, indicators->dimensions+1);
                    return PARSE_DIMENSIONS_OUT_OF_RANGE;
                }
                if (input[index] == EMPTY_CHAR) {
                    indicators->effect[indicators->dimensions++]=false;
                } else {
                    indicators->effect[indicators->dimensions++]=true;
                }
                break;
            case '\0':
                log_to_stderr(ERROR, "Unexpected end of string at position \x1b[1m%u\x1b[22m.", indicators->dimensions);
                return PARSE_INVALID_BRACKETS;
            default:
                log_to_stderr(ERROR, "Found invalid character of \x1b[1m'%c'\x1b[0m at position \x1b[1m%u\x1b[22m.", input[index], indicators->dimensions);
                return PARSE_INVALID_INDICATOR;
        }
    }

    log_to_stderr(ERROR, "Unexpected end of string at position \x1b[1m%u\x1b[22m.", indicators->dimensions);
    return PARSE_INVALID_BRACKETS;
}

/*
 * @brief Internal function to parse a string of `(2,3)` into a `Button` struct.
 *
 * Button inputs do not indicate the dimensions of itself, only the button's effect.
 * Hence `dimensions` need to be explicitly passed in.
 */
ExecutionStatus parse_button(STRING input, Button* button, USIZE dimensions) {
    unsigned long input_length = strlen(input);
    if (input_length <= 1) {
        log_to_stderr(ERROR, "Insufficient string buffer of length \x1b[1m%u\x1b[22m supplied to `parse_button`.", input_length);
        return PARSE_EMPTY_BUFFER;
    } else if (input[0] != '(') {
        log_to_stderr(ERROR, "Button string is malformed, expected '(': \x1b[1m\"%s\"\x1b[22m", input);
        return PARSE_INVALID_BRACKETS;
    } else if (input[input_length-1] != ')') {
        log_to_stderr(ERROR, "Button string is malformed, expected ')': \x1b[1m\"%s\"\x1b[22m", input);
        return PARSE_INVALID_BRACKETS;
    }

    ltrim_one_mut(input, '(');
    rtrim_one_mut(input, ')');

    NUMBER* numbers = (NUMBER*)malloc(MAX_DIM * sizeof(NUMBER));
    USIZE parsed_count;
    ExecutionStatus status = parse_numbers(input, numbers, &parsed_count);
    if (status!=SUCCESS) {
        free(numbers);
        return status;
    }

    // Sort the numbers to ensure ordering;
    qsort(numbers, parsed_count, sizeof(NUMBER), usize_comp);

    button->dimensions = dimensions;

    USIZE numbers_pointer = 0;
    for (USIZE index=0; index < button->dimensions; index++) {
        if (numbers_pointer < parsed_count && numbers[numbers_pointer] >= button->dimensions) {
            log_to_stderr(ERROR, "Found effect index \x1b[1m%u\x1b[22m, but max dimension is \x1b[1m%u\x1b[22m.", numbers[numbers_pointer], button->dimensions);
            return PARSE_DIMENSIONS_OUT_OF_RANGE;
        } else if (numbers_pointer < parsed_count && numbers[numbers_pointer] == index) {
            button->effect[index] = true;
            numbers_pointer++;
        } else {
            button->effect[index] = false;
        }
    }

    free(numbers);

    return SUCCESS;
}

/*
 * @brief Internal function to parse a string of `{3,1,20,6}` into a `Vector` struct.
 */
ExecutionStatus parse_vector(STRING input, Vector* vector) {
    unsigned long input_length = strlen(input);
    if (input_length <= 1) {
        log_to_stderr(ERROR, "Insufficient string buffer of length \x1b[1m%u\x1b[22m supplied to `parse_button`.", input_length);
        return PARSE_EMPTY_BUFFER;
    } else if (input[0] != '{') {
        log_to_stderr(ERROR, "Vector string is malformed, expected '(': \x1b[1m\"%s\"\x1b[22m", input);
        return PARSE_INVALID_BRACKETS;
    } else if (input[input_length-1] != '}') {
        log_to_stderr(ERROR, "Vector string is malformed, expected ')': \x1b[1m\"%s\"\x1b[22m", input);
        return PARSE_INVALID_BRACKETS;
    }

    ltrim_one_mut(input, '{');
    rtrim_one_mut(input, '}');

    // We can still afford to parse all of `MAX_DIM`, even if `vector->capacity` is lower,
    // so that we can report that to the user explicitly.
    NUMBER* numbers = (NUMBER*)malloc(MAX_DIM * sizeof(NUMBER));
    USIZE parsed_count;
    ExecutionStatus status = parse_numbers(input, numbers, &parsed_count);

    if (status!=SUCCESS) {
        free(numbers);
        return status;
    }

    // Overflow protection
    if (parsed_count > vector->capacity) {
        log_to_stderr(ERROR, "Provided Vector only has capacity of \x1b[1m%u\x1b[22m dimensions, found \x1b[31m\x1b[1m%u\x1b[22m.", vector->capacity, parsed_count);
        return PARSE_DIMENSIONS_OUT_OF_RANGE;
    }

    for (USIZE index=0; index < parsed_count; index++) {
        if (numbers[index] <= TARGET_AMOUNT_MAX) {
            vector->target[index] = (TARGET_AMOUNT)numbers[index];
        } else {
            log_to_stderr(
                ERROR,
                "At position \x1b[1m%u\x1b[22m, target value of \x1b[31m\x1b[1m%u\x1b[0m overflows the limit of \x1b[1m%u\x1b[22m.",
                index,
                numbers[index],
                TARGET_AMOUNT_MAX
            );
            free(numbers);
            return PARSE_TARGET_OVERFLOWS;
        }
    }
    vector->dimensions = parsed_count;

    return SUCCESS;
}


// ======================================================================================================
// UNIT TEST


#ifdef UNIT_TEST
void assert_parse_numbers(STRING input, ExecutionStatus expected_status, NUMBER* expected_array, USIZE expected_count) {
    char buffer[MAX_LINE_LENGTH];
    // We have to copy the static string into a mutable buffer due to `strtok`.
    strncpy(buffer, input, sizeof(buffer));

    NUMBER* actual_array = (NUMBER*)malloc(MAX_DIM * sizeof(NUMBER));
    USIZE actual_count;

    ExecutionStatus actual_status = parse_numbers((STRING)&buffer, actual_array, &actual_count);

    bool success_status = actual_status == expected_status;
    log_to_stderr(
        DEBUG,
        "Parsing \x1b[1m\"%s\"\x1b[22m, got status \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
        input,
        31+success_status,
        actual_status,
        expected_status
    );
    assert(success_status);

    if (expected_status != SUCCESS) {
        return;
    }

    bool success_count = actual_count == expected_count;
    log_to_stderr(
        DEBUG,
        "Parsed \x1b[1m\"%s\"\x1b[22m into \x1b[%um\x1b[1m%u\x1b[0m numbers, expecting \x1b[1m%u\x1b[22m.",
        input,
        31+success_count,
        actual_count,
        expected_count
    );
    assert(success_count);

    bool success_number;
    for (USIZE index=0; index < actual_count; index++) {
        success_number = actual_array[index] == expected_array[index];
        log_to_stderr(
            DEBUG,
            "At position \x1b[1m%u\x1b[22m, found \x1b[%um\x1b[1m%u\x1b[0m numbers, expecting \x1b[1m%u\x1b[22m.",
            index,
            31+success_number,
            actual_array[index],
            expected_array[index]
        );
        assert(success_number);
    }

    free(actual_array);
}

void assert_parse_indicators(STRING input, ExecutionStatus expected_status, EFFECT expected_effects, USIZE expected_dimensions) {
    char buffer[MAX_LINE_LENGTH];
    // We have to copy the static string into a mutable buffer due to `strtok`.
    strncpy(buffer, input, sizeof(buffer));

    Button indicators = new_button();

    ExecutionStatus actual_status = parse_indicators((STRING)&buffer, &indicators);

    bool success_status = actual_status == expected_status;
    log_to_stderr(
        DEBUG,
        "Parsing \x1b[1m\"%s\"\x1b[22m, got status \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
        input,
        31+success_status,
        actual_status,
        expected_status
    );
    assert(success_status);

    if (expected_status != SUCCESS) {
        return;
    }

    bool success_count = indicators.dimensions == expected_dimensions;
    log_to_stderr(
        DEBUG,
        "Parsed \x1b[1m\"%s\"\x1b[22m into \x1b[%um\x1b[1m%u\x1b[0m numbers, expecting \x1b[1m%u\x1b[22m.",
        input,
        31+success_count,
        indicators.dimensions,
        expected_dimensions
    );
    assert(success_count);

    bool success_effect;
    for (USIZE index=0; index < indicators.dimensions; index++) {
        success_effect = indicators.effect[index] == expected_effects[index];
        log_to_stderr(
            DEBUG,
            "At position \x1b[1m%u\x1b[22m, found \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
            index,
            31+success_effect,
            indicators.effect[index],
            expected_effects[index]
        );
        assert(success_effect);
    }

    free(indicators.effect);
    assert(indicators.capacity == MAX_DIM);
}

void assert_parse_button(STRING input, ExecutionStatus expected_status, EFFECT expected_effects, USIZE dimensions) {
    char buffer[MAX_LINE_LENGTH];
    // We have to copy the static string into a mutable buffer due to `strtok`.
    strncpy(buffer, input, sizeof(buffer));

    Button button = new_button();

    ExecutionStatus actual_status = parse_button((STRING)&buffer, &button, dimensions);

    bool success_status = actual_status == expected_status;
    log_to_stderr(
        DEBUG,
        "Parsing \x1b[1m\"%s\"\x1b[22m, got status \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
        input,
        31+success_status,
        actual_status,
        expected_status
    );
    assert(success_status);

    if (expected_status != SUCCESS) {
        return;
    }

    bool success_effect;
    for (USIZE index=0; index < button.dimensions; index++) {
        success_effect = button.effect[index] == expected_effects[index];
        log_to_stderr(
            DEBUG,
            "At position \x1b[1m%u\x1b[22m, found \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
            index,
            31+success_effect,
            button.effect[index],
            expected_effects[index]
        );
        assert(success_effect);
    }

    free(button.effect);
    assert(button.capacity == MAX_DIM);
}

void assert_parse_vector(STRING input, ExecutionStatus expected_status, TARGET expected_target, USIZE expected_dimensions) {
    char buffer[MAX_LINE_LENGTH];
    // We have to copy the static string into a mutable buffer due to `strtok`.
    strncpy(buffer, input, sizeof(buffer));

    Vector vector = new_vector();

    ExecutionStatus actual_status = parse_vector((STRING)&buffer, &vector);

    bool success_status = actual_status == expected_status;
    log_to_stderr(
        DEBUG,
        "Parsing \x1b[1m\"%s\"\x1b[22m, got status \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
        input,
        31+success_status,
        actual_status,
        expected_status
    );
    assert(success_status);

    if (expected_status != SUCCESS) {
        return;
    }

    bool success_count = vector.dimensions == expected_dimensions;
    log_to_stderr(
        DEBUG,
        "Parsed \x1b[1m\"%s\"\x1b[22m into \x1b[%um\x1b[1m%u\x1b[0m targets, expecting \x1b[1m%u\x1b[22m.",
        input,
        31+success_count,
        vector.dimensions,
        expected_dimensions
    );
    assert(success_count);

    bool success_target;
    for (USIZE index=0; index < vector.dimensions; index++) {
        success_target = vector.target[index] == expected_target[index];
        log_to_stderr(
            DEBUG,
            "At position \x1b[1m%u\x1b[22m, found \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
            index,
            31+success_target,
            vector.target[index],
            expected_target[index]
        );
        assert(success_target);
    }

    free(vector.target);
}

void test_parse_numbers() {
    NUMBER* invalid_array;

    NUMBER expected_0_valid[] = {};
    assert_parse_numbers("", SUCCESS, expected_0_valid, 0);

    NUMBER expected_1_valid[] = {300};
    assert_parse_numbers("300", SUCCESS, expected_1_valid, 1);

    NUMBER expected_5_valid[] = {2,3,4,5,6};
    assert_parse_numbers("2,3,4,5,6", SUCCESS, expected_5_valid, 5);

    NUMBER expected_10_valid[] = {1,2,3,4,5,6,7,8,9,0};
    assert_parse_numbers("1,2,3,4,5,6,7,8,9,0", SUCCESS, expected_10_valid, 10);

    assert_parse_numbers("1,2,3,4,5,6,7,8,9,10,11", PARSE_DIMENSIONS_OUT_OF_RANGE, invalid_array, 0);

    assert_parse_numbers("1,2,3,4,5,A,7", PARSE_INVALID_NUMBER, invalid_array, 0);

    NUMBER expected_overflow[] = {1,2,0};
    assert_parse_numbers("1,2,4294967296", SUCCESS, expected_overflow, 3);
}

void test_parse_indicators() {
    EFFECT_AMOUNT* invalid_array;

    EFFECT_AMOUNT expected_0_valid[] = {};
    assert_parse_indicators("[]", SUCCESS, expected_0_valid, 0);

    EFFECT_AMOUNT expected_1_valid[] = {1};
    assert_parse_indicators("[#]", SUCCESS, expected_1_valid, 1);

    EFFECT_AMOUNT expected_5_valid[] = {1,1,0,1,1};
    assert_parse_indicators("[##.##]", SUCCESS, expected_5_valid, 5);

    EFFECT_AMOUNT expected_10_valid[] = {0,1,1,0,1,1,1,0,0,0};
    assert_parse_indicators("[.##.###...]", SUCCESS, expected_10_valid, 10);

    assert_parse_indicators("", PARSE_EMPTY_BUFFER, invalid_array, 0);
    assert_parse_indicators("ABC", PARSE_INVALID_BRACKETS, invalid_array, 0);
    assert_parse_indicators("[.#.#[]", PARSE_INVALID_BRACKETS, invalid_array, 0);
    assert_parse_indicators("[.##.###...#]", PARSE_DIMENSIONS_OUT_OF_RANGE, invalid_array, 0);
    assert_parse_indicators("[.##.###...", PARSE_INVALID_BRACKETS, invalid_array, 0);
}

void test_parse_button() {
    EFFECT_AMOUNT* invalid_array;

    EFFECT_AMOUNT expected_0_valid[] = {0,0,0,0,0};
    assert_parse_button("()", SUCCESS, expected_0_valid, 5);

    EFFECT_AMOUNT expected_3_valid[] = {0,1,0,1,1};
    assert_parse_button("(1,3,4)", SUCCESS, expected_3_valid, 5);

    assert_parse_button("(1,3,5)", PARSE_DIMENSIONS_OUT_OF_RANGE, invalid_array, 5);

    EFFECT_AMOUNT expected_3_extended_valid[] = {0,1,0,1,0,1};
    assert_parse_button("(1,3,5)", SUCCESS, expected_3_extended_valid, 6);

    assert_parse_button("", PARSE_EMPTY_BUFFER, invalid_array, 0);
    assert_parse_button("ABC", PARSE_INVALID_BRACKETS, invalid_array, 0);
    assert_parse_button("[.#.#]", PARSE_INVALID_BRACKETS, invalid_array, 0);
    assert_parse_button("(1,2,3]", PARSE_INVALID_BRACKETS, invalid_array, 0);
    assert_parse_button("[1,2,3)", PARSE_INVALID_BRACKETS, invalid_array, 0);
    assert_parse_button("(1,4,A)", PARSE_INVALID_NUMBER, invalid_array, 0);
}

void test_parse_vector() {
    TARGET_AMOUNT* invalid_array;

    TARGET_AMOUNT expected_0_valid[] = {};
    assert_parse_vector("{}", SUCCESS, expected_0_valid, 0);

    TARGET_AMOUNT expected_5_valid[] = {7,5,12,7,2};
    assert_parse_vector("{7,5,12,7,2}", SUCCESS, expected_5_valid, 5);

    assert_parse_vector("{0,0,0,0,0,1,1,1,1,1,1}", PARSE_DIMENSIONS_OUT_OF_RANGE, invalid_array, 11);

    assert_parse_vector("", PARSE_EMPTY_BUFFER, invalid_array, 0);
    assert_parse_vector("ABC", PARSE_INVALID_BRACKETS, invalid_array, 0);
    assert_parse_vector("[.#.#]", PARSE_INVALID_BRACKETS, invalid_array, 0);
    assert_parse_vector("(1,2,3)", PARSE_INVALID_BRACKETS, invalid_array, 0);
    assert_parse_vector("{1,2,3)", PARSE_INVALID_BRACKETS, invalid_array, 0);
    assert_parse_vector("(1,2,3}", PARSE_INVALID_BRACKETS, invalid_array, 0);
    assert_parse_vector("{1,4,A}", PARSE_INVALID_NUMBER, invalid_array, 0);
    assert_parse_vector("{1,5,65536}", PARSE_TARGET_OVERFLOWS, invalid_array, 0);
}
#endif
