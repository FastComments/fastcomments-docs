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
let params: UpdateCommentParams = UpdateCommentParams {
    tenant_id: "acme-news-tenant".to_string(),
    id: "cmt-20251121-0001".to_string(),
    updatable_comment_params: models::UpdatableCommentParams {
        content: "Updated: I found additional sources that change my view on this.".to_string(),
        author_id: Some("user-9876".to_string()),
        tags: Some(vec!["correction".to_string(), "news".to_string()]),
        ..Default::default()
    },
    context_user_id: Some("moderator-42".to_string()),
    do_spam_check: Some(true),
    is_live: Some(true),
};
let response: FlagCommentPublic200Response = update_comment(&configuration, params).await?;
[inline-code-end]
