## Day 8

This problem itself was trivial to solve on modern hardware, but it is one that has the most direct application at work, so I wanted to "do it proper".

Among the 1000 X,Y,Z coordinates, there are 495,000 unique pairs of points, each with a Euclidean distance to be calculated. In the most naive sense, **if we have the full list of these distances, sorted ascending**, the problem is solved.

#### So our job is to restructure this problem to avoid calculating all 495,000 distances, and lazy evaluate them as needed.

Let's work backwards from the naive solution.

Assume we have:

- remove all self-referential distances (i.e. distance from node A to node A),
- remove all duplicate distances (i.e. A-B is the same as B-A), and then
- sort the remaining distances in ascending order,

then this hypothetical list can look like:

```text
    p3 -> p6 = 2
    p4 -> p5 = 6
    p1 -> p2 = 7
    p1 -> p3 = 9
    p5 -> p6 = 9
    ...
```

However, before we get to this point, we can instead have one sorted list per source node instead:

```text
    p1: p1 -> p2 = 7, p1 -> p3 = 9, ...
    p2: p2 -> p1 = 7, p2 -> p3 = 10, ...
    p3: p3 -> p6 = 2, p3 -> p1 = 9, ...
    p4: p4 -> p5 = 6, p4 -> p3 = 11, ...
    p5: p5 -> p4 = 6, p5 -> p6 = 9, ...
    p6: p6 -> p3 = 2, p6 -> p5 = 9, ...
```

Then we can scan the first nearest-neighbour of each node, the pop the smallest distance
from that list, shifting the next nearest-neighbour of that node to the front:

```text
    - popped p3 -> p6 = 2
    - p1: p1 -> p2 = 7, p1 -> p3 = 9, ...
    - p2: p2 -> p1 = 7, p2 -> p3 = 10, ...
    - p3: p3 -> p1 = 9, *p3 -> p2 = 10*, ...
    - p4: p4 -> p5 = 6, p4 -> p3 = 11, ...
    - p5: p5 -> p4 = 6, p5 -> p6 = 9, ...
    - p6: p6 -> p3 = 2, p6 -> p5 = 9, ...
```

**There is no difference in the final sorted order of distances between this approach and the naive approach**. However, this approach allows for lazy evaluation of distances, and only requires the computation of nearest-neighbours for each node, when the node was popped: in the example above, we may not even know about ``p3 -> p2 = 10`` at the time when we pop ``p3 -> p6 = 2``, and we only compute it afterwards to fill the gap.

You may notice that the above examples have a lot of duplicate distances (e.g. ``p1 -> p2 = 7`` and ``p2 -> p1 = 7``). This can be avoided by only asking each node to find its nearest-neighbours where ``pN`` is higher than itself (i.e. only the bottom half of the distance matrix):

```text
    p1: p1 -> p2 = 7, p1 -> p3 = 9, ...
    p2: p2 -> p3 = 10, ...
    p3: p3 -> p6 = 2, ...
    p4: p4 -> p5 = 6, ...
    p5: p5 -> p6 = 9, ...
```

``p6`` has no entries because it is the highest node. Any node that is only connected by nodes lower than itself will not have any entries as well.

This ensures our whole table only has ``<N-1`` entries at any time, and they shall always be non-repeating. We can be assured that this produces the same result, because if one of the repeats was the smallest distance, its counterpart in the other direction would have been in the heap at the same time, so keeping both is redundant.

#### Now how do we get each node to find its nearest-neighbours efficiently?

A KD-Tree is a spatial partitioning data structure that allows for efficient nearest-neighbour searches. By building a KD-Tree from the input nodes, we can quickly find the nearest-neighbour of any given node without having to compute the distance to every other node.

## Summary

Our approach:

- builds a KD-Tree from the input nodes for efficient nearest-neighbour computation,
- for each node, finds its nearest-neighbour that has not already been paired with it (i.e. only the bottom half of the distance matrix), and
- stores these relations in a min-heap sorted by distance,
- when popping a relation from the heap, fans out from the ``node_a`` of that relation to find its next nearest-neighbour that has not already been paired with it, and pushes that new relation onto the heap, replacing the popped relation.
- this continues until all unique relations have been popped from the heap, or some stopping condition is met, e.g. all nodes have been joined into a single graph.