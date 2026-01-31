## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| redirect_url | String | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'send_login_link Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn send_login_link_example() -> Result<FlagCommentPublic200Response, Error> {
    let params: SendLoginLinkParams = SendLoginLinkParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("john.doe@acme.com"),
        redirect_url: Some(String::from("https://acme.example.com/dashboard")),
    };
    let response: FlagCommentPublic200Response = send_login_link(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
