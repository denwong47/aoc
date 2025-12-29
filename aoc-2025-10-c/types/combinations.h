#ifndef TYPES_COMBINATIONS_H
#define TYPES_COMBINATIONS_H

#include "common.h"
#include "scenario.h"
#include "../utils/log.h"

typedef struct {
    USIZE* indices;
    USIZE button_count;
    USIZE size;
} CombinationIterator;

CombinationIterator iter_button_combinations(
    Scenario* scenario,
    USIZE size
);
void init_iterator(CombinationIterator* iterator);
USIZE* next_button_combination(CombinationIterator* iterator);
void free_iterator(CombinationIterator* iterator);

#ifdef UNIT_TEST
#include "scenario.h"
#include "../parse/line.h"
void test_combinations();
#endif

#endif
