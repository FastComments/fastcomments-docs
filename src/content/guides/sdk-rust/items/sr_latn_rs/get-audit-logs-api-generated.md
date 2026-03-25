## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| limit | f64 | No |  |
| skip | f64 | No |  |
| order | models::SortDir | No |  |
| after | f64 | No |  |
| before | f64 | No |  |

## Odgovor

Vraća: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_200_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_audit_logs'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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