/// Represents database instance to run Queries on.
/// ID should be unique across the run and preferably obtained by tracking Target in Journal.
struct Target {
    id: u64,
    host: String,
    port: u16,
    name: Option<String>,
}
