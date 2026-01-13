[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

此 API 端點可讓您為特定使用者標記（檢舉）評論。

注意事項：

- 此呼叫必須在使用者上下文中執行。該使用者可以是 FastComments.com 使用者、SSO 使用者或租戶使用者。
- 若已設定標記以隱藏的閾值，當評論被標記達到該次數時，將會即時自動隱藏該評論。
- 在評論被自動取消核准（隱藏）之後，該評論只能由管理員或版主重新核准。取消標記不會使評論重新核准。

[inline-code-attrs-start title = '評論檢舉 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

對於匿名檢舉，我們必須指定一個 `anonUserId`。這可以是代表匿名會話的 ID，或是一個隨機的 UUID。
這讓我們即使在使用者未登入時也能支援對評論的標記與取消標記。這樣一來，該評論可以被標示為
已標記，當以相同的 `anonUserId` 取回評論時。

[inline-code-attrs-start title = '匿名評論檢舉 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = '評論檢舉請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = '評論檢舉回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** 失敗時會包含。 **/
    reason?: string
    /** 該評論是否因被標記次數過多而被自動取消核准（隱藏）？ **/
    wasUnapproved?: boolean;
}
[inline-code-end]

---