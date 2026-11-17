## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackageResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a5b1e4d4f3c2b1a0c9d8e7f';
const packageId: string = 'pkg_64f2c9e8a1b3d4';

const response: GetTenantPackageResponse = await getTenantPackage(tenantId, packageId);

console.log(`Status: ${response.status}`);
console.log(`Package plan: ${response.package.plan}`);
console.log(`Comments per month: ${response.package.commentsPerMonth}`);
[inline-code-end]