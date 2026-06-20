## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| broadcast_id | String | Yes |  |
| sso | String | No |  |

## Response

Returns: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Example

[inline-code-attrs-start title = 'lock_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_lock_comment() -> Result<ApiEmptyResponse, Error> {
    let params: LockCommentParams = LockCommentParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        comment_id: "cmt-20240618-42".to_owned(),
        broadcast_id: "news/article/2024-06-18".to_owned(),
        sso: Some("user-12345-sso-token".to_owned()),
    };
    let response: ApiEmptyResponse = lock_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
