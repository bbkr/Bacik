use super::Journal;
use crate::query::Query;

/// In memory Journal.
/// Not recommended for real world application.
struct Volatile {
    id: u64,
    queries: Vec<Query>
}

impl Journal for Volatile {
    type ID = u64;
    type Error = String;

    fn new() -> Result<Self, Self::Error> {
        Ok(Self { id: 1, queries: Vec::new() })
    }

    fn id(&self) -> Self::ID {
        self.id
    }

    fn open(id: Self::ID) -> Result<Self, Self::Error> {
        Err("Volatile Journal has no persistent state to be opened.".to_string())
    }

    fn add_query(&mut self, sql: &str ) {
        let id = self.queries.len() + 1;
        self.queries.push( Query {id: id as u64, sql: sql.to_owned()});
    }

    fn get_queries(&self) -> &[Query] {
        self.queries.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(Volatile::new().unwrap().id(), 1);
    }

    #[test]
    fn open() {
        assert!(matches!(Volatile::open(1), Err(_)));
    }

    #[test]
    fn queries() {
        let mut j = Volatile::new().unwrap();

        j.add_query( "SELECT 1" );
        j.add_query( "SELECT 2" );

        assert_eq!(j.get_queries(), [ Query { id: 1, sql: "SELECT 1".to_string() }, Query { id: 2, sql: "SELECT 2".to_string() } ]);

    }
}
