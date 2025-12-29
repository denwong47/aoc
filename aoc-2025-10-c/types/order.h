#ifndef TYPES_ORDER_H
#define TYPES_ORDER_H

#include "common.h"
#include "vector.h"

/*
 * @brief A small struct to keep track of `USIZE`s in a specific order.
 */
typedef struct {
    USIZE* ids;
    USIZE count;
    USIZE capacity;
} Order;

Order new_order(USIZE capacity);
void free_order(Order* order);

#endif
