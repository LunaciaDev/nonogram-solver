use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::types::{
    board::Clue,
    cell::{Cell, CellStatus},
};

#[derive(Clone)]
pub struct Sector {
    pub clues: VecDeque<Rc<RefCell<Clue>>>,
    pub cell_ref: Vec<Rc<RefCell<Cell>>>,
}

pub enum SolveResult {
    Solved,
    PartialSolved,
    NoChange,
}

impl Sector {
    pub fn new(size: usize) -> Sector {
        Sector {
            clues: VecDeque::new(),
            cell_ref: Vec::with_capacity(size),
        }
    }

    /// Attempt to remove all blank cell from the sector via splitting,
    /// returning a vector of sectors created from the original sector.
    /// Return None if the sector contain no blank cell.
    /// If the Sector is split, the Sector this function is invoked on
    /// will contain the last sector created.
    ///
    /// [TODO]: This function could be improved by sharing the new Vec allocation?
    pub fn remove_blank(&mut self) -> Option<Vec<Sector>> {
        // Do we have any cell solved to be Blank?
        if self
            .cell_ref
            .iter()
            .any(|cell| matches!(cell.borrow().status, CellStatus::Blank))
        {
            return None;
        }

        // We do have Blank cell, so let's build new Sectors that does not contain blank.
        let mut result = Vec::new();
        let mut cell_iter = self.cell_ref.iter();

        loop {
            let non_empty_cells: Vec<Rc<RefCell<Cell>>> = cell_iter
                .by_ref()
                .take_while(|cell| !matches!(cell.borrow().status, CellStatus::Blank))
                .cloned()
                .collect();

            // Clone the iterator, check how many item until we hit a non-blank, then advance the actual iterator.
            // [TODO] Maybe there is a better way to approach this.
            let peeking_iter = cell_iter.clone();
            let blank_count = peeking_iter
                .take_while(|cell| matches!(cell.borrow().status, CellStatus::Blank))
                .count();

            // Advance the iterator by blank count, the count is there so the iterator is not lazy.
            cell_iter.by_ref().take(blank_count).count();

            // If we have consumed every single cell from the iterator
            if cell_iter.len() == 0 {
                // Mutate the original Sector with the last data
                // Dropping cell_iter so we can mutate the backing vector
                drop(cell_iter);
                self.cell_ref = non_empty_cells;

                // Checking just in case if the remaining clues does not fit.
                let mut clue_length = self.clues.len() as u32 - 1;
                for clue in self.clues.iter() {
                    clue_length += clue.borrow().length;
                }

                while clue_length > self.cell_ref.len() as u32 {
                    clue_length -= 1 + self
                        .clues
                        .pop_front()
                        .expect("There must still be clues left.")
                        .borrow()
                        .length;
                }

                break;
            }

            // There are cell remaining in the iterator, so we push the new Sector into result.
            let sector_length = non_empty_cells.len();
            let mut clue_length = 0;
            let mut sector_clue = VecDeque::new();

            for clue in self.clues.iter() {
                clue_length += clue.borrow().length;

                if clue_length > sector_length as u32 {
                    break;
                }

                sector_clue.push_back(clue.clone());
                clue_length += 1;
            }

            result.push(Sector {
                clues: sector_clue,
                cell_ref: non_empty_cells,
            });

            // Remove the clues that does not fit.
            let mut clue_length = self.clues.len() as u32 - 1;
            for clue in self.clues.iter() {
                clue_length += clue.borrow().length;
            }

            while clue_length > cell_iter.len() as u32 {
                clue_length -= 1 + self
                    .clues
                    .pop_front()
                    .expect("There must still be clues left.")
                    .borrow()
                    .length;
            }
        }

        Some(result)
    }

    pub fn solve(&mut self) -> SolveResult {
        todo!()
    }
}
