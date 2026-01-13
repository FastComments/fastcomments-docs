[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

投票は `urlId` によって取得する必要があります。

### 投票の種類

投票には3種類あります。

- 認証済み投票は対応するコメントに適用されます。これらはこのAPIを通じて作成できます。
- 検証が**保留中**の認証済み投票であり、そのためまだコメントに適用されていません。これはユーザーが FastComments.com の *ログインして投票* 機能を使用したときに作成されます。
- 匿名投票は対応するコメントに適用されます。これらは匿名コメントとともに作成されます。

これらは混乱を避けるためにAPIでは別々のリストで返されます。

[inline-code-attrs-start title = '投票の cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = '投票リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = '投票レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** 失敗時に含まれます。 **/
    reason?: string
    /** 認証済みで検証済みの投票。対応するコメントに適用されます。 **/
    appliedAuthorizedVotes: Vote[]
    /** 匿名投票。対応するコメントに適用されます。 **/
    appliedAnonymousVotes: Vote[]
    /** 検証が保留中の投票。まだ対応するコメントに適用されていません。 **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### 匿名投票の注意点

このAPIを通じて作成された匿名投票は `appliedAuthorizedVotes` リストに表示されることに注意してください。これらはAPIで API key を使って作成されたため、承認済みと見なされます。

`appliedAnonymousVotes` は、メールや API key などがない状態で作成された投票用の構造です。