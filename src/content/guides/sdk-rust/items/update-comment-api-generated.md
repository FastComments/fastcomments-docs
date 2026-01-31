## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| updatable_comment_params | models::UpdatableCommentParams | Yes |  |
| context_user_id | String | No |  |
| do_spam_check | bool | No |  |
| is_live | bool | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'update_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_update() -> Result<FlagCommentPublic200Response, Error> {
    let params: UpdateCommentParams = UpdateCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-123/comment-456".to_string(),
        updatable_comment_params: models::UpdatableCommentParams {
            content: "Updated to correct the date and clarify the source attribution.".to_string(),
            author_display_name: Some("Ava Thompson".to_string()),
            pinned: Some(false),
        },
        context_user_id: Some("moderator-42".to_string()),
        do_spam_check: Some(true),
        is_live: Some(true),
    };
    let response: FlagCommentPublic200Response = update_comment(configuration, params).await?;
    Ok(response)
}
[inline-code-end]
