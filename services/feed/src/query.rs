use async_graphql::Object;

use crate::feed::Feed;

pub struct Query;

#[Object]
impl Query {
    async fn feed(&self) -> Feed {
        Feed {
            content: "Crab".to_string(),
        }
    }
}
