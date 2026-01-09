[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

此 API 端點提供封鎖撰寫特定評論的使用者的功能。它支援封鎖來自 FastComments.com 使用者、SSO 使用者 和 租戶使用者 的評論。

它支援一個 `commentIdsToCheck` 主體參數，用來檢查在此操作完成後客戶端上是否有任何其他可能可見的評論應該被封鎖/解除封鎖。

注意：

- 此呼叫必須始終在使用者的上下文中進行。該使用者可以是 FastComments.com 使用者、SSO 使用者，或租戶使用者。
- 請求中的 `userId` 是正在「執行封鎖」的使用者。例如：`User A` 想要封鎖 `User B`。傳入 `userId=User A` 以及 `User B` 所撰寫的評論 id。
- 完全匿名的評論（沒有使用者 id、沒有電子郵件）無法被封鎖，系統將回傳錯誤。

[inline-code-attrs-start title = '評論封鎖 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

若要對匿名使用者進行封鎖，我們必須指定一個 `anonUserId`。這可以是代表匿名會話的 ID，或是一個隨機的 UUID。
這允許我們在使用者未登入時，透過使用相同的 `anonUserId` 來擷取評論並支援封鎖評論。

[inline-code-attrs-start title = '匿名評論封鎖 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '評論封鎖請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = '評論封鎖回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** 發生錯誤時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** 發生錯誤時包含。 **/
    reason?: string
    /** 如果定義了 commentIdsToCheck，則此映射中值為 true 的項目也會被封鎖。 **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---