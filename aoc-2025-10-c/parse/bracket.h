#ifndef PARSE_BRACKET_H
#define PARSE_BRACKET_H

#include <string.h>
#include "../types/vector.h"
#include "../utils/log.h"
#include "../utils/str.h"

int usize_comp(const void *a, const void *b);

ExecutionStatus parse_numbers(STRING input, NUMBER* array, USIZE* count);
ExecutionStatus parse_indicators(STRING input, Button* indicators);
ExecutionStatus parse_button(STRING input, Button* button, USIZE dimensions);
ExecutionStatus parse_vector(STRING input, Vector* vector);

#ifdef UNIT_TEST
void assert_parse_numbers(STRING input, ExecutionStatus expected_status, NUMBER* expected_array, USIZE expected_count);
void assert_parse_indicators(STRING input, ExecutionStatus expected_status, EFFECT expected_effects, USIZE expected_dimensions);
void assert_parse_button(STRING input, ExecutionStatus expected_status, EFFECT expected_effects, USIZE dimensions);
void assert_parse_vector(STRING input, ExecutionStatus expected_status, TARGET expected_target, USIZE expected_dimensions);

void test_parse_numbers();
void test_parse_indicators();
void test_parse_button();
void test_parse_vector();
#endif

#endif
