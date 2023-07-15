use crate::{
    context::SharedContext,
    graphql::modules::post::types::{Post, PostError, PostErrorCode},
    services::auth::Token,
};

use async_graphql::{Context, Result, ID};
use gabble::post::service::CreatePostDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct PostCreateInput {
    pub author_id: ID,
    pub parent_id: Option<ID>,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PostCreate {
    post: Option<Post>,
    error: Option<PostError>,
}

impl PostCreate {
    pub async fn exec(ctx: &Context<'_>, input: PostCreateInput) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();

        if let Some(jwt) = ctx.data_opt::<Token>() {
            let claims = context.services.auth.verify_token(jwt).unwrap();
            let is_head = input.parent_id.is_some();
            let dto = CreatePostDto {
                author_id: claims.uid.to_string(),
                parent_id: input.parent_id.map(|id| id.to_string()),
                head: is_head,
                title: input.title,
                content: input.content,
            };

            match context.services.post.create(dto).await {
                Ok(post) => {
                    return Ok(Self {
                        post: Some(Post::from(post)),
                        error: None,
                    })
                }
                Err(err) => {
                    return Ok(Self {
                        post: None,
                        error: Some(PostError {
                            code: PostErrorCode::Unknown,
                            message: format!("An error ocurred: {err}", err = err),
                        }),
                    })
                }
            }
        }

        Ok(Self {
            post: None,
            error: Some(PostError {
                code: PostErrorCode::Unauthorized,
                message: String::from("Invalid token provided"),
            }),
        })
    }
}
