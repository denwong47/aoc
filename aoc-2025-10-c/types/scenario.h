#ifndef TYPES_SCENARIO_H
#define TYPES_SCENARIO_H

#include "common.h"
#include "vector.h"

typedef struct {
    Button indicator;
    Button* buttons;
    Vector vector;
    USIZE button_count;
    USIZE button_capacity;
    USIZE dimensions;
} Scenario;

Scenario new_scenario();
void free_scenario(Scenario *scenario);

#endif
