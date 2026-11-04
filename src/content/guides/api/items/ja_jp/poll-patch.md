[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

投票に影響を与えずに投票(ポール)を編集します。質問や選択肢の誤字を修正したり、投票を締め切ったり再開したり、誰が投票したかを誰に見せるかを変更する場合に使用します。

選択肢はその `id` で指定され、`PATCH` は指定されたもののラベルを付け替えます。選択肢の追加、削除、並べ替えを行うには、選択肢の全リストを `PUT /api/v1/polls/:commentId` に送信してください。id 付きで送信された選択肢は、そこでも投票を保持します。

すべてのフィールドは省略可能ですが、少なくとも1つは指定する必要があります。

[inline-code-attrs-start title = 'ポール PATCH の cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = '投票を今すぐ締め切る cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'ポール PATCH リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** 既存の選択肢のラベルを付け替えます。指定したすべての id が投票に存在している必要があります。 **/
    options?: { id: string, label: string }[] | null
    /** 過去の日付を指定すると投票は即座に締め切られます。null を指定すると締め切られた投票が再開されます。 **/
    closesAt?: string | null
    /** 0 は匿名ユーザー、1 は管理者とモデレーター、2 は全員。 **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'ポール PATCH レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** 失敗時に含まれます。 **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### その他の注意事項

- 投票に存在しない選択肢の id を指定すると、何もせずに終了するのではなく、`poll-invalid` エラーで失敗します。
- ラベルは、変更しない選択肢を含めて、投票内で一意である必要があります。
- 投票の作成時とは異なり、ここでは `closesAt` に過去の日付を指定できます。これが投票を即座に締め切る方法です。
- 投票のプライバシーは、投票が一度行われると狭めることはできますが、広げることはできません。
- ロックされたコメントの投票は変更できず、`locked` エラーで失敗します。