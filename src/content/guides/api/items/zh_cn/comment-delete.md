[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

此 API 端点提供删除评论的功能。

注意事项：

- 如果需要，此 API 可以“实时”更新评论小部件（这会将 `creditsCost` 从 `1` 增加到 `2`）。
- 此 API 会删除所有子评论。
- 如果目标评论被锁定（`isLocked: true`），请求将被拒绝并返回 `code: 'locked'`。请先解锁评论，然后再删除。

[inline-code-attrs-start title = 'Comment DELETE cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Comment DELETE 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** 执行更新的用户。可用于校验其是否可以删除该评论。  **/
    contextUserId?: string
	/** 是否应对正在查看具有相同 urlId 的评论小部件实例的用户“实时”删除评论。注意：会将信用点消耗从 1 翻倍为 2。 **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment DELETE 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** 失败时包含。 **/
    reason?: string
}
[inline-code-end]

---