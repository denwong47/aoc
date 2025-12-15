# Day 4

This day is done in Python, since `NumPy` was instrumental in simplifying matrix operations for this problem.

This problem is about "counting the number of adjacent occupied tiles from any given tile in a grid". For the grid size involved, brute forcing it with nested loops is feasible, but not elegant.

Instead, we can use `NumPy`'s array slicing capabilities to create shifted versions of the grid, and then sum these shifted arrays to get the count of occupied adjacent tiles for each position in the grid. In other words, instead of walking a person through each cell to check its neighbours, we shift the floor in all 8 directions, and accumulate the "views" of each shift.

The most complex logic comes from the need to handle the edges of the grid correctly, ensuring that we don't ask for `ndarray[-1:x, -1:y]` which `NumPy` will reject. In hindsight, padding the grid with a border of empty tiles would have simplified this logic significantly.