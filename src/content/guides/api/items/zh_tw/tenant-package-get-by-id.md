[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

此路由會根據 id 回傳單一 Tenant Package。

[inline-code-attrs-start title = 'TenantPackage 以 ID 的 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackageByIdQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackageByIdResponse {
    status: 'success' | 'failed'
    /** 僅在失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** 僅在失敗時包含。 **/
    reason?: string
    tenantPackage: TenantPackage
}
[inline-code-end]