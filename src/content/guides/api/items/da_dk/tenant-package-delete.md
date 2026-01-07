[api-resource-header-start name = 'TenantPackage'; route = 'DELETE /api/v1/tenant-packages/:id'; creditsCost = 5; api-resource-header-end]

Denne rute giver mulighed for fjernelse af en `TenantPackage` efter id.

Du kan ikke fjerne en `TenantPackage`, der er i brug (en tenants `packageId` peger på pakken). Opdater `Tenant` først.

[inline-code-attrs-start title = 'TenantPackage Fjernelse cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Fjernelse Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackageDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Fjernelse Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackageDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'package-in-use'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
