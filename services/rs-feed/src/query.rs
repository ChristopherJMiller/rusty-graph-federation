use async_graphql::{Object, ID};

use crate::feed::Feed;

pub struct Query;

#[Object]
impl Query {
    async fn feed(&self, _id: ID) -> Feed {
        Feed {
            content: "Crab".to_string(),
        }
    }
}
