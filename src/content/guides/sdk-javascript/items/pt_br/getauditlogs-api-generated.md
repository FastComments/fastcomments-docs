## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| limit | number | Não |  |
| skip | number | Não |  |
| order | SORTDIR | Não |  |
| after | number | Não |  |
| before | number | Não |  |

## Resposta

Retorna: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getAuditLogs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_87f9a4';
const limit: number = 100;
const skip: number = 0;
const order: SORTDIR = SORTDIR.DESC;
const after: number = Date.now() - 30 * 24 * 60 * 60 * 1000; // 30 dias atrás
const auditLogsResponse: GetAuditLogsResponse = await getAuditLogs(tenantId, limit, skip, order, after);
console.log((auditLogsResponse as unknown) ? 'Audit logs fetched' : 'No logs');
[inline-code-end]