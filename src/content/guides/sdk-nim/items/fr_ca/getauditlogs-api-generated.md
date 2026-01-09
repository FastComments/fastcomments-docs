---
## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| limit | float64 | Non |  |
| skip | float64 | Non |  |
| order | SORTDIR | Non |  |
| after | float64 | Non |  |
| before | float64 | Non |  |

## Réponse

Renvoie : [`Option[GetAuditLogs_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_audit_logs200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getAuditLogs'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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