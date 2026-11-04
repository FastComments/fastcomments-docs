[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

1つの投票（poll）の集計数の背後にある個々の投票（vote）を、古いものから順に一覧表示します。返される100票ごとに1クレジットが消費されます。

投票（poll）はコメントに紐づいているため、投票（vote）は1つの投票（poll）ずつ読み取ることになり、`commentId` が必須です。`voterId` を指定すると特定の人がどう投票したかを確認でき、`optionId` を指定すると特定の選択肢を選んだ全員を一覧表示できます。

1回の呼び出しで最大1000票まで返されます。それ以上は `skip` を使ってページ送りしてください。

投票（poll）の `privacy` 設定は尊重されます：匿名投票（poll）の投票内容は読み取ることができず、リクエストは `poll-anonymous` エラーで失敗します。詳細は `PollVote` 構造体を参照してください。

[inline-code-attrs-start title = 'PollVotes Get cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'PollVotes Get レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### 選択肢ごとの票数の集計

結果を得るためにこれらを合計する必要はありません - 投票（poll）自体が集計数を持っています。代わりに `GET /api/v1/polls/:commentId` で投票（poll）を読み取り、誰が投票したかを知る必要がある場合にこのAPIを使用してください。

### ページ上のすべての投票

ページ全体の投票一覧を取得する機能はありません。ページ全体についてレポートするには、`GET /api/v1/comments` でそのページのコメントを取得します（各コメントの投票（poll）とその集計数が返されます）。その後、必要な投票（poll）の投票（vote）を読み取ってください。