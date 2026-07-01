## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| limit | number | Non |  |
| skip | number | Non |  |
| order | SORTDIR | Non |  |
| after | number | Non |  |
| before | number | Non |  |

## Réponse

Renvoie : [`GetAuditLogsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogsResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getAuditLogs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_42";
  const limit: number = 100;
  const skip: number = 10;
  const order: SORTDIR = "desc";
  const after: number = Date.now() - 3 * 24 * 60 * 60 * 1000; // il y a 3 jours

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