## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| limit | number | No |  |
| skip | number | No |  |
| order | SORTDIR | No |  |
| after | number | No |  |
| before | number | No |  |

## Odgovor

Vraća: [`GetAuditLogsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogsResponse1.ts)

## Primer

[inline-code-attrs-start title = 'getAuditLogs Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_42";
  const limit: number = 100;
  const skip: number = 10;
  const order: SORTDIR = "desc";
  const after: number = Date.now() - 3 * 24 * 60 * 60 * 1000; // 3 days ago

  const auditResponse: GetAuditLogsResponse1 = await getAuditLogs(
    tenantId,
    limit,
    skip,
    order,
    after
  );

  console.log(auditResponse);
})();
[inline-code-end]