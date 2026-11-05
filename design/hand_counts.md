# Hand counts

Counts are rows 0 to 9 and columns 0 to 19. Each digit is the number of live
neighbors of that cell. The cell itself is not counted, and anything off the
edge of the grid counts as dead.

## Middle: live cells at (4,10), (5,10), (6,10)

Worked out by hand. Rows 0-2 and 8-9 are all zeros.

```
row 3: 00000000011100000000
row 4: 00000000021200000000
row 5: 00000000032300000000
row 6: 00000000021200000000
row 7: 00000000011100000000
```

## Corner: live cells at (0,0), (1,0), (2,0)

Worked out by hand. Rows 4-9 are all zeros.

```
row 0: 12000000000000000000
row 1: 23000000000000000000
row 2: 12000000000000000000
row 3: 11000000000000000000
```

Program output (print_neighbor_counts): agreed with my hand counts, every row.
The program did not panic on the top edge, the left edge or the corner.
