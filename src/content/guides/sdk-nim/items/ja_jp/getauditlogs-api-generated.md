## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| limit | float64 | いいえ |  |
| skip | float64 | いいえ |  |
| order | SORTDIR | いいえ |  |
| after | float64 | いいえ |  |
| before | float64 | いいえ |  |

## レスポンス

戻り値: [`Option[GetAuditLogs_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_audit_logs200response.nim)

## 例

[inline-code-attrs-start title = 'getAuditLogs の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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