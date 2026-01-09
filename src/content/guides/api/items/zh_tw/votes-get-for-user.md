[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

允許取得使用者在指定 `urlId` 上留下的投票。接受一個 `userId`，該值可以是任何 FastComments.com 的使用者或 `SSO User`。

若你想顯示某使用者是否對留言投過票，這會很有用。當取得留言時，只要同時為該使用者對相同的 `urlId` 呼叫此 API 即可。

如果你使用匿名投票，則應傳入 `anonUserId`。

[inline-code-attrs-start title = '用戶投票 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = '匿名用戶投票 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

請注意匿名投票會出現在 `appliedAuthorizedVotes` 列表中。它們被視為已授權，因為是透過具有 API 金鑰的 API 建立的。

[inline-code-attrs-start title = '用戶投票請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = '用戶投票回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** 僅在失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** 僅在失敗時包含。 **/
    reason?: string
    /** 已授權、已驗證的投票，已套用到其對應的留言。 **/
    appliedAuthorizedVotes: Vote[]
    /** 待驗證的投票，尚未套用到其對應的留言。 **/
    pendingVotes: Vote[]
}
[inline-code-end]

---