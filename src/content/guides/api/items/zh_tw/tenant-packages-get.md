[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

此 API 使用分頁，透過 `skip` 查詢參數提供。TenantPackages 以每頁 `100` 筆回傳，依 `createdAt` 與 `id` 排序。

費用依回傳的 TenantPackages 數量計算，成本為 `1 credit per 10` 回傳的 TenantPackages。

[inline-code-attrs-start title = 'TenantPackage cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 分頁時要跳過的 tenant packages 數量。 **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** 僅於失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 僅於失敗時包含。 **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]

---