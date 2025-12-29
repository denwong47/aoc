#include "scenario.h"

/*
 * @brief Initialize a new `Scenario` object with all placeholder values.
 */
Scenario new_scenario() {
    Scenario scenario;

    scenario.indicator = new_button();
    scenario.buttons = (Button*)malloc(MAX_BUTTONS * sizeof(Button));
    scenario.vector = new_vector();
    scenario.button_count = 0;
    scenario.button_capacity = MAX_BUTTONS;
    scenario.dimensions = 0;

    return scenario;
}

/*
 * @brief Free all allocated heap space from a `Scenario` object.
 *
 * This will cause any further access to this object to trigger a segmentation fault.
 */
void free_scenario(Scenario *scenario) {
    free(scenario->indicator.effect);
    for (USIZE index=0; index < scenario->button_count; index++) {
        free(scenario->buttons[index].effect);
    }
    free(scenario->buttons);
    free(scenario->vector.target);
}
