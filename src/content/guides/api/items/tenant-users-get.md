[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

This API uses pagination, provided by the `skip` query parameter. TenantUsers are returned in pages of `100`, ordered by `signUpDate`, `username` and `id`.

The cost is based on the number of tenant users returned, costing `1 credit per 10` tenant users returned.

[inline-code-attrs-start title = 'TenantUser cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string;
    API_KEY: string;
    /** The number of tenant users to skip for pagination. **/
    skip?: number;
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed';
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key';
    /** Included on failure. **/
    reason?: string;
    tenantUsers?: TenantUser[];
}
[inline-code-end]
