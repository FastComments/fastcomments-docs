## Parameters

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| limit | f64 | Нет |  |
| skip | f64 | Нет |  |
| order | models::SortDir | Нет |  |
| after | f64 | Нет |  |
| before | f64 | Нет |  |

## Response

Возвращает: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_response.rs)

## Example

[inline-code-attrs-start title = 'Пример get_audit_logs'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params: GetAuditLogsParams = GetAuditLogsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        limit: Some(100.0),
        skip: Some(0.0),
        order: Some(models::SortDir::Desc),
        after: Some(1622505600.0),
        before: Some(1625097600.0),
    };
    let response: GetAuditLogsResponse = get_audit_logs(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---