pub use kvdb::KeyValueDB;

#[cfg(not(target_arch = "wasm32"))]
pub use kvdb_rocksdb::{Database as RocksDatabase, Error as RocksError};
#[cfg(target_arch = "wasm32")]
pub use kvdb_web::{Database as WebDatabase, Error as WebError};
pub use kvdb_memorydb::InMemory;
