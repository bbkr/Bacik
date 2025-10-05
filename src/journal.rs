use std::fmt::Display;

use crate::query::Query;

/// Journal tracks Queries, Targets, exectution statuses and optional rows.
/// It allows to collect and analyze data from given run or resume failed run.
pub trait Journal {
    /// ID is used to distinguish runs and must be unique.
    /// Trait does not enforce how concrete Journal implementations identify runs,
    /// for example file based Journal may use file name String (which is unique within Journals directory).
    /// The only requirement is that it must implement [`std::fmt::Display`] trait for easy UI formatting.
    type ID: Display;

    type Error;

    /// Create new Journal.
    fn new() -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Return ID of given run.
    fn id(&self) -> Self::ID;

    /// Open existing Journal identified by ID, usually to resume failed run.
    fn open(id: Self::ID) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Add new Query by assigning incremental ID to SQL statement.
    /// Queries will be executed in order they were registered.
    /// Registered Queries will be available through [`Self::get_queries()`] method.
    fn register_query(&mut self, sql: &str );

    /// Get all registered Queries in execution order.
    fn get_queries(&self) -> &[Query];

}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestJournal {
        id: u64,

        queries: Vec<Query>
    }

    impl Journal for TestJournal {
        type ID = u64;
        type Error = String;

        fn new() -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            Ok(Self { id: 1, queries: Vec::new() })
        }

        fn id(&self) -> Self::ID {
            self.id
        }

        fn open(id: Self::ID) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            Ok(Self { id, queries: Vec::new() })
        }

        fn register_query(&mut self, sql: &str ) {
            let id = self.queries.len() + 1;
            self.queries.push( Query {id: id as u64, sql: sql.to_owned()});
        }

        fn get_queries(&self) -> &[Query] {
            self.queries.as_slice()
        }
    }

    #[test]
    fn new() {
        assert_eq!(TestJournal::new().unwrap().id(), 1);
    }

    #[test]
    fn open() {
        assert_eq!(TestJournal::open(2).unwrap().id(), 2);
    }

    #[test]
    fn queries() {
        let mut j = TestJournal::new().unwrap();

        j.register_query( "SELECT 1" );
        j.register_query( "SELECT 2" );

        assert_eq!(j.get_queries(), [ Query { id: 1, sql: "SELECT 1".to_string() }, Query { id: 2, sql: "SELECT 2".to_string() } ]);

    }
}
