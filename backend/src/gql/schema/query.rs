use async_graphql::{Object};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn echo(&self, content: String) -> String {
        content
    }
}
