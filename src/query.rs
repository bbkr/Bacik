/// Represents SQL query.
/// ID must be unique across the run, which is guaranteed by the Journal.
#[derive(PartialEq, Eq, Debug)]
pub struct Query {
    pub id: u64,
    pub sql: String
}
