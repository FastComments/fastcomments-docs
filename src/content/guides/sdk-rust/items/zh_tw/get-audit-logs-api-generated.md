## 參數

| 名稱 | 型別 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| limit | f64 | 否 |  |
| skip | f64 | 否 |  |
| order | models::SortDir | 否 |  |
| after | f64 | 否 |  |
| before | f64 | 否 |  |

## 回應

回傳: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_200_response.rs)

## 範例

[inline-code-attrs-start title = 'get_audit_logs 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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