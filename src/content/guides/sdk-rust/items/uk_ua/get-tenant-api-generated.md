---
## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| id | String | Так |  |

## Відповідь

Повертає: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_tenant'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_tenant() -> Result<(), Error> {
    let params: GetTenantParams = GetTenantParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("news/article"),
    };
    let include_metadata: Option<bool> = Some(true);
    let tenant_response: GetTenant200Response = get_tenant(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---