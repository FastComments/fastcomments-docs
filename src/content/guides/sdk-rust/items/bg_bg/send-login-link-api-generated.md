## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| redirect_url | String | Не |  |

## Отговор

Връща: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за send_login_link'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_send_login_link() -> Result<FlagCommentPublic200Response, Error> {
    let params: SendLoginLinkParams = SendLoginLinkParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "jane.doe@acme.com".to_string(),
        redirect_url: Some("https://acme.example.com/dashboard".to_string()),
    };
    let response: FlagCommentPublic200Response = send_login_link(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---