## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| options | GetAuditLogsOptions | Ne |  |

## Odgovor

Vraća: [`Option[GetAuditLogsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_audit_logs_response.nim)

## Primjer

[inline-code-attrs-start title = 'getAuditLogs Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (auditOpt, httpResp) = client.getAuditLogs(tenantId = "my-tenant-123", options = GetAuditLogsOptions())
if auditOpt.isSome:
  let audit = auditOpt.get()
  echo audit
[inline-code-end]

---