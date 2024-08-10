use async_graphql::{Object, ID};

use crate::user::User;

pub struct Query;

#[Object]
impl Query {
    async fn user(&self, id: ID) -> User {
        User { id }
    }
}
