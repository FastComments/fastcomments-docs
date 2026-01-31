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

[inline-code-attrs-start title = 'pin_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<PinComment200Response, Error> {
    let params: PinCommentParams = PinCommentParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("cmt-12345"),
        broadcast_id: String::from("news/article/2026/01/13"),
        sso: Some(String::from("sso_user_98765")),
    };
    let pinned: PinComment200Response = pin_comment(&configuration, params).await?;
    Ok(pinned)
}
[inline-code-end]
