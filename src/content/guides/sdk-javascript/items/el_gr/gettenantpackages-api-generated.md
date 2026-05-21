## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| skip | number | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackages200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTenantPackages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-7b3c2f';
const skipCount: number = 10;
const packages: GetTenantPackages200Response = await getTenantPackages(tenantId, skipCount);
const packagesFromStart: GetTenantPackages200Response = await getTenantPackages(tenantId);
[inline-code-end]

---