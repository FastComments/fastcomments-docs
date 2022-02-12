[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

This route returns a single Tenant by id.

[inline-code-attrs-start title = 'Tenant By ID cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantByIdQueryParams {
    tenantId: string;
    API_KEY: string;
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantByIdResponse {
    status: 'success' | 'failed';
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'unauthorized';
    /** Included on failure. **/
    reason?: string;
    tenant?: Tenant;
}
[inline-code-end]
