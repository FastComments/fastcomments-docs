[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

1つの投票の集計の背後にある個々の投票を、古い順に一覧表示します。返される100票につき1クレジットです。

投票はコメントに属するため、投票は1つの投票ごとに読み取られ、`commentId` が必須です。`voterId` を使用して特定のユーザーの投票を確認したり、`optionId` を使用して特定の選択肢を選んだ全員を一覧表示したりして絞り込むことができます。

1回の呼び出しで返される投票は最大1000件です。さらに取得するには `skip` を使用してページングしてください。

投票の `privacy` 設定が尊重されます。匿名投票の投票は読み取れず、リクエストは `poll-anonymous` エラーで失敗します。詳細は `PollVote` 構造体をご参照ください。

[inline-code-attrs-start title = 'PollVotes 取得 cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes 取得 リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId: string
    voterId?: string
    optionId?: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes 取得 レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** 失敗時に含まれます。 **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### Counting Votes Per Option

結果を得るためにこれらを合計する必要はありません—投票は自分自身の集計を保持しています。代わりに `GET /api/v1/polls/:commentId` で投票を取得し、投票者を知りたいときにこの API を使用してください。

### Every Poll On A Page

ページ全体の投票一覧はありません。ページ全体をレポートするには、`GET /api/v1/comments` でコメントを取得します。これにより各コメントの投票とその集計が返され、必要な投票の投票を取得できます。

---