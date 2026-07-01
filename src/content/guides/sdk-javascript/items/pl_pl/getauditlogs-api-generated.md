## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| limit | number | Nie |  |
| skip | number | Nie |  |
| order | SORTDIR | Nie |  |
| after | number | Nie |  |
| before | number | Nie |  |

## Odpowiedź

Zwraca: [`GetAuditLogsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogsResponse1.ts)

## Przykład

[inline-code-attrs-start title = 'getAuditLogs Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_42";
  const limit: number = 100;
  const skip: number = 10;
  const order: SORTDIR = "desc";
  const after: number = Date.now() - 3 * 24 * 60 * 60 * 1000; // 3 dni temu

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