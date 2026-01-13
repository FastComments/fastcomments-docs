[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

投票必須透過 `urlId` 取得。

### 投票類型

有三種類型的投票：

- 已認證的投票，會套用到相對應的留言。您可以透過此 API 建立這些投票。
- 待驗證的已認證投票，因此尚未套用到留言。當使用者使用 FastComments.com *login to vote* 機制時，會建立這些投票。
- 匿名投票，會套用到相對應的留言。這些會在匿名留言時一併建立。

為了減少混淆，API 會將這些投票分別回傳在不同的列表中。

[inline-code-attrs-start title = 'Votes cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = '投票請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = '投票回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** 失敗時會包含。 **/
    reason?: string
    /** 經授權、已驗證的投票，會套用到其相對應的留言。 **/
    appliedAuthorizedVotes: Vote[]
    /** 匿名投票，會套用到其相對應的留言。 **/
    appliedAnonymousVotes: Vote[]
    /** 待驗證的投票，尚未套用到其相對應的留言。 **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### 匿名投票注意事項

請注意：透過此 API 建立的匿名投票會出現在 `appliedAuthorizedVotes` 列表中。由於這些是使用 API 金鑰透過 API 建立的，因此被視為已授權。

`appliedAnonymousVotes` 結構則用於未提供電子郵件、API 金鑰等的投票。

---