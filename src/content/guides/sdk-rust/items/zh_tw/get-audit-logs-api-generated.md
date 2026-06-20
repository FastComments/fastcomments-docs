## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| limit | f64 | 否 |  |
| skip | f64 | 否 |  |
| order | models::SortDir | 否 |  |
| after | f64 | 否 |  |
| before | f64 | 否 |  |

## 回應

回傳: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_response.rs)

## 範例

[inline-code-attrs-start title = 'get_audit_logs 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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