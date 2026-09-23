[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

既存のコメントに投票を添付するか、すでにある投票の全状態を設定します。

リクエストボディは完全な投票であり、送信したオプションはその順序で投票のオプションになります。各オプションはその `id` で照合されます。

- 既存のオプションの `id` を持つオプションを送信すると、そのオプションと投票が保持されます。ラベルと位置は送信した内容に更新されます。
- `id` がないオプションを送信すると、投票なしで追加されます。
- 省略した既存のオプションは削除され、そのオプションに投じられた投票も削除されます。`totalVotes` は同じ数だけ減少します。

オプションを追加するには、現在のオプションとその id を送信し、id のない新しいオプションを追加します。削除するには、そのオプションをリストから除外して送信します。オプションの id は `GET /api/v1/polls/:commentId` で返される投票に含まれています。

すべての id を送信しない場合、すべてのオプションが置き換えられ、投票済みの投票はすべて削除されます。投票がある場合は `replaceVotes=true` が必要で、これがないと API は `replace-votes-required` で応答します。

他のフィールドも置き換えられます。`closesAt`、`privacy`、`requireVoteToSeeResults` を省略するとデフォルトにリセットされます。単一のフィールドだけを変更し、他をそのままにしたい場合は `PATCH /api/v1/polls/:commentId` を使用してください。

[inline-code-attrs-start title = '投票作成 cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [
		{"id": "existing-option-id", "label": "The bugfix release"},
		{"label": "The feature release"}
	],
	"closesAt": "2026-12-31T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = '投票作成 リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** 投票に投票がある場合、既存のオプション ID をすべて保持しないために必要です。これを行わないとすべて削除されます。 **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** 既存のオプションの ID。これを保持するとそのオプションと投票が保持されます。省略すると新しいオプションが追加されます。 **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** 完全な順序付きリスト。省略された既存のオプションは投票とともに削除されます。 **/
    options: PollPutOption[]
    /** コメントにまだ投票がない場合、将来の日付である必要があります。オープンしたままの投票の場合は省略してください。 **/
    closesAt?: string | null
    /** 0: 匿名（デフォルト）、1: 管理者とモデレーター、2: 全員。 **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = '投票作成 応答構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** 失敗時に含まれます。 **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### その他の注意点

- 投票に存在しない `id`、または同じ `id` を2回指定した場合は `poll-invalid` エラーになります。投票がないコメントにはオプション ID がまだないため、送信するすべてのオプションは `id` を省略する必要があります。
- 投票がある場合、投票のプライバシーは狭めることはできますが、広げることはできません。
- この API はサイト設定に従います。サイトまたはページで投票が有効になっていない場合、`polls-disabled` エラーになります。
- ロックされたコメントの投票は変更できず、`locked` エラーになります。
- 接続されたウィジェットはリアルタイムで更新されるため、閲覧者はページをリロードせずに新しい投票を見ることができます。