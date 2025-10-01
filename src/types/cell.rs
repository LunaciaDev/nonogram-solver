#[derive(Default, Clone, Copy)]
pub struct Cell {
    pub status: CellStatus,
}

#[derive(Default, Clone, Copy)]
pub enum CellStatus {
    #[default]
    /// The cell has not been solved.
    Empty,
    /// Cell solved to be filled.
    Fill,
    /// Cell solved to be blank.
    Blank,
}

impl Cell {
    pub fn change_state(&mut self, state: CellStatus) {
        if let CellStatus::Empty = self.status {
            self.status = state;
            return;
        }

        panic!("Cannot change state on a solved cell");
    }
}