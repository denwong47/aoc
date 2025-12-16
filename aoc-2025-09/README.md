# Day 9

The tough stuff begins.

For the amount of points involved, brute-forcing all the areas between any two given diagonal points and sorting them is doable, but the main challenge is about checking if the resultant rectangle is wholly within the input polygon.

The naive approach is to flood fill the polygon area, however since the area is `100000 x 100000`, this is not feasible; in fact a recursion algorithm would likely hit stack limits.

**The two key questions are:**
1. how do we know if a polygon edge intersects with a rectangle edge?
2. if that intersection does not occur, how do we know if the rectangle is inside or outside the polygon?

### Line intersection - Visibility bounds

For each candidate point, we can evaluate its visibility bounds by projecting orthogonal lines away from it, checking for intersections with polygon edges.

The following cases are possible:
- no intersections: we can't form a rectangle on this side, because this side is wholly outside the polygon
- hits the middle of some edges: the closest intersection point is the visibility bound on this side
- hits a vertex: _this is tricky_ - if this vertex is a convex corner, then this is the visibility bound on this side; if it is a concave corner, then we should continue searching.

> [!NOTE]
> The above logic assumes the polygon is simple (i.e. no self-intersections). If the polygon is complex, then we need to handle additional cases.

> [!TIP]
> **In actual fact, the last case never occurs in the input data**, as no >2 points share the same X or Y coordinates, except the mouth of the "pacman" shape, but due to the way the input is structured, you won't find an answer along that line, so the algorithm still produces the correct answer. The following discussion is for completeness only, so that our solution passes the pragmatic test.

### Is this corner convex or concave?

Our algorithm above does not have any concept of "inside" or "outside" the polygon; it only knows about intersections. Therefore, when we hit a vertex, it is not possible to know if this is a convex or concave corner without additional information.

However, any lines that
- originate from within the polygon,
- ends outside the polygon, and
- does not hit any vertices along the way
**MUST** hit an odd number of edges along the way, whichever the direction.

Conversely, any lines that
- originate from outside the polygon,
- ends outside the polygon, and
- does not hit any vertices along the way
**MUST** hit an even number of edges along the way, whichever the direction.

Since we know that the polygon never meets the bounds of the coordinate system, we can confidently draw lines from the candidate point to the outside of the coordinate system, and count the number of intersections along the way. If we hit a vertex, we can change the direction slightly to avoid hitting the vertex directly, and repeat the process.

For example:

```text

XXXA
   X
   X
   X
   BXXXXXXXX#
   *        X
            X
   CXXXXXXXXX
   X
```

From point `A`, we want to find the visibility bounds downwards. We found `B`, which is a vertex. We can then go forward one unit to `*`. Knowing that there is a vertex right above `*`, we will count intersections towards the right instead. Assuming we found one intersection, we know that `B` is concave, and we can continue searching downwards towards `C`. If we found an even number of intersections instead, we know that `B` is convex, and we can stop here.
