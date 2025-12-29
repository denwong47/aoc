#include "combinations.h"

/*
 * @brief Build a new `CombinationIterator` that yields current combinations of
 * buttons from the given `Scenario`.
 */
CombinationIterator iter_button_combinations(
    Scenario* scenario,
    USIZE size
) {
    if (scenario->button_count < size) {
        size = scenario->button_count;
    }

    CombinationIterator iterator;
    iterator.button_count = scenario->button_count;
    iterator.size = size;
    iterator.indices = NULL;

    return iterator;
}

/*
 * @brief Write `0,1,2,3,...,N-1` into `iterator`.
 */
void init_iterator(CombinationIterator* iterator) {
    if (iterator->indices != NULL) {
        log_to_stderr(
            WARN,
            "Initializing on an already initialized `CombinationIterator`; there's no guarantee the `malloc` is correct."
        );
    } else {
        iterator->indices = (USIZE*)malloc(iterator->size * sizeof(USIZE));
    }

    for (USIZE index=0; index<iterator->size; index++) {
        // Start with 0,1,2,3,...,N-1
        iterator->indices[index] = index;
    }
}

/*
 * @brief Yield the next button combination.
 *
 * If there are no more combination available, return `NULL`.
 */
USIZE* next_button_combination(CombinationIterator* iterator) {
    if (iterator->indices == NULL) {
        init_iterator(iterator);
    } else {
        // Step 1: Reverse search the first column that is not maxed out yet.
        bool found=false;
        USIZE index_with_room, rindex, max_button_index_for_rindex;

        // Assume `c` is `iterator->button_count`, then
        // r of each position shall be:
        // r[i] = n-i-1
        // i.e. n-1,n-2,...,1,0
        // then the each column is topped out at:
        // m[i] = c-r-1 = c-n+i
        // i.e. c-n+1,...,c-2,c-1
        for (USIZE index=0; index<iterator->size; index++) {
            rindex = iterator->size-index-1;

            max_button_index_for_rindex = iterator->button_count-index-1;
            if (iterator->indices[rindex] < max_button_index_for_rindex) {
                log_to_stderr(
                    TRACE,
                    "At position \x1b[1m%u\x1b[22m, we have value \x1b[1m%u\x1b[22m < \x1b[1m%u\x1b[22m.",
                    rindex,
                    iterator->indices[rindex],
                    max_button_index_for_rindex
                );
                found=true;
                index_with_room = rindex;
                break;
            } else {
                log_to_stderr(
                    TRACE,
                    "At position \x1b[1m%u\x1b[22m, we have value \x1b[1m%u\x1b[22m >= \x1b[1m%u\x1b[22m.",
                    rindex,
                    iterator->indices[rindex],
                    max_button_index_for_rindex
                );
            }
        }

        // Step 2: If there is a column which is not full, we then add one to that column,
        // then take that value and add 1 to all subsequent columns.
        if (found) {
            // At this point, assuming `r=n-3`, then our `indices` should look like:
            // a,...,b,c-2,c-1
            // What we should do next, is:
            // a,...,b+1,b+2,b+3
            // Since we know from above that `b<c-3`, therefore `b+3<c`, so we could not be
            // out of bounds.
            USIZE button_index = iterator->indices[rindex]+1;
            for (USIZE index=rindex; index<iterator->size; index++) {
                iterator->indices[index]=button_index++;
            }
        } else {
            // If nothing was found, iterator is exhausted.
            log_to_stderr(TRACE, "`CombinationIterator` is exhausted, freeing heap allocation.");
            // We even tidy up after ourselves, aren't we nice?
            free_iterator(iterator);
        }
    }

    return iterator->indices;
}

/*
 * @brief Free the `CombinationIterator` from memory.
 */
void free_iterator(CombinationIterator* iterator) {
    free(iterator->indices);
    iterator->indices = NULL;
}

// ======================================================================================================
// UNIT TEST


#ifdef UNIT_TEST

void assert_combinations(STRING scenario_def, USIZE size, USIZE* expected, USIZE expected_items) {
    log_to_stderr(
        TRACE,
        "Testing combinations for \x1b[1m\"%s\"\x1b[22m...",
        scenario_def
    );
    Scenario scenario = new_scenario();
    ExecutionStatus parse_status = parse_line(scenario_def, &scenario);
    assert(parse_status==SUCCESS);

    USIZE* indices;
    CombinationIterator iterator = iter_button_combinations(&scenario, size);
    bool success_index;
    for (USIZE index=0; index<expected_items; index++) {
        indices = next_button_combination(&iterator);

        assert(indices!=NULL);
        for (USIZE subindex=0; subindex<size; subindex++) {
            success_index = indices[subindex] == expected[index*size+subindex];
            log_to_stderr(
                TRACE,
                "At position \x1b[1m%u\x1b[22m of the item #\x1b[1m%u\x1b[22m, we got Button #\x1b[%um\x1b[1m%u\x1b[0m, expecting #\x1b[1m%u\x1b[22m",
                subindex,
                index,
                31+success_index,
                indices[subindex],
                expected[index*size+subindex]
            );
            assert(success_index);
        }
    }
    log_to_stderr(
        TRACE,
        "We have exhausted the expected combinations, so we expect the iterator to be exhausted too. Probing it one last time..."
    );
    indices = next_button_combination(&iterator);
    assert(indices==NULL);

    free_scenario(&scenario);
}

void test_combinations() {
    USIZE expected_4_by_3[] = {
        0, 1, 2, 0, 1, 3, 0, 2, 3, 1, 2, 3
    };
    assert_combinations(
        "[...] (0,1) (1,2) (2) (0,2) {0,0,0}",
        3,
        (USIZE*)expected_4_by_3,
        4
    );

    USIZE expected_4_by_4[] = {
        0, 1, 2, 3
    };
    assert_combinations(
        "[...] (0,1) (1,2) (2) (0,2) {0,0,0}",
        4,
        (USIZE*)expected_4_by_4,
        1
    );

    USIZE expected_8_by_2[] = {
        0,1,0,2,0,3,0,4,0,5,0,6,0,7,1,2,1,3,1,4,1,5,1,6,1,7,2,3,2,4,2,5,2,6,2,7,3,4,3,5,3,6,3,7,4,5,4,6,4,7,5,6,5,7,6,7
    };
    assert_combinations(
        "[...] (0,1) (1,2) (2) (0,2) (0,1) (1,2) (2) (0,2) {0,0,0}",
        2,
        (USIZE*)expected_8_by_2,
        28
    );

}
#endif
