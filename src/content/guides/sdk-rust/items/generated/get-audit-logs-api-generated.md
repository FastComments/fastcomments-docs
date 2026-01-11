## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| limit | f64 | No |  |
| skip | f64 | No |  |
| order | models::SortDir | No |  |
| after | f64 | No |  |
| before | f64 | No |  |

## Response

Returns: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_audit_logs Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_audit_logs() -> Result<GetAuditLogs200Response, Error> {
    let params: GetAuditLogsParams = GetAuditLogsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        limit: Some(100.0),
        skip: Some(0.0),
        order: Some(models::SortDir::Desc),
        after: Some(1696000000.0),
        before: Some(1699000000.0),
    };
    let response: GetAuditLogs200Response = get_audit_logs(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
