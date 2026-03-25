## Parametri

| Name | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| limit | number | Ne |  |
| skip | number | Ne |  |
| order | SORTDIR | Ne |  |
| after | number | Ne |  |
| before | number | Ne |  |

## Odgovor

Vrne: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogs200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getAuditLogs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const tenantId: string = 'tenant_9b8f6c';
  const limit: number = 50;
  const skip: number = 0;
  const order: SORTDIR = 'desc';
  const after: number = Date.now() - 7 * 24 * 60 * 60 * 1000;
  const response: GetAuditLogs200Response = await getAuditLogs(tenantId, limit, skip, order, after);
  console.log(response);
}
main();
[inline-code-end]

---