#ifndef SOLVE_DIVIDE_H
#define SOLVE_DIVIDE_H

#include <math.h>
#include <time.h>
#include "../types/mod.h"
#include "brute.h"

#define MIN_QUOTIENT 5
#define MAX_VALUE_IN_VECTOR 15

Vector chunk_vector(Vector* vector, USIZE chunks);

#ifdef UNIT_TEST
#include "common.h"
#include "../parse/bracket.h"

void assert_chunk_vector(
    STRING vector_def,
    USIZE chunks,
    STRING manipulated_vector_def,
    STRING remainder_vector_def
);

void test_chunk_vector();

void assert_find_optimal_chunks(
    STRING vector_def,
    USIZE expected_chunks
);

void test_find_optimal_chunks();
void test_division();

#endif

#endif
