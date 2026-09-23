## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| limit | number | Non |  |
| skip | number | Non |  |
| order | SORTDIR | Non |  |
| after | number | Non |  |
| before | number | Non |  |
| username | string | Non |  |
| ip | string | Non |  |
| crudType | string | Non |  |
| resourceName | string | Non |  |
| targetId | string | Non |  |
| target | string | Non |  |
| includeManagedTenants | boolean | Non |  |

## Réponse

Retourne : [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getAuditLogs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchAuditLogs(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const limit: number = 50;
  const skip: number = 0;
  const order: SORTDIR = { direction: "DESC" };
  const after: number = Date.now() - 86400000; // il y a 1 jour
  const before: number = Date.now();
  const username: string = "alice.smith";
  const ip: string = "198.51.100.23";
  const crudType: string = "CREATE";
  const resourceName: string = "thread";
  const targetId: string = "thread_45678";
  const target: string = "forum_12";
  const includeManagedTenants: boolean = false;

  const logs: GetAuditLogsResponse = await getAuditLogs(
    tenantId,
    limit,
    skip,
    order,
    after,
    before,
    username,
    ip,
    crudType,
    resourceName,
    targetId,
    target,
    includeManagedTenants
  );

  console.log(logs);
}

fetchAuditLogs();
[inline-code-end]