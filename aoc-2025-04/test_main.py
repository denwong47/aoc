import numpy as np
import time as timer
import numpy.typing as npt
import pytest
import itertools
from pathlib import Path

from enum import IntEnum

Warehouse = npt.NDArray[np.int8]
Counter = npt.NDArray[np.uint8]

Slice = slice  # We will only use slices with step=None


class CellState(IntEnum):
    EMPTY = 0
    STOCK = 1
    REMOVED = -1


def get_input() -> str:
    input_path = Path(__file__).parent / "input.txt"
    return input_path.read_text()


def format_slice(s: Slice) -> str:
    return f"{s.start if s.start is not None else ''}:{s.stop if s.stop is not None else ''}"


def bounded(value: int, min_value: int, max_value: int) -> int:
    return max(min_value, min(value, max_value))


def text_to_warehouse(text: str) -> Warehouse:
    lines = text.strip().splitlines()
    height = len(lines)
    width = len(lines[0])
    warehouse = np.zeros((height, width), dtype=np.int8)
    for y, line in enumerate(lines):
        line = line.strip()
        for x, char in enumerate(line):
            if char == "@":
                warehouse[y, x] = CellState.STOCK
            elif char == ".":
                warehouse[y, x] = CellState.EMPTY
            elif char == "x":
                warehouse[y, x] = CellState.REMOVED
            else:
                raise ValueError(f"Unexpected character in warehouse map: {char}")
    return warehouse


def warehouse_to_text(warehouse: Warehouse) -> str:
    lines = []
    height, width = warehouse.shape
    for y in range(height):
        line_chars = []
        for x in range(width):
            cell = warehouse[y, x]
            if cell == CellState.STOCK:
                line_chars.append("@")
            elif cell == CellState.EMPTY:
                line_chars.append(".")
            elif cell == CellState.REMOVED:
                line_chars.append("x")
            else:
                raise ValueError(f"Unexpected cell state in warehouse: {cell}")
        lines.append("".join(line_chars))
    return "\n".join(lines)


def shifted_index(
    arr: npt.NDArray,
    *,
    x: int = 0,
    y: int = 0,
) -> tuple[Slice, Slice]:
    height, width = arr.shape

    x_start = 0 if x < 0 else x
    x_end = width if x > 0 else width + x

    y_start = 0 if y < 0 else y
    y_end = height if y > 0 else height + y

    return (slice(y_start, y_end), slice(x_start, x_end))


def offset_index(
    index: tuple[Slice, Slice],
    *,
    x: int = 0,
    y: int = 0,
) -> tuple[Slice, Slice]:
    y_slice, x_slice = index

    return (
        slice(
            y_slice.start + y,
            y_slice.stop + y,
        ),
        slice(
            x_slice.start + x,
            x_slice.stop + x,
        ),
    )


def add_stock_to_counter(
    warehouse: Warehouse,
    counter: Counter,
    *,
    x: int = 0,
    y: int = 0,
):
    counter_index = shifted_index(warehouse, x=x, y=y)
    # Counter index is shifted by (x, y), any may have been truncated,
    # so it may be smaller; we should get a new index for warehouse
    # by offsetting back by (-x, -y)
    warehouse_index = offset_index(counter_index, x=-x, y=-y)
    counter[counter_index] += warehouse[warehouse_index] == CellState.STOCK


def count_adjacent_stocks(
    warehouse: Warehouse,
) -> Counter:
    height, width = warehouse.shape
    counter: Counter = np.zeros((height, width), dtype=np.uint8)

    for x, y in itertools.product((-1, 0, 1), repeat=2):
        if x == 0 and y == 0:
            continue
        add_stock_to_counter(warehouse, counter, x=x, y=y)

    return counter


def remove_stocks(
    warehouse: Warehouse,
    counter: Counter,
    *,
    threshold: int = 4,
) -> int:
    to_remove = (warehouse == CellState.STOCK) & (counter < threshold)
    warehouse[to_remove] = CellState.REMOVED
    return np.sum(to_remove)


@pytest.mark.parametrize(
    ("input_warehouse", "input_counter", "x", "y", "expected_counter"),
    [
        pytest.param(
            np.array(
                [
                    [True, False, True, False],
                    [False, True, False, False],
                    [True, True, False, True],
                ],
                dtype=np.bool_,
            ),
            np.array(
                [
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                dtype=np.uint8,
            ),
            1,
            0,
            np.array(
                [
                    [0, 1, 0, 1],
                    [0, 0, 1, 0],
                    [0, 1, 1, 0],
                ],
                dtype=np.uint8,
            ),
            id="shift x=1",
        ),
        pytest.param(
            np.array(
                [
                    [True, False, True, False],
                    [False, True, False, False],
                    [True, True, False, True],
                ],
                dtype=np.bool_,
            ),
            np.array(
                [
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                dtype=np.uint8,
            ),
            0,
            1,
            np.array(
                [
                    [0, 0, 0, 0],
                    [1, 0, 1, 0],
                    [0, 1, 0, 0],
                ],
                dtype=np.uint8,
            ),
            id="shift y=1",
        ),
        pytest.param(
            np.array(
                [
                    [True, False, True, False],
                    [False, True, False, False],
                    [True, True, False, True],
                ],
                dtype=np.bool_,
            ),
            np.array(
                [
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                dtype=np.uint8,
            ),
            -1,
            0,
            np.array(
                [
                    [0, 1, 0, 0],
                    [1, 0, 0, 0],
                    [1, 0, 1, 0],
                ],
                dtype=np.uint8,
            ),
            id="shift x=-1",
        ),
        pytest.param(
            np.array(
                [
                    [True, False, True, False],
                    [False, True, False, False],
                    [True, True, False, True],
                ],
                dtype=np.bool_,
            ),
            np.array(
                [
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                dtype=np.uint8,
            ),
            0,
            -1,
            np.array(
                [
                    [0, 1, 0, 0],
                    [1, 1, 0, 1],
                    [0, 0, 0, 0],
                ],
                dtype=np.uint8,
            ),
            id="shift y=-1",
        ),
        pytest.param(
            np.array(
                [
                    [True, False, True, False],
                    [False, True, False, False],
                    [True, True, False, True],
                ],
                dtype=np.bool_,
            ),
            np.array(
                [
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                dtype=np.uint8,
            ),
            1,
            1,
            np.array(
                [
                    [0, 0, 0, 0],
                    [0, 1, 0, 1],
                    [0, 0, 1, 0],
                ],
                dtype=np.uint8,
            ),
            id="shift x=1,y=1",
        ),
        pytest.param(
            np.array(
                [
                    [True, False, True, False],
                    [False, True, False, False],
                    [True, True, False, True],
                ],
                dtype=np.bool_,
            ),
            np.array(
                [
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                dtype=np.uint8,
            ),
            -1,
            -1,
            np.array(
                [
                    [1, 0, 0, 0],
                    [1, 0, 1, 0],
                    [0, 0, 0, 0],
                ],
                dtype=np.uint8,
            ),
            id="shift x=-1,y=-1",
        ),
    ],
)
def test_add_stock_to_counter(
    input_warehouse: Warehouse,
    input_counter: Counter,
    x: int,
    y: int,
    expected_counter: Counter,
):
    add_stock_to_counter(
        input_warehouse,
        input_counter,
        x=x,
        y=y,
    )
    np.testing.assert_array_equal(input_counter, expected_counter)


def test_count_adjacent_stocks():
    warehouse = text_to_warehouse(
        """
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
        """
    )
    print("Input warehouse:\n", warehouse_to_text(warehouse), sep="")
    counter = count_adjacent_stocks(warehouse)
    removed = remove_stocks(warehouse, counter, threshold=4)
    print("Stocks removed:", removed)

    updated_warehouse_text = warehouse_to_text(warehouse)
    print("Updated warehouse:\n", updated_warehouse_text, sep="")
    assert removed == 13
    assert (
        updated_warehouse_text
        == (
            """
..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.
        """
        ).strip()
    )


if __name__ == "__main__":
    input_text = get_input()
    warehouse = text_to_warehouse(input_text)

    rounds = 0
    total_removed = 0
    start = timer.perf_counter_ns()
    while True:
        counter = count_adjacent_stocks(warehouse)
        removed = remove_stocks(warehouse, counter, threshold=4)
        if removed == 0:
            break
        total_removed += removed
        rounds += 1
        # print(f"After round {rounds:,}, removed {removed:,} stocks.")

    end = timer.perf_counter_ns()
    print(
        f"Stabilized after {rounds:,} rounds, total stocks removed: {total_removed:,}"
    )
    print(f"Execution time: {(end - start) / 1_000_000:.2f} ms")
