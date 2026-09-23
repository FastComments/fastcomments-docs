`Poll` は独立したオブジェクトではなく、コメントに付随します。コメントと同時に作成されます（`POST /api/v1/comments` を参照）、または後で既存のコメントに `PUT /api/v1/polls/:commentId` で追加できます。

投票数は投票自体に保持されるため、投票を取得するだけで結果が得られ、合計を計算する必要はありません。そのカウントの背後にある個々の投票は `PollVote` オブジェクトです。

各オプションには、投票作成時に生成される `id` が付与されます。この id を使用して投票を行ったり、オプションのラベルを変更したり、オプション（およびその投票）を保持したりします（オプションを追加または削除して `PUT` する場合）。オプションを参照する唯一の安全な方法はこの id であり、リスト内の位置で参照してはいけません。

[inline-code-attrs-start title = '投票構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPollOption {
    id: string
    label: string
    votes: number
}

interface CommentPoll {
    question: string
    options: CommentPollOption[]
    totalVotes: number
    /** 設定されて過去の場合、投票は締め切られ、これ以上投票を受け付けません。 **/
    closesAt?: string | null
    /** 0 は匿名（デフォルト）、1 は管理者とモデレーター、2 は全員。未設定の場合は匿名です。 **/
    privacy?: 0 | 1 | 2 | null
    /** true の場合、投票していないユーザーには結果が非表示になります。未設定の場合は false です。 **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Limits

- 質問は必須で、最大200文字です。
- 投票は2〜10個のオプションを持ちます。
- オプションのラベルは必須で、最大100文字、投票内で（大文字小文字を区別せず）一意である必要があります。
- `closesAt` は投票作成時に未来の日付でなければなりません。投票を即座に締め切るには、過去の日付で `PATCH` してください。

### Site Settings

投票はサイト設定に従います。設定は「Customize Widget」から変更できます：

- 投票を作成する前に投票機能を有効にする必要があります。無効な場合、APIは `polls-disabled` を返します。
- 投票はログインユーザーのみに制限できます。その場合、`anonUserId` のみが含まれる投票は `poll-login-required` で拒否されます。