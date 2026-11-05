# The eight offsets (dr, dc)

`dr` is the change in row and `dc` is the change in column, measured from the middle cell.

```
+----------+----------+----------+
| (-1,-1)  | (-1, 0)  | (-1,+1)  |
+----------+----------+----------+
| ( 0,-1)  |    XX    | ( 0,+1)  |
+----------+----------+----------+
| (+1,-1)  | (+1, 0)  | (+1,+1)  |
+----------+----------+----------+
```

The middle square `(0, 0)` is crossed out (XX). It is the cell itself, not a neighbor.
The other 8 squares are the neighbors.

Row goes down as `dr` increases, and column goes right as `dc` increases.
