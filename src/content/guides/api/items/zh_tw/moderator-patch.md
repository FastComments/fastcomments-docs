[api-resource-header-start name = 'Moderator'; route = 'PATCH /api/v1/moderators/:id'; creditsCost = 1; api-resource-header-end]

此 API 端點提供透過 `id` 更新 `Moderator` 的功能。

更新 `Moderator` 有以下限制：

- 更新 `Moderator` 時不得提供下列欄位：
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- 當指定 `userId` 時，該使用者必須存在。
- 當指定 `userId` 時，該使用者必須屬於查詢參數中指定的相同 `tenantId`。
- 在相同租戶中，兩名管理員不得使用相同的 `email`。
- 您不得變更與 `Moderator` 關聯的 `tenantId`。

[inline-code-attrs-start title = '管理員 PATCH cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/moderators/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = '管理員 PATCH 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '管理員 PATCH 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface ModeratorPatchResponse {
    status: 'success' | 'failed'
    /** 僅於失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found';  
    /** 僅於失敗時包含。 **/
    reason?: string
}
[inline-code-end]