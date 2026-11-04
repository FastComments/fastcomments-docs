[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

此路由提供邀請單一 `Moderator` 的功能。

傳送邀請電子郵件給 `Moderator` 時有以下限制：
- `Moderator` 必須已經存在。
- `fromName` 的長度不得超過 `100 characters`。

**注意事項：**
- 如果具有所提供電子郵件的使用者已存在，該使用者將被邀請來審核您租戶的留言。
- 如果具有所提供電子郵件的使用者**不存在**，邀請連結將引導他們完成帳號建立流程。
- 邀請將於 `30 days` 後失效。

我們可以為只知道電子郵件的使用者建立 `Moderator`：

[inline-code-attrs-start title = 'Moderator 邀請 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

這將會發送一封類似 `Bob at TenantName is inviting you to be a moderator...` 的電子郵件。

[inline-code-attrs-start title = 'Moderator 邀請請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** 發送給使用者的電子郵件將以此名稱作為寄件者顯示。 **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator 邀請回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** 失敗時包含此欄位。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** 失敗時包含此欄位。 **/
    reason?: string
}
[inline-code-end]