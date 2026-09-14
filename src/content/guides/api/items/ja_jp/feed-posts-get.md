[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

フィード内の投稿を取得します。新しいものが先です。ページネーションはカーソルベースです。最後に受け取った投稿の `_id` を `afterId` として渡すと、次のページが取得できます。

返される10件の投稿につき1クレジットがかかります。最低でも1クレジットです。

[inline-code-attrs-start title = 'FeedPost cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** この投稿IDの後の投稿を返します。最初のページの場合は省略してください。 **/
    afterId?: string
    /** デフォルトは10です。最大は1000です。 **/
    limit?: number
    /** これらのタグを持つ投稿のみ返します。複数のタグを指定する場合はパラメータを繰り返してください。 **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 応答構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** 失敗時に含まれます。 **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]