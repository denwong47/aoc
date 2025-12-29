#ifndef PARSE_LINE_H
#define PARSE_LINE_H

#include <string.h>
#include "../types/mod.h"
#include "../utils/log.h"
#include "bracket.h"

ExecutionStatus parse_line(STRING input, Scenario* scenario);

#ifdef UNIT_TEST
void assert_parse_line(
    STRING input,
    ExecutionStatus expected_status,

    EFFECT expected_indicator,
    EFFECT expected_buttons_concatenated,
    TARGET expected_target,
    USIZE expected_button_count,
    USIZE expected_dimensions
);
void test_parse_line();
#endif

#endif
