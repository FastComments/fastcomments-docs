## 參數

| 名稱 | 類型 | 必需 | 說明 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| limit | float64 | 否 |  |
| skip | float64 | 否 |  |
| order | SORTDIR | 否 |  |
| after | float64 | 否 |  |
| before | float64 | 否 |  |

## 回應

回傳: [`Option[GetAuditLogs_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_audit_logs200response.nim)

## 範例

[inline-code-attrs-start title = 'getAuditLogs 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getAuditLogs(
  tenantId = "my-tenant-123",
  limit = 100.0,
  skip = 0.0,
  order = SORTDIR(0),
  after = 0.0,
  before = 0.0
)
if response.isSome:
  let audit = response.get()
  echo audit
[inline-code-end]

---