[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

此 API 回傳由您的租戶所管理的租戶。

分頁由 `skip` 查詢參數提供。租戶以每頁 `100` 筆回傳，依照 `signUpDate` 與 `id` 排序。

費用依回傳的租戶數量計算，每回傳 10 位租戶收取 `1 credit`。

[inline-code-attrs-start title = 'Tenant cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

您可以在 `Tenant` 物件上定義 `meta` 參數並查詢符合條件的租戶。例如，對於鍵 `someKey` 與 meta 值 `some-value`，我們可以建立一個包含該鍵/值對的 JSON 物件，然後將其 URI 編碼作為查詢參數來進行篩選：

[inline-code-attrs-start title = 'Tenant 以 meta 查詢 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 用於分頁要跳過的租戶數量。 **/
    skip?: number
    /** 以 meta 參數過濾。 **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時會包含。 **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]