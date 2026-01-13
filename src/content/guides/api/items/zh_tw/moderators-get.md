[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

此 API 使用分頁，由 `skip` 查詢參數提供。管理員會以每頁 `100` 名的方式回傳，依據 `createdAt` 和 `id` 排序。

成本基於回傳的管理員數量計算，為每回傳 `1 credit per 10` 名管理員收費。

[inline-code-attrs-start title = '管理員 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = '管理員 請求 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 分頁時要跳過的管理員數量。 **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = '管理員 回應 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsResponse {
    status: 'success' | 'failed'
    /** 僅在失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 僅在失敗時會包含。 **/
    reason?: string
    moderators?: Moderator[]
}
[inline-code-end]