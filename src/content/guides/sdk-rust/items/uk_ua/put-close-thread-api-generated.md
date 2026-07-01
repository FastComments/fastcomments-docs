## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| url_id | String | Так |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Приклад

[inline-code-attrs-start title = 'put_close_thread Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let params = PutCloseThreadParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article-123".to_string(),
        sso: Some("sso-token-abc".to_string()),
    };
    put_close_thread(&config, params).await?;
    Ok(())
}
[inline-code-end]