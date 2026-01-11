## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| broadcast_id | String | Yes |  |
| sso | String | No |  |

## Response

Returns: [`PinComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pin_comment_200_response.rs)

## Example

[inline-code-attrs-start title = 'un_pin_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn perform_unpin(configuration: &configuration::Configuration) -> Result<PinComment200Response, Error> {
    let params: UnPinCommentParams = UnPinCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-8f3a2b".to_string(),
        broadcast_id: "news/article/2025/11/21".to_string(),
        sso: Some("sso-jwt-abc123".to_string()),
    };
    let response: PinComment200Response = un_pin_comment(configuration, params).await?;
    Ok(response)
}
[inline-code-end]
