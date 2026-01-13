[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

指定した `urlId` に対してユーザーが行った投票を取得できます。`userId` は FastComments.com のユーザーまたは `SSO User` を指定できます。

コメントに対してユーザーが投票したかどうかを表示したい場合に便利です。コメントを取得する際、単にユーザーのために同時にこの API を呼び出し、
同じ `urlId` を使用してください。

匿名投票を使用している場合は、代わりに `anonUserId` を渡してください。

[inline-code-attrs-start title = 'ユーザーの投票（cURL）例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = '匿名ユーザーの投票（cURL）例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

匿名の投票は `appliedAuthorizedVotes` リストに表示されます。API キーを使用して API 経由で作成されたため、承認済みとして扱われます。

[inline-code-attrs-start title = 'ユーザーの投票リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'ユーザーの投票レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** 失敗時に含まれます。 **/
    reason?: string
    /** 承認済みで検証済みの投票。対応するコメントに適用されます。 **/
    appliedAuthorizedVotes: Vote[]
    /** 検証待ちの投票。まだ対応するコメントには適用されていません。 **/
    pendingVotes: Vote[]
}
[inline-code-end]