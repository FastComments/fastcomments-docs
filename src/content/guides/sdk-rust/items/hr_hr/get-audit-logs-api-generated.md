---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| limit | f64 | Ne |  |
| skip | f64 | Ne |  |
| order | models::SortDir | Ne |  |
| after | f64 | Ne |  |
| before | f64 | Ne |  |

## Odgovor

Vraća: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_200_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_audit_logs Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetAuditLogsParams = GetAuditLogsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        limit: Some(100.0),
        skip: Some(0.0),
        order: Some(models::SortDir::Desc),
        after: Some(1672531200.0),
        before: Some(1675209600.0),
    };
    let response: GetAuditLogs200Response = get_audit_logs(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---