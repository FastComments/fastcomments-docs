[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

目前您只能获取与您的账户关联的所有页面（或通过 `/by-url-id` 获取单个页面）。如果您需要更细粒度的搜索，请[联系我们](https://fastcomments.com/auth/my-account/help)。 

[inline-code-attrs-start title = '页面 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = '页面 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '页面 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

#### 有用提示

The `Comment` API requires a `urlId`. You can call the `Pages` API first, to see what the `urlId` values available to you
look like.

---