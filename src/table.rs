use std::fmt;

pub mod size {
    use std::mem::size_of;

    pub const ROW_SIZE: usize = size_of::<super::Row>();
    pub const PAGE_SIZE: usize = 4096;
    pub const TABLE_MAX_PAGES: usize = 100;
    pub const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
    pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;
}

pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl Default for Row {
    fn default() -> Row {
        Row {
            id: 0,
            username: String::from("nate"),
            email: String::from("test@test.com"),
        }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {} {}", self.id, self.username, self.email)
    }
}

pub struct Page {
    pub num_rows: usize,
    pub rows: Vec<Row>,
}

pub struct Table {
    pub num_rows: usize,
    pub pages: Vec<Page>,
}

impl Table {
    pub fn insert_row(&mut self, row_to_insert: Row) {
        let page_num = self.num_rows / size::ROWS_PER_PAGE;

        self.pages[page_num].rows.push(row_to_insert);
        self.pages[page_num].num_rows += 1;
    }
}

pub fn new_table() -> Table {
    Table {
        num_rows: 0,
        pages: vec![Page {
            num_rows: 0,
            rows: Vec::with_capacity(size::ROWS_PER_PAGE),
        }],
    }
}
