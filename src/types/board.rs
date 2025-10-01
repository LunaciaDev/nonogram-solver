use std::{cell::RefCell, iter::zip, rc::Rc};

use crate::types::{cell::Cell, sector::Sector};

pub struct Board {
    pub size: usize,
    pub rows: Vec<Line>,
    pub cols: Vec<Line>,

    backing_data: Vec<Vec<Rc<RefCell<Cell>>>>,
}

#[derive(Clone)]
pub struct Line {
    pub clues: Vec<Rc<RefCell<Clue>>>,
    pub sectors: Vec<Sector>,
}

pub struct Clue {
    pub length: u32,
    pub is_solved: bool
}

impl Clue {
    pub fn new(value: u32) -> Self {
        Clue {
            length: value,
            is_solved: false,
        }
    }
}

impl Board {
    pub fn new(size: usize) -> Self {
        // [TODO]: Maybe consider going unsafe to chunk the backing data as 1 contiguous chunk.
        // Initialize the backing data.
        let backing_data = vec![vec![Rc::new(RefCell::new(Cell::default())); size]; size];

        // Allocate the views into backing data.
        let mut rows = vec![
            Line {
                clues: Vec::new(),
                sectors: vec![Sector::new(size); 1],
            };
            size
        ];
        let mut cols = vec![
            Line {
                clues: Vec::new(),
                sectors: vec![Sector::new(size); 1],
            };
            size
        ];

        // Initialize the views into backing data.
        for (row_view, backing_row) in zip(rows.iter_mut(), backing_data.iter()) {
            for (col_view, backing_cell) in zip(cols.iter_mut(), backing_row.iter()) {
                row_view
                    .sectors
                    .first_mut()
                    .expect("The first element must have been created")
                    .cell_ref
                    .push(backing_cell.clone());
                col_view
                    .sectors
                    .first_mut()
                    .expect("The first element must have been created")
                    .cell_ref
                    .push(backing_cell.clone());
            }
        }

        Board {
            size,
            rows,
            cols,
            backing_data,
        }
    }
}
