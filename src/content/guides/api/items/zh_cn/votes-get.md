[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

必须通过 `urlId` 来获取投票。

### 投票类型

投票有三种类型：

- 经过身份验证的投票，会应用到相应的评论。您可以通过此 API 创建这些投票。
- 经过身份验证的投票，但处于**待验证**状态，因此尚未应用于评论。当用户使用 FastComments.com 的 *登录以投票* 机制时，会创建这些投票。
- 匿名投票，会应用到相应的评论。这些与匿名评论一起创建。

为了减少混淆，API 会将它们分别返回在不同的列表中。

[inline-code-attrs-start title = 'Votes cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Votes 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Votes 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** 在失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** 在失败时包含。 **/
    reason?: string
    /** 已授权、已验证的投票，应用于其对应的评论。 **/
    appliedAuthorizedVotes: Vote[]
    /** 匿名投票，应用于其对应的评论。 **/
    appliedAnonymousVotes: Vote[]
    /** 待验证的投票，尚未应用于其对应的评论。 **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### 匿名投票说明

注意：通过此 API 创建的匿名投票将出现在 `appliedAuthorizedVotes` 列表中。因为它们是使用 API 密钥通过 API 创建的，所以被视为已授权。

`appliedAnonymousVotes` 结构用于没有电子邮件、API 密钥等的投票。

---