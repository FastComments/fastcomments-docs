## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenant_id | String | Так |  |
| id | String | Так |  |
| update_tenant_body | models::UpdateTenantBody | Так |  |

## Відповідь

Повертає: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Приклад

[inline-code-attrs-start title = 'update_tenant Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = UpdateTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "tenant-001".to_string(),
        update_tenant_body: UpdateTenantBody {
            description: Some("Primary tenant for Acme Corp".to_string()),
            ..Default::default()
        },
    };
    let _ = update_tenant(&configuration, params).await?;
    Ok(())
}
[inline-code-end]