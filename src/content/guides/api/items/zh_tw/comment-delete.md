[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

此 API 端點提供刪除評論的功能。

注意事項：

- 如果需要，此 API 可以將評論小工具 "live" 即時更新（這會將 `creditsCost` 從 `1` 增加到 `2`）。
- 此 API 將刪除所有子評論。
- 如果目標評論已鎖定（`isLocked: true`），請求會以 `code: 'locked'` 被拒絕。請先解鎖該評論，然後再刪除。

[inline-code-attrs-start title = '評論 DELETE cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = '評論 DELETE 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** 執行更新的使用者。若需要，可用來檢查他們是否有權刪除該評論。  **/
    contextUserId?: string
	/** 是否應將評論「即時」刪除，讓正在查看具有相同 urlId 的評論小工具實例的使用者看到。注意：會將信用成本從 1 增加為 2。 **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = '評論 DELETE 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** 於失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** 於失敗時包含。 **/
    reason?: string
}
[inline-code-end]

---