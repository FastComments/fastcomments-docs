## 參數

| 名稱 | Type | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| limit | float64 | 否 |  |
| skip | float64 | 否 |  |
| order | SORTDIR | 否 |  |
| after | float64 | 否 |  |
| before | float64 | 否 |  |

## 回應

回傳: [`Option[GetAuditLogsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_audit_logs_response.nim)

## 範例

[inline-code-attrs-start title = 'getAuditLogs 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getAuditLogs(
  tenantId = "my-tenant-123",
  limit = 50.0,
  skip = 0.0,
  order = SORTDIR.DESC,
  after = 1622505600.0,
  before = 1625097600.0
)

if response.isSome:
  let logs = response.get()
  echo logs
else:
  echo "No audit logs returned"
[inline-code-end]

---