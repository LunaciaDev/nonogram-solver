# Nonogram Solver

## Method

Each cell of the nonogram can be in one of the 3 state:

- Empty: The cell is not solved.
- Fill: The cell is solved to be a filled cell.
- Blank: The cell is solved to be a blank cell.

We define a sector as a slice of a row/column, with associated clues. A sector has the following properties:

- It must not contain a blank cell. If a sector contain a blank cell, it is split into 2 smaller sector at the blank cell, not including the blank.
- A sector inherit partial clues from its row/column. These clues may be duplicated (shared by multiple sector on the same rol/col), cannot be solved (how do you prove that a clue is solved in this sector when it could be solved at the other sector), thus require deduplication.

Cells in sector may change state via the following:

- If `sum(clues) + len(clues) - 1 = len(sector)`, all cell in the sector can be solved.
- Otherwise, we can get block filled based on the value of the clues. If `len(sector) - (sum(clues) + len(clues) - 1) > clue`, that clue can have `len(sector) - (sum(clues) + len(clues) - 1) - clue` cells solved. The location of the solved cell also depend on that calculation (TODO: expand this section).
- If the length of empty cell from edge to first filled block (`empty_len`) is smaller than the first/last clue, we can fill more cell to the right of the filled cell (`fill_len`) until `empty_len + fill_len = first/last clue`.
- If the length of empty cell from edge to the first filled block where the next block is empty (`b_len`) is bigger than the first clue, but smaller than first + second clue + 1, we can mark `b_len - first clue` cell from the edge as empty.

With these rules, we can recursively solve/split sector until all has been solved.