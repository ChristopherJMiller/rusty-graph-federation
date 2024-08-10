use async_graphql::{Object, ID};

use crate::user::User;

pub struct Query;

#[Object]
impl Query {
    #[graphql(entity)]
    async fn find_user_by_id(&self, id: ID) -> User {
        User { id }
    }

    #[graphql(entity)]
    async fn find_user_by_id_with_username(
        &self,
        #[graphql(key)] id: ID,
        username: String,
    ) -> User {
        User { id }
    }

    #[graphql(entity)]
    async fn find_user_by_id_and_username(&self, id: ID, username: String) -> User {
        User { id }
    }
}
