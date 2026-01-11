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

[inline-code-attrs-start title = 'flag_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<FlagComment200Response, Error> {
    let params: FlagCommentParams = FlagCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "cmt-987654321".to_string(),
        user_id: Some("user-42".to_string()),
        anon_user_id: None,
    };
    let response: FlagComment200Response = flag_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
