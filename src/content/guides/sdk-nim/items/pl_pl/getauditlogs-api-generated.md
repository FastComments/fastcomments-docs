## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| limit | float64 | Nie |  |
| skip | float64 | Nie |  |
| order | SORTDIR | Nie |  |
| after | float64 | Nie |  |
| before | float64 | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetAuditLogsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_audit_logs_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getAuditLogs'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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