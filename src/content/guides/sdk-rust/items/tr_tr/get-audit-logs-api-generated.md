---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| limit | f64 | Hayır |  |
| skip | f64 | Hayır |  |
| order | models::SortDir | Hayır |  |
| after | f64 | Hayır |  |
| before | f64 | Hayır |  |

## Yanıt

Döndürür: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_audit_logs Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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