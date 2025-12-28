#ifdef UNIT_TEST

#include "range.h"
#include "consolidate.h"

int test_ranges() {
    // Test parsing
    Ranges ranges = new_ranges();
    char range_input[] = "1-4,2-7,30-60,42-69,13-420";
    BOUNDS expected_parsing[] = {1,4,2,7,30,60,42,69,13,420};
    ExecutionStatus status = add_ranges_from_text(range_input, &ranges);
    // ExecutionStatus status = add_ranges_from_stdin(&ranges);
    assert(status == SUCCESS);
    Range* range;
    for (USIZE index=0; index < ranges.count; index++) {
        range = &ranges.data[index];
        assert(range->start == expected_parsing[index*2]);
        assert(range->end == expected_parsing[index*2+1]);
    }

    return 0;
}

int test_sorting() {
    // Test Sorting
    assert_compare("123-125", "122-125", 1);
    assert_compare("123-125", "123-125", 0);
    assert_compare("123-125", "124-125", -1);
    assert_compare("123-125", "123-126", -1);
    assert_compare("123-125", "123-124", 1);

    BOUNDS expected_sort_1[] = {1,2,3,4,5,6};
    assert_sort_ranges("1-2,3-4,5-6", expected_sort_1);
    assert_sort_ranges("5-6,3-4,1-2", expected_sort_1);
    assert_sort_ranges("3-4,1-2,5-6", expected_sort_1);

    BOUNDS expected_sort_2[] = {1,2,1,3,2,3,2,3,2,4};
    assert_sort_ranges("1-2,1-3,2-3,2-3,2-4", expected_sort_2);
    assert_sort_ranges("2-4,1-3,2-3,1-2,2-3", expected_sort_2);
    assert_sort_ranges("2-3,2-3,2-4,1-3,1-2", expected_sort_2);

    return 0;
}

int test_within() {
    // Test within
    assert_in_ranges("3-5,10-14,16-20,12-18", 6, RANGES_NOT_SORTED);
    assert_in_ranges("3-5,10-14,12-18,16-20", 3, SUCCESS);
    assert_in_ranges("3-5,10-14,12-18,16-20", 4, SUCCESS);
    assert_in_ranges("3-5,10-14,12-18,16-20", 5, SUCCESS);
    assert_in_ranges("3-5,10-14,12-18,16-20", 2, NOT_IN_RANGES);
    assert_in_ranges("3-5,10-14,12-18,16-20", 6, NOT_IN_RANGES);
    assert_in_ranges("3-5,10-14,12-18,16-20", 10, SUCCESS);
    assert_in_ranges("3-5,10-14,12-18,16-20", 12, SUCCESS);
    assert_in_ranges("3-5,10-14,12-18,16-20", 18, SUCCESS);
    assert_in_ranges("3-5,10-14,12-18,16-20", 20, SUCCESS);
    assert_in_ranges("3-5,10-14,12-18,16-20", 21, NOT_IN_RANGES);

    return 0;
}

int test_consolidate() {
    assert_combine_ranges("1-3", "2-4", SUCCESS, 1, 4);
    assert_combine_ranges("1-3", "3-4", SUCCESS, 1, 4);
    assert_combine_ranges("1-3", "4-5", RANGES_NOT_OVERLAPPING, 1, 3);
    assert_combine_ranges("1-3", "1-2", RANGES_NOT_SORTED, 1, 3);
    assert_combine_ranges("1-4", "2-3", SUCCESS, 1, 4);
    assert_combine_ranges("1-8", "8-10", SUCCESS, 1, 10);
    assert_combine_ranges("1-8", "9-10", RANGES_NOT_OVERLAPPING, 1, 8);

    BOUNDS example_ranges[] = {3,5,10,20};
    assert_consolidate_ranges("3-5,10-14,16-20,12-18", example_ranges);

    return 0;
}

int main() {
    test_ranges();
    test_sorting();
    test_within();
    test_consolidate();
}

#endif
