## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenant_id | String | Oui |  |
| limit | f64 | Non |  |
| skip | f64 | Non |  |
| order | models::SortDir | Non |  |
| after | f64 | Non |  |
| before | f64 | Non |  |

## Réponse

Retourne : [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_audit_logs'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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