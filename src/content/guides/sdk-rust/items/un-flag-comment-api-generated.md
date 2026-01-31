## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| user_id | String | No |  |
| anon_user_id | String | No |  |

## Response

Returns: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_200_response.rs)

## Example

[inline-code-attrs-start title = 'un_flag_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_unflag() -> Result<FlagComment200Response, Error> {
    let params: UnFlagCommentParams = UnFlagCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/2026/launch-announcement#comment-241".to_string(),
        user_id: Some("user-9f3a".to_string()),
        anon_user_id: Some("anon-2b7c".to_string()),
    };
    let response: FlagComment200Response = un_flag_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
