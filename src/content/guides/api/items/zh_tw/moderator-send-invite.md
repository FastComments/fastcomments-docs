[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

此路由提供邀請單一位 `Moderator` 的功能。

The following restrictions exist to send an invite email to a `Moderator`:
- `Moderator` 必須已經存在。
- `fromName` 長度不得超過 `100 characters`。

**Notes:**
- 如果具有該電子郵件的使用者已存在，他們將會被邀請來管理您租戶的評論。
- 如果具有該電子郵件的使用者 **不存在**，邀請連結會引導他們建立帳戶。
- 邀請將在 `30 days` 後到期。

我們可以為只知道電子郵件的使用者建立 `Moderator`：

[inline-code-attrs-start title = '版主邀請 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

This will send an email like `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = '版主邀請請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** 寄送給使用者的電子郵件將顯示為由此名稱發出。 **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = '版主邀請回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** 失敗時會包含。 **/
    reason?: string
}
[inline-code-end]

---