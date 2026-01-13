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

回傳: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_200_response.rs)

---