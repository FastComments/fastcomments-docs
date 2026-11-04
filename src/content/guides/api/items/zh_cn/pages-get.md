[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

目前您只能获取与您的账户关联的所有页面（或通过 `/by-url-id` 获取单个页面）。如果您需要更精细的搜索功能，请[联系我们](https://fastcomments.com/auth/my-account/help)。

[inline-code-attrs-start title = 'Pages cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Pages 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Pages 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失败时包含。 **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### 实用提示

`Comment` API 需要一个 `urlId`。您可以先调用 `Pages` API，查看可用的 `urlId` 值是什么样的。