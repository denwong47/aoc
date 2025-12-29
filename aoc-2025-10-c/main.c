#include "./solve/mask.h"
#include "./parse/line.h"
#include "utils/log.h"


int main() {
    STRING buffer = (STRING)malloc(MAX_LINE_LENGTH * sizeof(char));
    size_t len=0;
    size_t read;

    ExecutionStatus status;

    USIZE total_presses = 0;
    while ((read=getline(&buffer, &len, stdin)) != -1) {
        Scenario scenario = new_scenario();
        log_to_stderr(INFO, "Found a line of length \x1b[1m%u\x1b[22m.", len);
        status = parse_line(buffer, &scenario);

        if (status!=SUCCESS) {
            return status;
        }

        log_to_stderr(
            INFO,
            "Parse a line with \x1b[1m%u\x1b[22m dimensions and \x1b[1m%u\x1b[22m buttons.",
            scenario.dimensions,
            scenario.button_count
        );

        Vector destination = new_vector_with_dimensions(scenario.dimensions);
        Solution solution = new_solution(scenario.button_count);
        status = bfs_for_mask(&scenario, &scenario.indicator, &solution, &destination);
        if (status!=SUCCESS) {
            return status;
        }

        for (USIZE index=0; index < solution.button_count; index++) {
            total_presses += solution.presses[index];
        }

        len=0;

        free_solution(&solution);
        free_vector(&destination);
        free_scenario(&scenario);
    }

    free(buffer);
    printf("Part 1: total number of button presses: \x1b[32m\x1b[1m%u\x1b[0m\n", total_presses);
}
