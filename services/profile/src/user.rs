use async_graphql::{SimpleObject, ID};

#[derive(SimpleObject)]
pub struct User {
    pub id: ID,
}
