#ifndef UTILS_NUMBERS_H
#define UTILS_NUMBERS_H

#include <string.h>
#include "../types/common.h"
#include "../types/status.h"
#include "log.h"

ExecutionStatus parse_numbers(STRING input, NUMBER* array, USIZE* count);

#endif
