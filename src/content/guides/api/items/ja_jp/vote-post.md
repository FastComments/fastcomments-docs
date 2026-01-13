[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

このルートは、単一の認可された `Vote` を追加する機能を提供します。投票は `up` (+1) または `down` (-1) にできます。

[inline-code-attrs-start title = '投票作成の cURL リクエスト例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '匿名投票作成の cURL リクエスト例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '投票作成リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = '投票作成レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** 失敗時に含まれます。 **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### 匿名投票の作成

匿名投票は、クエリパラメータで `userId` の代わりに `anonUserId` を設定することで作成できます。

このIDはどこかのユーザーオブジェクトに対応している必要はありません（したがって匿名です）。これは単にセッションの識別子であり、同じセッション内で再度投票を取得して、コメントが投票されているかを確認できます。

もし FastComments のような「匿名セッション」がない場合は、単純にランダムなID（UUIDなど）を設定できます（ただし、スペース節約のために短い識別子の方が好ましいです）。

### その他の注意点

- このAPIはテナントレベルの設定に従います。例えば、特定のページで投票を無効にしている場合、API経由で投票を作成しようとすると、エラーコード `voting-disabled` で失敗します。
- このAPIはデフォルトでライブです。
- このAPIは対応する `Comment` の `votes` を更新します。