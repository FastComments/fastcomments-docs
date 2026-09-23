[api-resource-header-start name = 'Poll'; route = 'GET /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

コメントに付随する投票（ポール）を取得し、現在の投票数を返します。

コメントAPIでもコメント自体に投票情報が含まれます。そのため、コメント全体ではなく結果だけが必要な場合にこのエンドポイントを使用してください。

[inline-code-attrs-start title = 'Poll取得 cURL例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Poll取得リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Poll取得レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

投票が設定されていないコメント、削除されたコメント、存在しないコメントIDはすべて同じ応答となり、`poll-not-found` が返されます。