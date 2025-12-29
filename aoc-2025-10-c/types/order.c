#include "order.h"

/*
 * @brief Function to create a new `Order` struct with the given capacity.
 */
Order new_order(USIZE capacity) {
    Order order;

    order.ids = (USIZE*)malloc(capacity*sizeof(USIZE));
    order.count = 0;
    order.capacity = capacity;

    return order;
}

/*
 * @brief Free an `Order` struct from memory.
 */
void free_order(Order* order) {
    free(order->ids);
}
