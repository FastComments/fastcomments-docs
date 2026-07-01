## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| options | GetAuditLogsOptions | Nein |  |

## Antwort

Rückgabe: [`Option[GetAuditLogsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_audit_logs_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getAuditLogs Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (auditOpt, httpResp) = client.getAuditLogs(tenantId = "my-tenant-123", options = GetAuditLogsOptions())
if auditOpt.isSome:
  let audit = auditOpt.get()
  echo audit
[inline-code-end]