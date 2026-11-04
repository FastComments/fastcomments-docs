[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

投票の記録を行います。

投票者は1つの投票につき最大1票を持てます。同じ投票者に対して再度このAPIを呼び出すと、2票目を追加するのではなく、その投票を新しい選択肢に移動します。すでに選択した選択肢への投票は何もしません。

レスポンスには投票が含まれるため、2回目のリクエストなしで更新されたカウントを取得できます。

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

[inline-code-attrs-start title = 'PollVote 作成リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** エンドユーザーのIP。匿名のレート制限に使用されます。デフォルトは呼び出し元のIPです。 **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote 作成レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### 匿名投票

ログインしていないユーザーの投票を記録するには、`userId` の代わりに `anonUserId` を設定します。このIDはどこかのユーザーに対応している必要はなく、セッションを識別するだけのものなので、同じ人が二重にカウントされることはありません。

匿名投票はサイトで有効化されている必要があります。ログインユーザーに投票が制限されている場合、`anonUserId` のみの投票は `poll-login-required` で失敗します。

匿名投票は、セッションをクリアすることによる一つの投票への連投を防ぐため、IPごと・投票ごとにレート制限がかかります。エンドユーザーの `ip` を送信することで、その制限があなたのサーバーではなくエンドユーザーに適用されます。

### その他の注意事項

- `userId` はあなたのサイトに存在するユーザーである必要があります。他のサイトに属するユーザーへの投票は拒否されます。
- 終了した投票への投票は `poll-closed` で失敗します。
- このAPIは投票のカウントを更新し、接続されているウィジェットにリアルタイムで反映します。