mod query;
pub use query::QueryRoot;

pub type AppSchema = async_graphql::Schema<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

#[cfg(test)]
mod test {
    use super::*;
    use async_graphql::{EmptyMutation,EmptySubscription};

    #[tokio::test]
    async fn add() {
        let schema = async_graphql::Schema::new(QueryRoot,EmptyMutation,EmptySubscription );
        let _res = dbg!(schema.execute("{ add(x: 10, y: 20) }").await);
    }
}
