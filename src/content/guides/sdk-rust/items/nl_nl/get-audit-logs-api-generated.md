## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| limit | f64 | Nee |  |
| skip | f64 | Nee |  |
| order | models::SortDir | Nee |  |
| after | f64 | Nee |  |
| before | f64 | Nee |  |

## Respons

Retourneert: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_200_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_audit_logs Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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