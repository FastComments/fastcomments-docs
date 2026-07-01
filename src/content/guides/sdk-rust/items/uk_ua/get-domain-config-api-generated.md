## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|--------------|------|
| tenant_id | String | Так |  |
| domain | String | Так |  |

## Відповідь

Повертає: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_domain_config_response.rs)

## Приклад

[inline-code-attrs-start title = 'get_domain_config Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let params = GetDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain: "news/article".to_string(),
        locale: Some("en-US".to_string()),
    };
    let _response = get_domain_config(&config, params).await?;
    Ok(())
}
[inline-code-end]