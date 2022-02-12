[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

This API uses pagination, provided by the `skip` query parameter. TenantPackages are returned in pages of `100`, ordered by `createdAt` and `id`.

The cost is based on the number of tenant packages returned, costing `1 credit per 10` tenant packages returned.

[inline-code-attrs-start title = 'TenantPackage cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string;
    API_KEY: string;
    /** The number of tenant packages to skip for pagination. **/
    skip?: number;
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed';
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key';
    /** Included on failure. **/
    reason?: string;
    tenantPackages?: TenantPackage[];
}
[inline-code-end]
