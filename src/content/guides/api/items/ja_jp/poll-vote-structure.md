A `PollVote` は投票に対する1人の回答です。投票自体に表示されるカウントはこれと同期しているため、合計ではなく *誰が* 何に投票したかを知りたいときにだけ必要です。

投票者は1つの投票につき最大1票しか持てません。再度投票すると、2票目を追加するのではなく既存の票が新しいオプションに移動し、`updatedAt` がその時刻を記録します。

`voterId` は投票者がログインしていたときの `userId` で、そうでない場合は `anonUserId` です。

[inline-code-attrs-start title = 'PollVote 構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** 投票者がログインしていたときの userId、それ以外は anonUserId です。 **/
    voterId: string
    optionId: string
    createdAt: string
    /** 投票者が最後に投票を別のオプションに移した時刻。 **/
    updatedAt?: string
}
[inline-code-end]

### Privacy

投票の `privacy` 設定は、この API に対してコメントウィジェットでの動作と同様に適用されます：

- **Anonymous** (デフォルト): 誰がどのように投票したかは誰も見ることができないため、投票は読み取れません。`GET /api/v1/poll-votes` と `GET /api/v1/poll-votes/:id` は `poll-anonymous` を返します。投票のカウントは `GET /api/v1/polls/:commentId` から依然として取得可能です。
- **Admins and moderators**: API キーはサイトの管理者に属しているため、投票を読むことができます。
- **Everyone**: 誰でも投票を読むことができます。

投票が存在する場合、投票のプライバシーは狭めることはできますが、広げることはできません。