#ifndef UTILS_LOG_H
#define UTILS_LOG_H

#include <stdbool.h>
#include <stdio.h>
#include <stdarg.h>

#define LOG_LEVELS(X) \
    X(TRACE) \
    X(DEBUG) \
    X(INFO) \
    X(WARN) \
    X(ERROR) \
    X(CRITICAL)

#define TRACE_COLOUR 8
#define DEBUG_COLOUR 6
#define INFO_COLOUR 4
#define WARN_COLOUR 11
#define ERROR_COLOUR 9
#define CRITICAL_COLOUR 1

typedef enum {
    #define AS_ENUM(name) name = name ## _COLOUR,
    LOG_LEVELS(AS_ENUM)
    #undef AS_ENUM
} LogLevel;

char* log_level_name(LogLevel level);
bool should_log(LogLevel level);
void write_to_stderr(char *format, ...);
void variadic_log_to_stderr_with_sep(LogLevel level, const char *format, va_list args, char* sep);
void log_to_stderr_with_sep_without_ln(LogLevel level, const char *format, ...);
void log_to_stderr(LogLevel level, const char *format, ...);

#ifdef VERBOSE
#define ERROR_ONLY false
#else
#define ERROR_ONLY true
#endif

#ifdef UNIT_TEST
void test_log();
#endif

#endif
