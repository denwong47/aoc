#ifndef TYPES_COMMON_H
#define TYPES_COMMON_H

#include "imports.h"
#define EFFECT_AMOUNT bool
#define TARGET_AMOUNT unsigned short
#define TARGET_AMOUNT_MAX USHRT_MAX
#define PRESS_AMOUNT unsigned short
#define EFFECT EFFECT_AMOUNT*
#define TARGET TARGET_AMOUNT*
#define PRESSES PRESS_AMOUNT*
#define DISTANCE unsigned long
#define MAX_NUMBERS 12
#define MAX_DIM 10
#define MAX_BUTTONS 12
#define MAX_PRESSES 384
#define MAX_LINE_LENGTH 1024
#define BALANCE_FACTOR 2
#define EMPTY_CHAR '.'
#define FULL_CHAR '#'

// This is used as a staging space for numbers before we know what type it is
#define NUMBER unsigned int

#define USIZE unsigned short
#define USIZE_MAX USHRT_MAX
#define STRING char*
#define ITER_COUNTER unsigned long long
#define ITER_LOG_INTERVAL 512

#endif
