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
async fn run_unpin() -> Result<PinComment200Response, Error> {
    let params: UnPinCommentParams = UnPinCommentParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("cmt-9f2b1a"),
        broadcast_id: String::from("news/article/2026/01/12/breaking"),
        sso: Some(String::from("user-9876-sso-token")),
    };
    let resp: PinComment200Response = un_pin_comment(&configuration, params).await?;
    Ok(resp)
}
[inline-code-end]
