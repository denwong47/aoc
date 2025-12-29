#ifndef TYPES_VECTOR_H
#define TYPES_VECTOR_H

#include "common.h"
#include "status.h"
#include "../utils/log.h"

typedef struct {
    EFFECT effect;
    USIZE dimensions;
    USIZE capacity;
} Button;

typedef struct {
    TARGET target;
    USIZE dimensions;
    USIZE capacity;
} Vector;

Button new_button();
void display_button(LogLevel level, STRING prefix, Button* button);
bool are_buttons_eq(Button* lhs, Button* rhs);
bool is_empty_button(Button* button);
Vector new_vector();
Vector new_vector_with_dimensions(USIZE dimensions);
void empty_vector(Vector* vector);
Vector clone_vector(Vector* vector);
void display_vector(LogLevel level, STRING prefix, Vector* vector);
Vector vector_from_button(Button* button);
ExecutionStatus is_vector_matching_mask(Vector* vector, Button* mask);
ExecutionStatus add_to_vector(Vector* vector, Button* button_press);
ExecutionStatus add_scalar_to_vector_upto(Vector* vector, USIZE amount, Vector* limit);
ExecutionStatus subtract_from_vector(Vector* vector, Button* button_press);
ExecutionStatus combine_vectors(Vector* lhs, Vector* rhs);
ExecutionStatus subtract_vectors(Vector* lhs, Vector* rhs);
ExecutionStatus divide_vector_by_scalar(Vector* vector, USIZE factor);
Button skim_vector_to_even(Vector* vector);
ExecutionStatus balance_vectors_by_amount(Vector* lhs, Vector* rhs, TARGET_AMOUNT amount);
DISTANCE square_euclidean_length_of_vector(Vector* vector);
bool is_empty_vector(Vector* vector);
bool are_vectors_eq(Vector* lhs, Vector* rhs);
int compare_vectors(Vector* lhs, Vector* rhs);
int compare_vectors_for_qsort(const void* lhs, const void* rhs);

void free_button(Button* button);
void free_vector(Vector* vector);

#ifdef UNIT_TEST
#include <string.h>
#include "../parse/bracket.h"
void assert_vectors_eq(
    Vector* lhs,
    Vector* rhs
);
void assert_vector_op(
    ExecutionStatus (* operation)(Vector*, Vector*),
    STRING lhs_definition,
    STRING rhs_definition,
    ExecutionStatus expected_status,
    STRING expected_definition
);

void test_vector_ops();

void assert_divide_vector_by_scalar(
    STRING vector_def,
    USIZE factor,
    ExecutionStatus expected_status,
    STRING expected_def
);

void test_divide_vector_by_scalar();

void assert_add_scalar_to_vector_upto(
    STRING vector_def,
    USIZE amount,
    STRING destination_def,
    ExecutionStatus expected_status,
    STRING expected_def
);

void test_add_scalar_to_vector_upto();

void assert_skim_vector_to_even(
    STRING vector_def,
    STRING skimmed_def,
    STRING expected_def
);

void test_skim_vector_to_even();

void assert_balance_vectors_by_amount(
    STRING lhs_def,
    STRING rhs_def,
    USIZE amount,
    ExecutionStatus expected_status,
    STRING expected_lhs_def,
    STRING expected_rhs_def
);

void test_balance_vectors_by_amount();
#endif
#endif
