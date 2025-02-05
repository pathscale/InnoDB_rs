// pub mod record;
//
// use anyhow::{anyhow, Error, Result};
// use num_enum::TryFromPrimitive;
// use record::Record;
//
// use crate::innodb::InnoDBError;
//
// use super::{Page, PageType};
//
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// #[repr(u8)]
// pub enum IndexFormat {
//     Redundant = 0,
//     Compact = 1,
// }
//
// #[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
// #[repr(u16)]
// pub enum PageDirection {
//     Left = 1,
//     Right = 2,
//     SameRec = 3,  // Not Used?
//     SamePage = 4, // Not Used?
//     NoDirection = 5,
// }
//
// /*
//  Actual Layout
//  +----------------------------------------------+----------------------------------------------+
//  |        Number of Directory Slots (2)         |            Heap Top Position (2)             |
//  +-----------+----------------------------------+----------------------------------------------+
//  |Format Flag|      Number of Heap Records      |         First Garbage Record Offset          |
//  +-----------+----------------------------------+----------------------------------------------+
//  |                Garbage Space                 |             Last Insert Position             |
//  +----------------------------------------------+----------------------------------------------+
//  |                Page Direction                |     Number of Inserts in Page Direction      |
//  +----------------------------------------------+----------------------------------------------+
//  |              Number of Records               |          Maximum Transaction ID (8)          |
//  +----------------------------------------------+----------------------------------------------+
//  |                               Maximum Transaction ID (cont.)                                |
//  +----------------------------------------------+----------------------------------------------+
//  |        Maximum Transaction ID (cont.)        |                  Page Level                  |
//  +----------------------------------------------+----------------------------------------------+
//  |                                        Index ID (8)                                         |
//  +---------------------------------------------------------------------------------------------+
//  |                                      Index ID (cont.)                                       |
//  +---------------------------------------------------------------------------------------------+
// */
// #[derive(Debug, Clone)]
// pub struct IndexHeader {
//     pub number_of_directory_slots: u16,
//     pub heap_top_position: u16,
//     pub format: IndexFormat,         // highest bit of the next field.
//     pub number_of_heap_records: u16, // lower 15 bits
//     pub first_garbage_record_offset: u16,
//     pub garbage_space: u16,
//     pub last_insert_position: u16,
//     pub page_direction: PageDirection,
//     pub number_of_inserts_in_page_direction: u16,
//     pub number_of_records: u16,
//     pub maximum_transaction_id: u64,
//     pub page_level: u16,
//     pub index_id: u64,
// }
//
// impl IndexHeader {
//     pub fn from_bytes(data: &[u8]) -> Result<Self> {
//         if data.len() < 36 {
//             return Err(Error::msg("Data slice is too short"));
//         }
//
//         let format_and_num_heap_records_raw = u16::from_be_bytes([data[4], data[5]]);
//
//         Ok(IndexHeader {
//             number_of_directory_slots: u16::from_be_bytes([data[0], data[1]]),
//             heap_top_position: u16::from_be_bytes([data[2], data[3]]),
//             format: if (format_and_num_heap_records_raw & 0x8000) == 0 {
//                 IndexFormat::Redundant
//             } else {
//                 IndexFormat::Compact
//             },
//             number_of_heap_records: format_and_num_heap_records_raw & 0x7FFF,
//             first_garbage_record_offset: u16::from_be_bytes([data[6], data[7]]),
//             garbage_space: u16::from_be_bytes([data[8], data[9]]),
//             last_insert_position: u16::from_be_bytes([data[10], data[11]]),
//             page_direction: PageDirection::try_from_primitive(u16::from_be_bytes([
//                 data[12], data[13],
//             ]))?,
//             number_of_inserts_in_page_direction: u16::from_be_bytes([data[14], data[15]]),
//             number_of_records: u16::from_be_bytes([data[16], data[17]]),
//             maximum_transaction_id: u64::from_be_bytes([
//                 data[18], data[19], data[20], data[21], data[22], data[23], data[24], data[25],
//             ]),
//             page_level: u16::from_be_bytes([data[26], data[27]]),
//             index_id: u64::from_be_bytes([
//                 data[28], data[29], data[30], data[31], data[32], data[33], data[34], data[35],
//             ]),
//         })
//     }
// }
//
// #[derive(Debug)]
// pub struct IndexPage<'a> {
//     pub page: Page<'a>,
//     pub index_header: IndexHeader,
// }
//
// impl<'a> IndexPage<'a> {
//     pub fn try_from_page(page: Page<'a>) -> Result<Self> {
//         if page.header.page_type != PageType::Index {
//             return Err(anyhow!(InnoDBError::InvalidPageType {
//                 expected: PageType::Index,
//                 has: page.header.page_type
//             }));
//         }
//
//         Ok(IndexPage {
//             index_header: IndexHeader::from_bytes(page.body())?,
//             page,
//         })
//     }
//
//     pub fn record_at(&self, offset: usize) -> Result<Record> {
//         Record::try_from_offset(self.page.raw_data, offset)
//     }
//
//     pub fn infimum(&self) -> Result<Record> {
//         self.record_at(99)
//     }
//
//     pub fn supremum(&self) -> Result<Record> {
//         self.record_at(112)
//     }
// }
