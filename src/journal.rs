use std::fmt::Display;
use crate::query::Query;

mod volatile;

/// Journal tracks Queries, Targets, exectution statuses and optional rows.
/// It allows to collect and analyze data from given run or resume failed run.
pub trait Journal: Sized {
    /// ID is used to distinguish runs and must be unique.
    /// Trait does not enforce how concrete Journal implementations identify runs,
    /// for example file based Journal may use file name String (which is unique within Journals directory).
    /// The only requirement is that it must implement [`std::fmt::Display`] trait for easy UI formatting.
    type ID: Display;

    type Error;

    /// Create new Journal.
    fn new() -> Result<Self, Self::Error>;

    /// Return ID of given run.
    fn id(&self) -> Self::ID;

    /// Open existing Journal identified by ID, usually to resume failed run.
    fn open(id: Self::ID) -> Result<Self, Self::Error>;

    /// Add new Query.
    /// Queries will be executed in order they were added.
    fn add_query(&mut self, sql: &str );

    /// Get all registered Queries in execution order.
    fn get_queries(&self) -> &[Query];

}
