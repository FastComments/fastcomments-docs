## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| limit | float64 | Nej |  |
| skip | float64 | Nej |  |
| order | SORTDIR | Nej |  |
| after | float64 | Nej |  |
| before | float64 | Nej |  |

## Svar

Returnerer: [`Option[GetAuditLogs_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_audit_logs200response.nim)

## Eksempel

[inline-code-attrs-start title = 'getAuditLogs Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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