## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetAuditLogsOptions | No |  |

## Одговор

Враћа: [`Option[GetAuditLogsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_audit_logs_response.nim)

## Пример

[inline-code-attrs-start title = 'getAuditLogs Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (auditOpt, httpResp) = client.getAuditLogs(tenantId = "my-tenant-123", options = GetAuditLogsOptions())
if auditOpt.isSome:
  let audit = auditOpt.get()
  echo audit
[inline-code-end]