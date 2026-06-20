## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| limit | float64 | Не |  |
| skip | float64 | Не |  |
| order | SORTDIR | Не |  |
| after | float64 | Не |  |
| before | float64 | Не |  |

## Одговор

Враћа: [`Option[GetAuditLogsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_audit_logs_response.nim)

## Пример

[inline-code-attrs-start title = 'getAuditLogs пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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