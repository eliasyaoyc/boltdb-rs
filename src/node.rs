use crate::bucket;
use crate::bucket::Bucket;
use crate::page::PgId;

type Nodes = Vec<u8>;
type NextNode = Option<Box<Node>>;

/// Represents an in-memory, deserialized `page`.
struct Node {
    bucket: Bucket,
    is_leaf: bool,
    unbalanced: bool,
    spilled: bool,
    key: Vec<u8>,
    pgid: PgId,
    parent: NextNode,
    children: Nodes,
    inodes: Inodes,
}


type Inodes = Vec<Inode>;

struct Inode {}