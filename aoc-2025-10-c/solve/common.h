#ifndef SOLVE_COMMON_H
#define SOLVE_COMMON_H

#include <stdlib.h>
#include "../types/mod.h"
#include "../utils/log.h"

typedef struct {
    USIZE button_id;
    DISTANCE distance;
} ButtonComparator;

ExecutionStatus square_euclidean_length_of_vector_with_button_to_target(Vector* vector, Button* button, Vector* target, DISTANCE* distance);

// // Hiding this function as its not safe:
// // it does not know about the length of `distances`, and it can overflow.
// void sort_button_ids_by_provided_distances(
//     Order* order,
//     DISTANCE* distances
// );

ExecutionStatus rank_buttons_by_euclidean_distance(
    Scenario* scenario,
    Vector* current_position,
    Vector* destination,
    Order* order
);

#ifdef UNIT_TEST

#include "../parse/mod.h"
#include <string.h>

void assert_square_euclidean_length_of_vector_with_button_to_target(
    STRING vector_def,
    STRING button_def,
    STRING destination_def,
    ExecutionStatus expected_status,
    DISTANCE expected_distance
);
void assert_sort_button_ids_by_provided_distances(
    STRING order_def,
    STRING distance_def,
    USIZE* expected_order
);
void assert_rank_buttons_by_euclidean_distance(
    STRING scenario_def,
    STRING current_position_def,
    STRING destination_def,
    ExecutionStatus expected_status,
    USIZE expected_count,
    USIZE* expected_order
);
int test_euclidean_lengths();
int test_sort_button_ids();
int test_rank_buttons_by_euclidean_distance();

void assert_solver(
    STRING solver_name,
    ExecutionStatus (* solver)(Scenario*, Vector*, Solution*),
    STRING scenario_def,
    ExecutionStatus expected_status,
    STRING solution_def
);

#endif
#endif
