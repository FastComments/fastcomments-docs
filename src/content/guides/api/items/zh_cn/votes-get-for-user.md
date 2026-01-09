[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

允许获取用户在给定 `urlId` 上留下的投票。需要一个 `userId`，可以是任何 FastComments.com 用户或 `SSO User`。

如果你想显示用户是否对评论投过票，这很有用。在获取评论时，只需同时为该用户对相同的 `urlId` 调用此 API。

如果你使用匿名投票，则应改为传递 `anonUserId`。

[inline-code-attrs-start title = '针对用户的投票 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = '匿名用户的投票 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

注意，匿名投票将出现在 `appliedAuthorizedVotes` 列表中。它们被视为已授权，因为它们是通过带有 API 密钥的 API 创建的。

[inline-code-attrs-start title = '针对用户的投票 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = '针对用户的投票 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** 失败时包含。 **/
    reason?: string
    /** 已授权、已验证的投票，已应用到其对应的评论。 **/
    appliedAuthorizedVotes: Vote[]
    /** 等待验证的投票，尚未应用到其对应的评论。 **/
    pendingVotes: Vote[]
}
[inline-code-end]