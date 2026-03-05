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
async fn run() -> Result<FlagCommentPublic200Response, Error> {
    let updatable_comment_params: models::UpdatableCommentParams = models::UpdatableCommentParams {
        content: Some("We've updated the policy — please review the changes.".to_string()),
        author_name: Some("Jane Doe".to_string()),
        ..Default::default()
    };
    let params = UpdateCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/2026-01-12/comment-873".to_string(),
        updatable_comment_params,
        context_user_id: Some("user-9876".to_string()),
        do_spam_check: Some(true),
        is_live: Some(false),
    };
    let response: FlagCommentPublic200Response = update_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
