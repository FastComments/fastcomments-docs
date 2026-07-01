## Parameters

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| options | GetAuditLogsOptions | Не |  |

## Response

Връща: [`Option[GetAuditLogsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_audit_logs_response.nim)

## Example

[inline-code-attrs-start title = 'Пример getAuditLogs'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (auditOpt, httpResp) = client.getAuditLogs(tenantId = "my-tenant-123", options = GetAuditLogsOptions())
if auditOpt.isSome:
  let audit = auditOpt.get()
  echo audit
[inline-code-end]