## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| limit | number | Nein |  |
| skip | number | Nein |  |
| order | SORTDIR | Nein |  |
| after | number | Nein |  |
| before | number | Nein |  |

## Antwort

Gibt zurück: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogs200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getAuditLogs Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9a8b7c';
const limit: number = 100;
const skip: number = 0;
const after: number = Date.now() - 30 * 24 * 60 * 60 * 1000; // vor 30 Tagen
const before: number = Date.now();
const auditLogs: GetAuditLogs200Response = await getAuditLogs(tenantId, limit, skip, undefined, after, before);
[inline-code-end]

---