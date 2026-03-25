## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| limit | number | Nej |  |
| skip | number | Nej |  |
| order | SORTDIR | Nej |  |
| after | number | Nej |  |
| before | number | Nej |  |

## Svar

Returnerer: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogs200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getAuditLogs Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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