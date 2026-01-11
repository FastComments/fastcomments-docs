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

返回: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_audit_logs_200_response.rs)

---