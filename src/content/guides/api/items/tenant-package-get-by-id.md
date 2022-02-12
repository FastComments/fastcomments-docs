[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

This route returns a single Tenant Package by id.

[inline-code-attrs-start title = 'TenantPackage By ID cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackageByIdQueryParams {
    tenantId: string;
    API_KEY: string;
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackageByIdResponse {
    status: 'success' | 'failed';
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found';
    /** Included on failure. **/
    reason?: string;
    tenantPackage: TenantPackage;
}
[inline-code-end]
