[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

This API returns tenants that are managed by your tenant.

Pagination is provided by the `skip` query parameter. Tenants are returned in pages of `100`, ordered by `signUpDate`, and `id`.

The cost is based on the number of tenants returned, costing `1 credit per 10` tenants returned.

[inline-code-attrs-start title = 'Tenant cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenants to skip for pagination. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]
