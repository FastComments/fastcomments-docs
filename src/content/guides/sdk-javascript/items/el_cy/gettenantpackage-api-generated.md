## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackage200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4b8c2a9f';
const packageId: string = 'pkg_7d3e1b5c';
const includeMetadata: boolean | undefined = true;
const packageResponse: GetTenantPackage200Response = await getTenantPackage(tenantId, packageId);
[inline-code-end]