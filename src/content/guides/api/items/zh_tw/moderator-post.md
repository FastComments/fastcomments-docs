[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

此路由允許新增單一個 `Moderator`。

建立 `Moderator` 有下列限制：

- 必須提供 `name` 和 `email`。`userId` 為選填。
- 建立 `Moderator` 時不得提供下列值：
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
- 當指定 `userId` 時，他們必須屬於在查詢參數中指定的相同 `tenantId`。
- 同一租戶中不能新增兩位具有相同 `email` 的 moderator。

我們可以為只有電子郵件資訊的使用者建立 `Moderator`：

[inline-code-attrs-start title = '透過 Email 建立 Moderator 的 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

或者，我們可以為屬於我們租戶的使用者建立 `Moderator`，以追蹤其審核統計資料：

[inline-code-attrs-start title = '透過租戶使用者建立 Moderator 的 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Moderator 建立請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator 建立回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** 失敗時會包含。 **/
    reason?: string
    moderator?: Moderator; // 成功時會回傳完整建立的 Moderator。
}
[inline-code-end]