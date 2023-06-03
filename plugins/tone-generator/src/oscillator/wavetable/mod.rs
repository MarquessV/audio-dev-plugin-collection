pub use noise::get_noise_table;
pub use saw::get_saw_table;
pub use sin::get_sin_table;
pub use square::get_square_table;
pub use triangle::get_triangle_table;

mod noise;
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

/// Ensures that the static tables defined using [`std::sync::OnceLock`] are initialized.
/// This should be called before the tables are used in an audio thread to avoid
/// an expensive computation and allocation during processing.
pub fn initialize_tables() {
    get_sin_table();
    get_triangle_table();
    get_saw_table();
    get_square_table();
    get_noise_table();
}
