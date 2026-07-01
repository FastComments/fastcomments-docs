## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| limit | f64 | Nej |  |
| skip | f64 | Nej |  |
| order | models::SortDir | Nej |  |
| after | f64 | Nej |  |
| before | f64 | Nej |  |

## Respons

Returnerer: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_response.rs)

## Eksempel

[inline-code-attrs-start title = 'get_audit_logs Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_audit_logs(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetAuditLogsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        limit: Some(100.0),
        skip: Some(0.0),
        order: Some(models::SortDir::Desc),
        after: Some(1622505600.0),
        before: None,
    };
    let _response: GetAuditLogsResponse = get_audit_logs(config, params).await?;
    Ok(())
}
[inline-code-end]

---