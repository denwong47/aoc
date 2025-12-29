#ifndef UTILS_STR_H
#define UTILS_STR_H

#include <string.h>
#include "../types/common.h"

void ltrim_one_mut(STRING string, char matches);
void rtrim_one_mut(STRING string, char matches);

#ifdef UNIT_TEST
#include "../utils/log.h"
void assert_trim(void (* callback)(STRING, char), STRING input, STRING expected, char matches);
void test_trim();
#endif

#endif
