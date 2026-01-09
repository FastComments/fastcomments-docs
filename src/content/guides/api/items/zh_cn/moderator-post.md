[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

该路由允许添加单个 `Moderator`。

创建 `Moderator` 有以下限制：

- 必须始终提供 `name` 和 `email`。`userId` 可选。
- 创建 `Moderator` 时不得提供以下值：
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- 当指定 `userId` 时，该用户必须存在。
- 当指定 `userId` 时，该用户必须属于查询参数中指定的相同 `tenantId`。
- 同一租户中不能添加具有相同 `email` 的两个 `Moderator`。

我们可以为仅知道电子邮件的用户创建 `Moderator`：

[inline-code-attrs-start title = 'Moderator 通过电子邮件创建的 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

或者，我们可以为属于我们租户的用户创建 `Moderator`，以跟踪他们的审核统计数据：

[inline-code-attrs-start title = 'Moderator 通过租户用户创建的 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "userId": "some-tenant-user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Moderator 创建请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator 创建响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** 失败时包含。 **/
    reason?: string
    moderator?: Moderator; // 成功时返回完整创建的 Moderator。
}
[inline-code-end]

---