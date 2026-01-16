# wfc3d
Tile implementation of the Wave Function Collapse applied to 3D objects

# to run:
`m̀ake run`


# pseudo code of implementation:
```
    all cells are initialized

    pick one cell with min entropy (random at first)
        collapse it (pick on option from possibilities)
        propagate it (update all cells surrounding it)
        repeat
```
