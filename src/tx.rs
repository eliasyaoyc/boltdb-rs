/// `TxId` represents the internal transaction identifier.
pub type TxId = u64;

/// `Tx` represents a read-only or read-write transaction on the database.
/// Read-only transactions can be used for retrieving values for keys and creating cursors.
/// Read/write transactions can create and remove buckets and create and remove keys.
///
/// IMPORTANT: You must commit or rollback transactions when you are done with
/// them. Pages can not be reclaimed by the writer until no more transactions
pub struct Tx {}