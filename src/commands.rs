
pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognized,
}

pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognized,
    PrepareSyntaxError,
}

pub enum StatementTypes {
    StatementInsert,
    StatementSelect,
    Unknown,
}

pub struct Statement {
    pub statementType: StatementTypes,
    pub row_to_insert: super::table::Row,
}

impl Default for Statement {
    fn default() -> Statement {
        Statement {
            statementType: StatementTypes::Unknown,
            row_to_insert: super::Row::default(),
        }
    }
}

pub enum ExecuteResult {
    EXECUTE_SUCCESS,
    EXECUTE_FAILURE,
    EXECUTE_TABLE_FULL,
}