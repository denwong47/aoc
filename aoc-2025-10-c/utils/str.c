#include "str.h"

/*
 * @brief Remove one character from the string if it matches the given
 * character.
 *
 * This is performed by moving the `char` in memory, and thus can be expensive.
 *
 * This assumes the string has at least a single `\0` in it;
 * its behaviour is undefined if the capacity of `string` is 0.
 */
void ltrim_one_mut(STRING string, char matches) {
    unsigned long length = strlen(string);
    if (length > 0 && string[0] == matches) {
        memmove(string, string+1, length);
    }
}

/*
 * @brief Remove one character from the end of the string if it matches the
 * given character.
 *
 * This is performed by changing the character into a `\0`; and is therefore
 * destructive.
 */
void rtrim_one_mut(STRING string, char matches) {
    unsigned long length = strlen(string);
    if (length > 0 && string[length-1] == matches) {
        string[length-1] = '\0';
    }
}

#ifdef UNIT_TEST
void assert_trim(void (* callback)(STRING, char), STRING input, STRING expected, char matches) {
    unsigned long input_length = strlen(input);
    STRING buffer = (STRING)malloc(input_length*sizeof(char));
    strncpy(buffer, input, input_length);
    buffer[input_length] = '\0';

    callback(buffer, matches);

    bool success = strcmp(buffer, expected) == 0;
    log_to_stderr(
        DEBUG,
        "Trimmed \x1b[1m\"%s\"\x1b[22m to \x1b[%um\x1b[1m\"%s\"\x1b[0m, expected \x1b[1m\"%s\"\x1b[22m.",
        input,
        31+success,
        buffer,
        expected
    );

    free(buffer);

    assert(success);
}

void test_trim() {
    assert_trim(ltrim_one_mut, "12345", "2345", '1');
    assert_trim(ltrim_one_mut, "12345", "12345", '2');
    assert_trim(ltrim_one_mut, "1", "", '1');
    assert_trim(ltrim_one_mut, "", "", '1');
    assert_trim(ltrim_one_mut, "\ntext", "text", '\n');
    assert_trim(rtrim_one_mut, "12345", "1234", '5');
    assert_trim(rtrim_one_mut, "12345", "12345", '4');
    assert_trim(rtrim_one_mut, "1", "", '1');
    assert_trim(rtrim_one_mut, "", "", '1');
    assert_trim(rtrim_one_mut, "text\n", "text", '\n');
}
#endif
