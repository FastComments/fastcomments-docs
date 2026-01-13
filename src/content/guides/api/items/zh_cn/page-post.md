[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

此 API 端点提供创建页面的功能。

一个常见的用例是访问控制。

注意：

- 如果您已经在评论线程中发表评论，或调用 API 创建了一个 `Comment`，您已经创建了一个 `Page` 对象！您可以尝试通过 `/by-url-id` `Page` 路由获取它，传入与评论小部件相同的 `urlId`。
- `Page` 结构包含一些 **计算得出** 的值。当前这些是 `commentCount` 和 `rootCommentCount`。它们会自动填充，不能由 API 设置。尝试设置这些值会导致 API 返回错误。

[inline-code-attrs-start title = '页面 POST cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = '页面 POST 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '页面 POST 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** 发生失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** 发生失败时包含。 **/
    reason?: string
    /** 已创建的页面。 **/
    page?: Page
}
[inline-code-end]