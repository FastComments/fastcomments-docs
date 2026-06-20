## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| redirect_url | String | Не |  |

## Одговор

Враћа: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Пример

[inline-code-attrs-start title = 'send_login_link Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn send_link_example() -> Result<(), Error> {
    let params: SendLoginLinkParams = SendLoginLinkParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-9876".to_string(),
        redirect_url: Some("https://acme.example.com/welcome".to_string()),
    };
    let response: ApiEmptyResponse = send_login_link(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---