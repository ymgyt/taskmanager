mod query;
pub use query::QueryRoot;

pub type AppSchema = async_graphql::Schema<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;
