## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| redirect_url | String | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'send_login_link Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn send_login_example() -> Result<FlagCommentPublic200Response, Error> {
    let params: SendLoginLinkParams = SendLoginLinkParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("user-98765"),
        redirect_url: Some(String::from("https://acme.example.com/dashboard")),
    };
    let response: FlagCommentPublic200Response = send_login_link(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---