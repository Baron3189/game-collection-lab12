
Inputs: whether the cell is alive, and its number of live neighbors (0 to 8).
Rows needed: 2 (alive or dead) x 9 (neighbor counts) = 18.

| alive | neighbors | next generation |
|-------|-----------|-----------------|
| true  | 0 | false |
| true  | 1 | false |
| true  | 2 | true  |
| true  | 3 | true  |
| true  | 4 | false |
| true  | 5 | false |
| true  | 6 | false |
| true  | 7 | false |
| true  | 8 | false |
| false | 0 | false |
| false | 1 | false |
| false | 2 | false |
| false | 3 | true  |
| false | 4 | false |
| false | 5 | false |
| false | 6 | false |
| false | 7 | false |
| false | 8 | false |


Only 3 of the 18 rows give a live cell in the next generation.
Every other row is dead.
