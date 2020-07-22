pub use kvdb::KeyValueDB;

pub use crate::backend::{Database, Error, Result};

mod backend;

// TODO: Different backends require different parameters
#[cfg(not(target_arch = "wasm32"))]
pub fn open_database() -> Result<Database> {
    Ok(Database::open(&Default::default(), "database")?)
}

#[cfg(target_arch = "wasm32")]
pub fn open_database() -> Result<Database> {
    let db = futures::executor::block_on(Database::open("Test".to_owned(), 4))?;
    Ok(db)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanity() {
        let db = open_database().unwrap();

        assert!(true);
    }
}
