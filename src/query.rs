/// Represents SQL query.
/// ID should be unique across the run and preferably obtained by tracking Query in Journal.
#[derive(PartialEq, Eq, Debug)]
pub struct Query {
    pub id: u64,
    pub sql: String
}
