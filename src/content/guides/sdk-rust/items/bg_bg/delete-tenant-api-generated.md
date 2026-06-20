---
## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| sure | String | Не |  |

## Отговор

Връща: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за delete_tenant'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteTenantParams = DeleteTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "tenant-5f2d".to_string(),
        sure: Some("confirm".to_string()),
    };
    let response: ApiEmptyResponse = delete_tenant(&configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]

---