use super::page::Page;
use anyhow::Result;

pub mod lru;
pub mod simple;

pub trait BufferManager {
    fn pin(&self, offset: u32) -> Result<&Page>;
    fn unpin(&self, page: &Page);
}
