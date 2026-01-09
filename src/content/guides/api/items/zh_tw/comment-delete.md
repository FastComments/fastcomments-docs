[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

此 API 端點提供刪除留言的功能。

注意事項：

- 此 API 可以在需要時即時更新留言小工具（live）。這會將 `creditsCost` 從 `1` 提高到 `2`。
- 此 API 會刪除所有子留言。

[inline-code-attrs-start title = '留言 刪除 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = '留言 刪除 請求 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** 執行更新的使用者。若需要可用來檢查該使用者是否可以刪除該留言。  **/
    contextUserId?: string
	/** 是否應對正在檢視具有相同 urlId 的留言小工具實例的使用者即時（live）刪除該留言。NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = '留言 刪除 回應 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** 失敗時包含。 **/
    reason?: string
}
[inline-code-end]