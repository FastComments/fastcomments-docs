[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

投票の投票数を変更せずに投票を編集します。質問や選択肢の誤字を修正したり、投票を閉じたり再開したり、投票者が誰かを見ることができるかを変更したりするために使用します。

オプションはその `id` で指定され、`PATCH` は指定したオプションのラベルを変更します。オプションを追加、削除、または順序変更するには、`PUT /api/v1/polls/:commentId` に完全なオプションリストを送信してください。`id` を含めて送信したオプションは投票も保持されます。

すべてのフィールドは任意ですが、少なくとも1つは指定する必要があります。

[inline-code-attrs-start title = '投票パッチ cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = '投票を今すぐ閉じる cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = '投票パッチ リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** 既存のオプションのラベルを変更します。指定されたすべての id はすでに投票に存在している必要があります。 **/
    options?: { id: string, label: string }[] | null
    /** 過去の日付は投票を即座に閉じます。null は閉じた投票を再開します。 **/
    closesAt?: string | null
    /** 0 は匿名、1 は管理者とモデレーター、2 は全員。 **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = '投票パッチ 応答構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Other Notes

- 投票に存在しないオプション id を指定すると、何もしないで静かに失敗するのではなく、`poll-invalid` エラーで失敗します。
- ラベルは投票内で一意である必要があり、変更しないオプションも含めて重複してはいけません。
- 投票の作成とは異なり、ここでは `closesAt` を過去の日時に設定できます。これが投票を即座に閉じる方法です。
- 投票に投票がある場合、プライバシー設定は狭めることはできますが、広げることはできません。
- ロックされたコメントの投票は変更できず、`locked` エラーで失敗します。