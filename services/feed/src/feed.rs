use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Feed {
    pub content: String,
}
