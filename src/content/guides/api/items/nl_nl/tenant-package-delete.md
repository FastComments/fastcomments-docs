[api-resource-header-start name = 'TenantPackage'; route = 'DELETE /api/v1/tenant-packages/:id'; creditsCost = 5; api-resource-header-end]

Deze route verzorgt het verwijderen van een `TenantPackage` op basis van het id.

U mag geen `TenantPackage` verwijderen die in gebruik is (de `packageId` van een tenant verwijst naar het pakket). Werk eerst de `Tenant` bij.

[inline-code-attrs-start title = 'Voorbeeld cURL-verzoek voor TenantPackage-verwijdering'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van TenantPackage-verwijderingsverzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackageDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van TenantPackage-verwijderingsantwoord'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackageDeleteResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'package-in-use'
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]