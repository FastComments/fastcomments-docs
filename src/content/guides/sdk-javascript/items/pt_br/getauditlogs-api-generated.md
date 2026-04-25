## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| limit | number | Não |  |
| skip | number | Não |  |
| order | SORTDIR | Não |  |
| after | number | Não |  |
| before | number | Não |  |

## Resposta

Retorna: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogs200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getAuditLogs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9a8b7c';
const limit: number = 100;
const skip: number = 0;
const after: number = Date.now() - 30 * 24 * 60 * 60 * 1000; // 30 dias atrás
const before: number = Date.now();
const auditLogs: GetAuditLogs200Response = await getAuditLogs(tenantId, limit, skip, undefined, after, before);
[inline-code-end]