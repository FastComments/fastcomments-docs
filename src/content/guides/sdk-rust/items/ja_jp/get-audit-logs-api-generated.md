## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| limit | f64 | いいえ |  |
| skip | f64 | いいえ |  |
| order | models::SortDir | いいえ |  |
| after | f64 | いいえ |  |
| before | f64 | いいえ |  |

## レスポンス

戻り値: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_response.rs)

## 例

[inline-code-attrs-start title = 'get_audit_logs の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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