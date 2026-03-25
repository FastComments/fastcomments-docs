## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| un_block_from_comment_params | models::UnBlockFromCommentParams | Yes |  |
| user_id | String | No |  |
| anon_user_id | String | No |  |

## Response

Returns: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/un_block_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'un_block_user_from_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_unblock() -> Result<UnBlockCommentPublic200Response, Error> {
    let unblock_body: models::UnBlockFromCommentParams = models::UnBlockFromCommentParams {
        reason: Some(String::from("False positive - reviewed by moderator")),
        moderator_id: Some(String::from("moderator-42")),
    };
    let params: UnBlockUserFromCommentParams = UnBlockUserFromCommentParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("news/article/comments/abc123"),
        un_block_from_comment_params: unblock_body,
        user_id: Some(String::from("user-9876")),
        anon_user_id: Some(String::from("anon-5f4d")),
    };
    let result: UnBlockCommentPublic200Response = un_block_user_from_comment(&configuration, params).await?;
    Ok(result)
}
[inline-code-end]
