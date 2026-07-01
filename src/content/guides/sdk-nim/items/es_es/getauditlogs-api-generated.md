## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| options | GetAuditLogsOptions | No |  |

## Respuesta

Devuelve: [`Option[GetAuditLogsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_audit_logs_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getAuditLogs'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (auditOpt, httpResp) = client.getAuditLogs(tenantId = "my-tenant-123", options = GetAuditLogsOptions())
if auditOpt.isSome:
  let audit = auditOpt.get()
  echo audit
[inline-code-end]