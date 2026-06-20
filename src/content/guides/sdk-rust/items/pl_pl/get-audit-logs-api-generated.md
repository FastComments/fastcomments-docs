## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| limit | f64 | Nie |  |
| skip | f64 | Nie |  |
| order | models::SortDir | Nie |  |
| after | f64 | Nie |  |
| before | f64 | Nie |  |

## Odpowiedź

Zwraca: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_audit_logs'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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