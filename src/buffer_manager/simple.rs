use anyhow::Result;
use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    slice,
};
use tracing::trace;

use super::BufferManager;
use crate::page::{Page, PAGE_SIZE};

pub struct SimpleBufferManager {
    page_directory: PathBuf,
    page_cache: RefCell<HashMap<u32, Box<[u8]>>>,
}

impl SimpleBufferManager {
    pub fn new<P>(dir: P) -> Self
    where
        P: AsRef<Path>,
    {
        SimpleBufferManager {
            page_directory: dir.as_ref().to_owned(),
            page_cache: RefCell::new(HashMap::new()),
        }
    }

    fn get_page(&self, offset: u32) -> Result<&[u8]> {
        if let Some(buf) = self.page_cache.borrow().get(&offset) {
            assert_eq!(buf.len(), PAGE_SIZE);
            let ptr = buf.as_ptr();
            return Ok(unsafe { slice::from_raw_parts(ptr, PAGE_SIZE) });
        }

        let path_path = self.page_directory.join(format!("{}.pages", 0));
        let mut buf_reader = BufReader::new(File::open(&path_path)?);
        buf_reader.seek(SeekFrom::Start(offset as u64 * PAGE_SIZE as u64))?;
        let mut buf = Box::new([0u8; PAGE_SIZE]);
        buf_reader.read_exact(buf.as_mut())?;
        self.page_cache.borrow_mut().insert(offset, buf);
        let ptr = self.page_cache.borrow().get(&offset).unwrap().as_ptr();
        Ok(unsafe { slice::from_raw_parts(ptr, PAGE_SIZE) })
    }
}

impl BufferManager for SimpleBufferManager {
    fn pin(&self, offset: u32) -> Result<&Page> {
        let buf = self.get_page(offset)?;
        trace!("Opened ({})", offset);
        Ok(Page::from_bytes(buf))
    }

    fn unpin(&self, page: &Page) {
        trace!("Closed ({})", page.header().page_id);
    }
}
