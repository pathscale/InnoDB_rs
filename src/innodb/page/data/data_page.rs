use crate::innodb::page::Page;
pub struct DataPageHeader {
    pub offset: u16,
}
pub struct DataPage {
    pub page: Page,
}
impl DataPage {
    pub fn new(page: Page) -> Self {
        Self { page }
    }
    pub fn header(&self) -> &DataPageHeader {
        unsafe { std::mem::transmute(self.page.body().as_ptr()) }
    }
    pub fn header_mut(&mut self) -> &mut DataPageHeader {
        unsafe { std::mem::transmute(self.page.body_mut().as_mut_ptr()) }
    }
}
