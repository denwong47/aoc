#include "types.c"
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <stdbool.h>

USIZE pow10(USIZE n)
{
    return (USIZE) pow(10, n);
}

RANGE_TYPE create_mask(USIZE len, USIZE repeats) {
    RANGE_TYPE mask = 0;

    USIZE pattern_length = len / repeats;
    assert(pattern_length > 0);

    for (USIZE pos=0; pos < len; pos+=pattern_length) {
        mask += pow10(pos);
    }
    return mask;
}

USIZE base_10_length(RANGE_TYPE number) {
    return (USIZE) floor(log10(number)) +1;
}

bool is_invalid(RANGE_TYPE candidate, USIZE max_repeats) {
    USIZE max_length = base_10_length(candidate);
    if (max_repeats == 0) {
        max_repeats = max_length;
    }
    for (USIZE repeats=2; repeats <= max_length && repeats <= max_repeats; repeats++) {
        if (max_length % repeats != 0) {
            continue;
        }
        USIZE mask = create_mask(max_length, repeats);
        if (candidate % mask == 0) {
            #ifdef VERBOSE
            printf("\x1b[31m%lu\x1b[0m is divisible by %lu, and therefore invalid.\n", candidate, mask);
            #endif
            return true;
        }
    }
    #ifdef VERBOSE
    printf("\x1b[32m%lu\x1b[0m is a valid ID.\n", candidate);
    #endif

    return false;
}

RANGE_TYPE sum_invalids_in_range(RANGE_TYPE start, RANGE_TYPE end, USIZE max_repeats) {
    RANGE_TYPE total = 0;
    for (RANGE_TYPE candidate=start; candidate <= end; candidate++) {
        if (is_invalid(candidate, max_repeats)) {
            total += candidate;
        }
    }
    #ifdef VERBOSE
    printf("Between \x1b[1m%lu\x1b[0m-\x1b[1m%lu\x1b[0m, the total of all invalid numbers is \x1b[31m%lu\x1b[0m.\n", start, end, total);
    #endif
    return total;
}

#ifdef UNIT_TEST
int main() {
    assert(pow10(0) == 1);
    assert(pow10(1) == 10);
    assert(pow10(2) == 100);
    assert(pow10(6) == 1000000);
    assert(create_mask(6, 3) == 10101);
    assert(create_mask(6, 2) == 1001);
    assert(create_mask(6, 1) == 1);
    assert(create_mask(6, 6) == 111111);
    assert(is_invalid(1188511885, 0) == true);
    assert(is_invalid(1188511886, 0) == false);
    assert(is_invalid(22, 0) == true);
    assert(is_invalid(333, 0) == true);
    assert(is_invalid(343, 0) == false);
    assert(is_invalid(65656565, 0) == true);
    assert(is_invalid(12345678, 0) == false);
    assert(sum_invalids_in_range(998, 1012, 0) == 2009);
    assert(sum_invalids_in_range(1188511880, 1188511890, 0) == 1188511885);

    return 0;
}
#endif
