## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| limit | float64 | Ні |  |
| skip | float64 | Ні |  |
| order | SORTDIR | Ні |  |
| after | float64 | Ні |  |
| before | float64 | Ні |  |

## Відповідь

Повертає: [`Option[GetAuditLogs_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_audit_logs200response.nim)

## Приклад

[inline-code-attrs-start title = 'getAuditLogs Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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