`Poll` は独立したオブジェクトではなく、コメントに紐付けられます。コメントと同時に作成されるか( `POST /api/v1/comments` を参照)、後から既存のコメントに `PUT /api/v1/polls/:commentId` を使って追加されます。

投票数はポール自体に保持されるため、ポールを読み取るだけで、何かを合計することなく結果を取得できます。これらの投票数の背後にある個々の投票は `PollVote` オブジェクトです。

各選択肢には、ポールの作成時に生成される `id` があります。この id は、投票を行う、選択肢のラベルを変更する、そして選択肢を追加または削除してポールを `PUT` する際に選択肢(とその投票)を保持するために使用します。選択肢を参照する唯一の安全な方法はこの id であり、リスト内の位置では決してありません。

[inline-code-attrs-start title = 'ポールの構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** 設定されていて過去の日時である場合、ポールは終了しており、それ以上の投票は受け付けません。 **/
    closesAt?: string | null
    /** 0 は匿名(デフォルト)、1 は管理者とモデレーター、2 は全員。存在しない場合は匿名を意味します。 **/
    privacy?: 0 | 1 | 2 | null
    /** true の場合、まだ投票していない人には投票数が表示されません。存在しない場合は false を意味します。 **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### 制限

- 質問は必須で、最大 200 文字です。
- ポールには 2 から 10 個の選択肢が必要です。
- 選択肢のラベルは必須で、最大 100 文字、かつポール内で一意である必要があります(大文字小文字は区別しません)。
- `closesAt` はポール作成時に未来の日時である必要があります。ポールを即座に終了させるには、過去の日時で `PATCH` してください。

### サイト設定

ポールはサイトの設定に従います。設定は Customize Widget から変更できます:

- ポールを作成するには、ポール機能が有効化されている必要があります。そうでない場合、API は `polls-disabled` を返します。
- 投票をログイン済みユーザーに限定できます。その場合、`anonUserId` のみで送信された投票は `poll-login-required` で拒否されます。