#include "line.h"
#include "bracket.h"

/*
 * @brief A function to parse a single `char*` object into its respective
 * components.
 */
ExecutionStatus parse_line(STRING input, Scenario* scenario) {
    STRING buffer = (STRING)malloc(sizeof(char) * MAX_LINE_LENGTH);
    USIZE buffer_size=0;
    USIZE input_length = strlen(input);

    bool has_indicators = false;
    bool has_vector = false;

    USIZE pointer = 0;
    ExecutionStatus status;
    char current_char;
    while (true) {
        current_char = input[pointer];

        if (buffer_size > 0 && (current_char == ' ' || current_char == '\0' || current_char == '\n' || current_char == EOF)) {
            // Manually terminate the string.
            buffer[buffer_size] = '\0';

            // Flush the buffer to whatever struct it should be.
            switch (buffer[0]) {
                case '[':
                    status = parse_indicators(buffer, &scenario->indicator);
                    has_indicators = true;
                    break;
                case '(':
                    if (!has_indicators) {
                        log_to_stderr(ERROR, "Parsing buttons before indicators, dimension unknown.");
                        free(buffer);
                        return PARSE_LINE_MISSING_COMPONENTS;
                    }
                    if (scenario->button_count+1 >= MAX_BUTTONS) {
                        return INSUFFICIENT_CAPACITY;
                    }
                    Button button = new_button();
                    status = parse_button(buffer, &button, scenario->indicator.dimensions);
                    if (status == SUCCESS) {
                        scenario->buttons[scenario->button_count++] = button;
                    }
                    break;
                case '{':
                    status = parse_vector(buffer, &scenario->vector);
                    has_vector = true;
                    break;
                default:
                    log_to_stderr(ERROR, "Unknown input segment found: \x1b[1m\"%s\"\x1b[22m.", buffer);
                    free(buffer);
                    return PARSE_INVALID_BRACKETS;
            }

            if (status != SUCCESS) {
                free(buffer);
                return status;
            }

            if (current_char != ' ') {
                log_to_stderr(TRACE, "Found end of line at character \x1b[1m%u\x1b[0m.", pointer);
                break;
            }
            // Reset the buffer size, and rewrite from there.
            buffer_size = 0;
        } else {
            buffer[buffer_size++] = current_char;
        }

        pointer++;
    }
    free(buffer);

    if (has_indicators && has_vector && scenario->button_count > 0) {
        if (scenario->indicator.dimensions != scenario->vector.dimensions) {
            log_to_stderr(
                ERROR,
                "Line \x1b[1m\"%s\"\x1b[22m has mismatched indicator (\x1b[1m%u\x1b[0m) and vector (\x1b[1m%u\x1b[0m) dimensions.",
                input,
                scenario->indicator.dimensions,
                scenario->vector.dimensions
            );
            return MISMATCHED_DIMENSIONS;
        }
        scenario->dimensions = scenario->indicator.dimensions;
        return SUCCESS;
    } else if (!has_indicators) {
        log_to_stderr(ERROR, "Line \x1b[1m\"%s\"\x1b[22m missing Indicators.", input);
    } else if (!has_vector) {
        log_to_stderr(ERROR, "Line \x1b[1m\"%s\"\x1b[22m missing Vector.", input);
    } else {
        log_to_stderr(ERROR, "Line \x1b[1m\"%s\"\x1b[22m missing Buttons.", input);
    }

    return PARSE_LINE_MISSING_COMPONENTS;
}


// ======================================================================================================
// UNIT TEST


#ifdef UNIT_TEST
void assert_parse_line(
    STRING input,
    ExecutionStatus expected_status,

    EFFECT expected_indicator,
    EFFECT expected_buttons_concatenated,
    TARGET expected_target,
    USIZE expected_button_count,
    USIZE expected_dimensions
) {
    Scenario scenario = new_scenario();

    ExecutionStatus actual_status = parse_line(input, &scenario);

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

    bool success_count = scenario.dimensions == expected_dimensions;
    log_to_stderr(
        DEBUG,
        "Parsed \x1b[1m\"%s\"\x1b[22m into a scenario of \x1b[%um\x1b[1m%u\x1b[0m dimension, expecting \x1b[1m%u\x1b[22m.",
        input,
        31+success_count,
        scenario.dimensions,
        expected_dimensions
    );
    assert(success_count);

    // Assert indicators matches.
    bool success_indicators;
    log_to_stderr(
        DEBUG,
        "Asserting indicators."
    );
    for (USIZE index=0; index < scenario.indicator.dimensions; index++) {
        success_indicators = scenario.indicator.effect[index] == expected_indicator[index];
        log_to_stderr(
            DEBUG,
            "At position \x1b[1m%u\x1b[22m, found \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
            index,
            31+success_indicators,
            scenario.indicator.effect[index],
            expected_indicator[index]
        );
        assert(success_indicators);
    }

    // Assert buttons.
    bool success_button_effect;
    // Use a bigger object here, since this is MAX_BUTTON * MAX_DIM, which USIZE does not guarantee
    unsigned long expected_buttons_pointer = 0;
    log_to_stderr(
        DEBUG,
        "Asserting buttons."
    );
    for (USIZE button_index=0; button_index < scenario.button_count; button_index++) {
        Button* button = &scenario.buttons[button_index];

        assert(button->dimensions == scenario.dimensions);

        for (USIZE effect_index=0; effect_index < button->dimensions; effect_index++) {
            success_button_effect = button->effect[effect_index] == expected_buttons_concatenated[expected_buttons_pointer];

            log_to_stderr(
                DEBUG,
                "At position \x1b[1m%u\x1b[22m of button \x1b[1m%u\x1b[22m, found \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
                effect_index,
                button_index,
                31+success_button_effect,
                button->effect[effect_index],
                expected_buttons_concatenated[expected_buttons_pointer]
            );
            assert(success_indicators);
            expected_buttons_pointer++;
        }
    }
    // Assert indicators matches.
    ExecutionStatus success_target;
    log_to_stderr(
        DEBUG,
        "Asserting vector."
    );
    for (USIZE index=0; index < scenario.vector.dimensions; index++) {
        success_target = scenario.vector.target[index] == expected_target[index];
        log_to_stderr(
            DEBUG,
            "At position \x1b[1m%u\x1b[22m, found \x1b[%um\x1b[1m%u\x1b[0m, expecting \x1b[1m%u\x1b[22m.",
            index,
            31+success_target,
            scenario.vector.target[index],
            expected_target[index]
        );
        assert(success_target);
    }

    free_scenario(&scenario);
}

void test_parse_line() {
    EFFECT_AMOUNT example_1_indicators[] = {0,1,1,0};
    EFFECT_AMOUNT example_1_buttons_concatenated[] = {0,0,0,1, 0,1,0,1, 0,0,1,0, 0,0,1,1, 1,0,1,0, 1,1,0,0};
    TARGET_AMOUNT example_1_target[] = {3,5,4,7};

    assert_parse_line(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        SUCCESS,
        example_1_indicators,
        example_1_buttons_concatenated,
        example_1_target,
        6,
        4
    );

    EFFECT_AMOUNT example_2_indicators[] = {0,0,0,0,0,0,0,0,1};
    EFFECT_AMOUNT example_2_buttons_concatenated[] = {0,0,0,1,0,1,0,0,0, 0,1,1,0,0,1,1,1,1, 1,1,0,0,0,1,0,0,0, 0,1,0,1,1,0,0,0,1, 0,1,1,1,1,1,0,0,1, 1,1,0,0,0,0,1,0,0, 1,0,1,1,1,1,0,0,1, 0,0,1,1,1,1,1,1,0, 0,1,1,1,1,1,0,0,0, 0,1,1,1,1,1,0,1,1, 1,1,1,0,1,1,1,1,1};
    TARGET_AMOUNT example_2_target[] = {65,110,97,89,92,122,60,55,80};

    assert_parse_line(
        "[........#] (3,5) (1,2,5,6,7,8) (0,1,5) (1,3,4,8) (1,2,3,4,5,8) (0,1,6) (0,2,3,4,5,8) (2,3,4,5,6,7) (1,2,3,4,5) (1,2,3,4,5,7,8) (0,1,2,4,5,6,7,8) {65,110,97,89,92,122,60,55,80}",
        SUCCESS,
        example_2_indicators,
        example_2_buttons_concatenated,
        example_2_target,
        11,
        9
    );
}
#endif
