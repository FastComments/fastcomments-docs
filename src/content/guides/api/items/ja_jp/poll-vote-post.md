[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

投票を記録します。

投票者は各投票で最大1票しか持てません。同じ投票者に対して再度呼び出すと、2票目を追加するのではなく、投票が新しいオプションに移動します。また、すでに選択したオプションに投票しても何も起こりません。

レスポンスには投票が含まれるため、2回目のリクエストを行わずに更新されたカウントを取得できます。

[inline-code-attrs-start title = 'PollVote 作成 cURL 例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"userId": "user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = '匿名 PollVote 作成 cURL 例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"anonUserId": "some-randomly-generated-identifier",
	"ip": "203.0.113.4"
}'
[inline-code-end]

[inline-code-attrs-start title = 'PollVote 作成 リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** userId または anonUserId のいずれかが必須です。 **/
    userId?: string
    anonUserId?: string
    /** エンドユーザーの IP。匿名レートリミットに使用されます。デフォルトは呼び出し元の IP です。 **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote 作成 レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** 失敗時に含まれます。 **/
    reason?: string
    pollVote: PollVote
    /** 更新されたカウントを含む投票。 **/
    poll: CommentPoll
}
[inline-code-end]

### Anonymous Votes

`anonUserId` を `userId` の代わりに設定して、ログインしていないユーザーの投票を記録します。その ID はどこかのユーザーに対応している必要はなく、セッションを識別するだけなので、同一人物が二重にカウントされることはありません。

匿名投票はサイトで有効にする必要があります。投票がログインユーザーに限定されている場合、`anonUserId` のみを使用した投票は `poll-login-required` エラーで失敗します。

匿名投票は投票ごとに IP 単位でレート制限されます。これにより、1人がセッションをクリアして投票を大量に行うことを防止します。エンドユーザーの `ip` を送信すると、制限がサーバーではなくそのユーザーに適用されます。

### Other Notes

- `userId` はサイト上に存在するユーザーである必要があります。他のサイトに属するユーザーへの投票は拒否されます。
- 閉じられた投票に対する投票は `poll-closed` エラーで失敗します。
- この API は投票のカウントを更新し、接続されたウィジェットにリアルタイムでプッシュします。

---