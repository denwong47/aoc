#include "log.h"

/*
 * @brief Returns the log level name.
 */
char* log_level_name(LogLevel level) {
    char* text;

    switch (level) {
        #define AS_CASE(name) case name: return #name;
        LOG_LEVELS(AS_CASE)
        #undef AS_CASE
        default: return "UNKNOWN";
    }
}

/*
 * @brief Return whether we should be logging at the chosen level.
 */
bool should_log(LogLevel level) {
    if (ERROR_ONLY && level != ERROR && level != CRITICAL) {
        // Nothing to log, its not verbose mode.
        return false;
    } else if (level == TRACE) {
        #ifndef VERBOSE_TRACE
        // If we didn't specify `VERBOSE_TRACE` level logging, don't bother outputting traces.
        return false;
        #endif
    }
    return true;
}

/*
 * @brief Write a static string to `stderr`.
 *
 * This provides a centralised place in one module where all `stderr` writing
 * will be defined.
 */
void write_to_stderr(char *format, ...) {
    va_list args;
    va_start(args, format); // Gather variadic arguments AFTER `format`
    vfprintf(stderr, format, args);
    va_end(args);
}

/*
 * @brief Variadic version of `log_to_stderr_with_sep`.
 *
 * Provides a wrapper for `log_to_stderr`.
 */
void variadic_log_to_stderr_with_sep(LogLevel level, const char *format, va_list args, char* sep) {
    if (!should_log(level)) {
        return;
    }
    const char *level_str = log_level_name(level);
    fprintf(stderr, "\x1b[38;5;%um%s\x1b[39m: ", level, level_str);

    vfprintf(stderr, format, args);

    // Add a newline for convenience
    if (sep != NULL) {
        fprintf(stderr, "%s", sep);
    }
}


/*
 * @brief Log a statement using the given level without the terminating new line.
 */
void log_to_stderr_with_sep_without_ln(LogLevel level, const char *format, ...) {
    va_list args;
    va_start(args, format); // Gather variadic arguments AFTER `format`

    variadic_log_to_stderr_with_sep(level, format, args, NULL);
    va_end(args);
}

/*
 * @brief Log a statement using the given level.
 */
void log_to_stderr(LogLevel level, const char *format, ...) {
    va_list args;
    va_start(args, format); // Gather variadic arguments AFTER `format`

    variadic_log_to_stderr_with_sep(level, format, args, "\n");
    va_end(args);
}

#ifdef UNIT_TEST
/*
 * @brief Testing the log function.
 *
 * These tests do not actually assert anything; they always pass unless
 * they do not compile, or there are any sort of Segfault. This is mostly
 * for visual inspection.
 */
void test_log() {
    LogLevel levels[] = {
        TRACE,
        DEBUG,
        INFO,
        WARN,
        ERROR,
        CRITICAL
    };

    for (unsigned short index=0; index<(sizeof(levels)/sizeof(LogLevel)); index++) {
        log_to_stderr(levels[index], "The title block should have been printed in \x1b[1mANSI Colour %u\x1b[0m.", levels[index]);
    }
}
#endif
