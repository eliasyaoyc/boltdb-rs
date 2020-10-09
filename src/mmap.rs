use memmap::MmapOptions;
use std::io::Write;
use std::fs::File;

use crate::db::DB;

pub fn mmap(mut db: DB, mmap_size: usize) -> ::std::io::Result<DB> {
    let file = File::open(db.path())?;
    let mut opt = MmapOptions::new();
    db.mmap = Some(unsafe { opt.map_exec(&file) }?);
    db.mmap_size = mmap_size;
    Ok(db)
}

#[test]
fn it_works() {
    let buffer = "hello word".as_bytes();
    let mut file = tempfile::tempfile().unwrap();
    file.write_all(buffer);
    let mut opt = MmapOptions::new();
    let mut mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    assert_eq!(buffer, mmap.as_ref());
    // println!("{:#?}", String::from_utf8_lossy(&mmap[0..buffer.len()]));
}
