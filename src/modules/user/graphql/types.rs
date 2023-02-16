use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{ComplexObject, Context, Enum, SimpleObject, ID};
use serde::{Deserialize, Serialize};

use crate::context::SharedContext;
use crate::modules::link::graphql::Link;
use crate::shared::repository::Repository;

pub type LinksConnection = Connection<ID, Link, EmptyFields, EmptyFields>;

#[derive(Copy, Clone, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum UserErrorCode {
    EmailTaken,
    Unauthorized,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct AccessToken {
    pub access_token: String,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct UserError {
    pub code: UserErrorCode,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct User {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub links_ids: Vec<ID>,
}

impl User {
    async fn links(&self, ctx: &Context<'_>) -> Result<Vec<Link>, ()> {
        let context = ctx.data_unchecked::<SharedContext>();
        let links = self
            .links_ids
            .iter()
            .map(|link_id| {
                let link = context
                    .repositories
                    .link
                    .find_by_key(link_id.as_bytes())
                    .unwrap()
                    .unwrap();

                Link::from(link)
            })
            .collect::<Vec<Link>>();

        Ok(links)
    }
}
