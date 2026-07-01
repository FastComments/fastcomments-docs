## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| limit | number | Ne |  |
| skip | number | Ne |  |
| order | SORTDIR | Ne |  |
| after | number | Ne |  |
| before | number | Ne |  |

## Odgovor

Vraća: [`GetAuditLogsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogsResponse1.ts)

## Primer

[inline-code-attrs-start title = 'Primer getAuditLogs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_42";
  const limit: number = 100;
  const skip: number = 10;
  const order: SORTDIR = "desc";
  const after: number = Date.now() - 3 * 24 * 60 * 60 * 1000; // pre 3 dana

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

---