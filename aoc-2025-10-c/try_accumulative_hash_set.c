#include <stdio.h>
#include <assert.h>
#include "libc_accumulative_hash_set.h"

// Replicate the Rust unit tests in C
int main() {
    struct CAccumulativeHashSetU64 *set = new_hash_set();
    
    printf("Initial hash set state:                    %lu\n", hash_set_state(set));

    assert(!hash_set_contains_path_to(set, 10));
    assert(hash_set_transverse_to(set, 10));
    printf("Hash set state after transverse to 10:     %lu\n", hash_set_state(set));

    // 10 -> 10 does not exist
    assert(!hash_set_contains_path_to(set, 10));
    assert(hash_set_transverse_to(set, 16));
    printf("Hash set state after transverse to 16:     %lu\n", hash_set_state(set));
    assert(hash_set_visit_and_backtrack(set));
    assert(hash_set_contains_path_to(set, 16));
    printf("Hash set state after backtracking from 16: %lu\n", hash_set_state(set));

    // It should refuse to transverse to 16 again
    {
        unsigned long current_state = hash_set_state(set);
        unsigned int current_visited = hash_set_visited_count(set);
        assert(!hash_set_transverse_to(set, 16));
        assert(hash_set_state(set) == current_state);
        assert(hash_set_visited_count(set) == current_visited);
    }

    // After we backtrack, the state should be now 0
    assert(hash_set_visit_and_backtrack(set));
    printf("Hash set state after backtracking from 10: %lu\n", hash_set_state(set));

    assert(hash_set_contains_path_to(set, 10));
    assert(hash_set_visited_count(set) == 2);
    assert(hash_set_state(set) == 0);

    // Now we go to 16 instead, but `16 -> 10` should already exist
    // because order does not matter
    assert(hash_set_transverse_to(set, 16));
    assert(hash_set_contains_path_to(set, 10));
    printf("Hash set state after transverse to 16:     %lu\n", hash_set_state(set));
    // It should refuse to transverse to 10 again
    {
        unsigned long current_state = hash_set_state(set);
        unsigned int current_visited = hash_set_visited_count(set);
        assert(!hash_set_transverse_to(set, 10));
        assert(hash_set_state(set) == current_state);
        assert(hash_set_visited_count(set) == current_visited);
    }

    // But it should allow us to go to 20
    assert(hash_set_transverse_to(set, 20));
    printf("Hash set state after transverse to 20:     %lu\n", hash_set_state(set));
    assert(hash_set_visit_and_backtrack(set));
    printf("Hash set state after backtracking from 20: %lu\n", hash_set_state(set));

    assert(hash_set_visited_count(set) == 3);
    
    free_hash_set(set);
    return 0;
}
