---
## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| sso | String | No |  |

## Відповідь

Повертає: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_manual_badges_response.rs)

## Приклад

[inline-code-attrs-start title = 'get_manual_badges Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_badges(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetManualBadgesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        sso: Some("news/article".to_string()),
    };
    let _response = get_manual_badges(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---