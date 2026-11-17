## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |

## Απάντηση

Επιστρέφει: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackageResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a5b1e4d4f3c2b1a0c9d8e7f';
const packageId: string = 'pkg_64f2c9e8a1b3d4';

const response: GetTenantPackageResponse = await getTenantPackage(tenantId, packageId);

console.log(`Status: ${response.status}`);
console.log(`Package plan: ${response.package.plan}`);
console.log(`Comments per month: ${response.package.commentsPerMonth}`);
[inline-code-end]