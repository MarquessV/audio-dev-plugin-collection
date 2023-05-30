pub use saw::SAW_TABLE;
pub use sin::SIN_TABLE;
pub use square::SQUARE_TABLE;
pub use triangle::TRIANGLE_TABLE;

mod saw;
mod sin;
mod square;
mod triangle;

pub const TABLE_SIZE: usize = 1 << 15;
pub const TABLE_SIZE_MASK: usize = TABLE_SIZE - 1;

#[derive(Debug)]
pub struct Table {
    table: [f32; TABLE_SIZE],
}

impl std::ops::Index<usize> for Table {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[index]
    }
}

impl Table {
    #[must_use]
    pub const fn get_table(&self) -> &[f32; TABLE_SIZE] {
        &self.table
    }
}

/// Ensures that the static tables defined using [`once_cell`] are initialized.
/// This should be called before the tables are used in an audio thread to avoid
/// an expensive computation and allocation during processing.
pub fn initialize_tables() {
    _ = &*SIN_TABLE;
    _ = &*TRIANGLE_TABLE;
    _ = &*SAW_TABLE;
    _ = &*SQUARE_TABLE;
}
