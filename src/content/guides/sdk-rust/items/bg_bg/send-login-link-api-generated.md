## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| redirect_url | String | Не |  |

## Отговор

Връща: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Пример

[inline-code-attrs-start title = 'send_login_link пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(config: &configuration::Configuration) -> Result<(), Error> {
    let params = SendLoginLinkParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
        redirect_url: Some("https://acme.com/after-login".to_string()),
    };
    send_login_link(config, params).await?;
    Ok(())
}
[inline-code-end]