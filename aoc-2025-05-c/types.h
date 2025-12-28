#ifndef TYPES_H
#define TYPES_H

#define MAX_RANGES 256
#define MAX_LINE_LENGTH 128
#define RANGE_DEFINITION char*
#define BOUNDS unsigned long
#define USIZE unsigned short
#define COUNT unsigned long

typedef struct {
    BOUNDS start;
    BOUNDS end;
} Range;

typedef struct {
    Range* data;
    USIZE count;
    USIZE capacity;
} Ranges;

typedef enum {
    SUCCESS = 0,
    PARSE_FAILURE_INVALID_RANGE = 1,
    PARSE_FAILURE_RANGES_FULL = 2,
    PARSE_FAILURE_EMPTY_LINE = 3,
    PARSE_FAILURE_NOT_A_NUMBER = 4,
    NOT_IN_RANGES = 16,
    RANGES_NOT_SORTED = 17,
    RANGES_NOT_OVERLAPPING = 32,
} ExecutionStatus;

#endif
