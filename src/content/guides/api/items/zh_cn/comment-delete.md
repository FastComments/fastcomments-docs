[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

此 API 端点提供删除评论的功能。

Notes:

- 如果需要，此 API 可以实时更新评论小部件（这会将 `creditsCost` 从 `1` 提高到 `2`）。
- 此 API 将删除该评论的所有子评论。

[inline-code-attrs-start title = '评论 删除 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = '评论 删除 请求 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** 执行更新的用户。可用于检查他们是否有权限删除该评论。  **/
    contextUserId?: string
	/** 是否应对同一 urlId 的评论小部件的查看者进行“实时”删除。NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = '评论 删除 响应 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** 失败时包含。 **/
    reason?: string
}
[inline-code-end]