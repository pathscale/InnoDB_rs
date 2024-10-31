use derive_more::{Display, From};
use num_enum::TryFromPrimitive;
use rkyv::{Archive, Deserialize, Serialize};
use std::mem::transmute;

pub const PAGE_SIZE: usize = 4 * 1024;
pub const PAGE_HEADER_SIZE: usize = size_of::<PageHeader>();
pub mod data;
pub mod index;

/// Represents page's identifier. Is unique within the table bounds
#[derive(
    Archive,
    Copy,
    Clone,
    Deserialize,
    Debug,
    Display,
    Eq,
    From,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct PageId(pub u32);

impl From<PageId> for usize {
    fn from(value: PageId) -> Self {
        value.0 as usize
    }
}
impl From<usize> for PageId {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}
impl From<i32> for PageId {
    fn from(value: i32) -> Self {
        Self(value as u32)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u16)]
pub enum PageType {
    /// Freshly allocated
    Allocated = 0,
    /// Undo log
    UndoLog = 2,
    /// File segment inode
    Inode = 3,
    /// Insert buffer free list
    IbufFreeList = 4,
    /// Insert buffer bitmap
    IbufBitmap = 5,
    /// System internal
    Sys = 6,
    /// Transaction system header
    TrxSys = 7,
    /// File space header
    FspHdr = 8,
    /// Extent descriptor
    Xdes = 9,
    /// Uncompressed BLOB
    Blob = 10,
    /// First compressed BLOB
    Zblob = 11,
    /// Subsequent compressed BLOB
    Zblob2 = 12,
    /// Unknown
    Unknown = 13,
    /// Compressed
    Compressed = 14,
    /// Encrypted
    Encrypted = 15,
    /// Compressed and Encrypted
    CompressedAndEncrypted = 16,
    /// Encrypted R-tree
    EncryptedRtree = 17,
    /// Uncompressed SDI BLOB
    SdiBlob = 18,
    /// Compressed SDI BLOB
    SdiZblob = 19,
    /// Legacy doublewrite buffer
    LegacyDblwr = 20,
    /// Rollback Segment Array
    RsegArray = 21,
    /// Index of uncompressed LOB
    LobIndex = 22,
    /// Data of uncompressed LOB
    LobData = 23,
    /// First page of an uncompressed LOB
    LobFirst = 24,
    /// First page of a compressed LOB
    ZlobFirst = 25,
    /// Data of compressed LOB
    ZlobData = 26,
    /// Index of compressed LOB
    ZlobIndex = 27,
    /// Fragment of compressed LOB
    ZlobFrag = 28,
    /// Index of fragment for compressed LOB
    ZlobFragEntry = 29,
    /// Serialized Dictionary Information
    SDI = 17853,
    /// R-tree index
    RTree = 17854,
    /// B+Tree index
    Index = 17855,
}

#[allow(clippy::derivable_impls)]
impl Default for PageType {
    fn default() -> Self {
        PageType::Allocated
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(align(8))]
pub struct PageHeader {
    pub page_id: PageId,
    pub prev: u32,
    pub next: u32,
    pub lsn: u64,
    pub page_type: PageType,
}

#[derive(PartialEq)]
#[repr(align(8))]
pub struct Page {
    pub raw_data: [u8; PAGE_SIZE],
}
impl Page {
    pub fn from_bytes(bytes: &[u8]) -> &Self {
        assert_eq!(bytes.len(), PAGE_SIZE);
        unsafe { transmute(bytes.as_ptr()) }
    }
    pub fn from_bytes_mut(bytes: &mut [u8]) -> &mut Self {
        assert_eq!(bytes.len(), PAGE_SIZE);
        unsafe { transmute(bytes.as_ptr()) }
    }
    pub fn header(&self) -> &PageHeader {
        unsafe { transmute(&self.raw_data) }
    }
    pub fn header_mut(&mut self) -> &mut PageHeader {
        unsafe { transmute(&mut self.raw_data) }
    }
    pub fn body(&self) -> &[u8] {
        &self.raw_data[size_of::<PageHeader>()..]
    }
    pub fn body_mut(&mut self) -> &mut [u8] {
        &mut self.raw_data[size_of::<PageHeader>()..]
    }
}
impl Default for Page {
    fn default() -> Self {
        Self {
            raw_data: [0; PAGE_SIZE],
        }
    }
}
