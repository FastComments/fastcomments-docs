---
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

戻り値: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_200_response.rs)

---