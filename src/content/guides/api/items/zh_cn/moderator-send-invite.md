[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

此路由提供邀请单个 `Moderator` 的功能。

向 `Moderator` 发送邀请邮件时存在以下限制：
- `Moderator` 必须已存在。
- `fromName` 长度不得超过 `100 characters`。

**注意：**
- 如果具有所提供电子邮件的用户已存在，他们将被邀请来管理您租户的评论。
- 如果具有所提供电子邮件的用户**不存在**，邀请链接将引导他们完成创建他们的账户的流程。
- 邀请将在 `30 days` 后过期。

我们可以仅凭电子邮件为用户创建一个 `Moderator`：

[inline-code-attrs-start title = '版主邀请 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

这将发送一封类似于 `Bob at TenantName is inviting you to be a moderator...` 的电子邮件

[inline-code-attrs-start title = '版主邀请 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** 发送给用户的电子邮件将显示为来自此名称。 **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = '版主邀请 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** 在失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** 在失败时包含。 **/
    reason?: string
}
[inline-code-end]

---