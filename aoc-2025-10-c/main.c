#include "./solve/mask.h"
#include "./solve/divide.h"
#include "./parse/line.h"
#include "types/solution.h"
#include "utils/log.h"


int main() {
    STRING buffer = (STRING)malloc(MAX_LINE_LENGTH * sizeof(char));
    size_t len=0;
    size_t read;

    ExecutionStatus status;

    USIZE total_presses_p1 = 0;
    USIZE total_presses_p2 = 0;
    while ((read=getline(&buffer, &len, stdin)) != -1) {
        Scenario scenario = new_scenario();
        printf("Parsing \x1b[1m%s\x1b[22m...\n", buffer);
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
        Solution solution_p1 = new_solution(scenario.button_count);
        Solution solution_p2 = new_solution(scenario.button_count);
        printf("Solving Part 1 for \x1b[1m%s\x1b[22m...\n", buffer);
        status = bfs_for_mask(&scenario, &scenario.indicator, &solution_p1, &destination);
        if (status!=SUCCESS) {
            return status;
        }
        for (USIZE index=0; index < solution_p1.button_count; index++) {
            total_presses_p1 += solution_p1.presses[index];
        }

        printf("Solving Part 2 for \x1b[1m%s\x1b[22m...\n", buffer);
        status = solve_by_division(&scenario, &scenario.vector, &solution_p2);
        if (status!=SUCCESS) {
            return status;
        }
        for (USIZE index=0; index < solution_p2.button_count; index++) {
            total_presses_p2 += solution_p2.presses[index];
        }

        len=0;

        free_solution(&solution_p1);
        free_vector(&destination);
        free_scenario(&scenario);
    }

    free(buffer);
    printf("Part 1: total number of button presses: \x1b[32m\x1b[1m%u\x1b[0m\n", total_presses_p1);
    printf("Part 2: total number of button presses: \x1b[32m\x1b[1m%u\x1b[0m\n", total_presses_p2);
}
