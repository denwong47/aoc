#ifdef UNIT_TEST

#include "./utils/log.h"
#include "./utils/str.h"
#include "./parse/bracket.h"
#include "./parse/line.h"
#include "./types/combinations.h"
#include "./types/vector.h"
#include "./solve/common.h"
#include "./solve/brute.h"
#include "./solve/bisect.h"
#include "./solve/mask.h"

int main() {
    test_log();
    test_trim();

    test_parse_numbers();
    test_parse_indicators();
    test_parse_button();
    test_parse_vector();

    test_parse_line();

    test_vector_ops();

    test_euclidean_lengths();
    test_sort_button_ids();
    test_rank_buttons_by_euclidean_distance();
    test_divide_vector_by_scalar();
    test_add_scalar_to_vector_upto();
    test_balance_vectors_by_amount();
    test_skim_vector_to_even();

    test_combinations();
    test_bfs_for_mask();
    // test_brute();
    test_bisection();

    return 0;
}

#endif
