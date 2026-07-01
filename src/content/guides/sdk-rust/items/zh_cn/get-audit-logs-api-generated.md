## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| limit | f64 | 否 |  |
| skip | f64 | 否 |  |
| order | models::SortDir | 否 |  |
| after | f64 | 否 |  |
| before | f64 | 否 |  |

## 响应

返回：[`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_response.rs)

## 示例

[inline-code-attrs-start title = 'get_audit_logs 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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