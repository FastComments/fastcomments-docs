[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

このルートは単一の `FeedPost` を作成します。すべての投稿には作者が必要なため、`fromUserId` は必須で、アカウント上の既存の FastComments または SSO ユーザーの ID でなければなりません。

[inline-code-attrs-start title = 'FeedPost 作成 cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&isLive=true&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "fromUserId": "some-user-id",
    "title": "Release 2.0 is out",
    "contentHTML": "<p>Read the notes and tell us what you think.</p>",
    "tags": ["releases"],
    "links": [
        {
            "url": "https://example.com/releases/2.0",
            "title": "Release notes",
            "description": "Everything that changed in 2.0."
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 作成 リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** 現在ブラウザで開かれているフィードに投稿をプッシュします。デフォルトは false です。 **/
    isLive?: boolean
    /** 保存前にスパムエンジンで投稿を処理します。デフォルトは false です。 **/
    doSpamCheck?: boolean
    /** doSpamCheck の一部として実行される重複コンテンツチェックをスキップします。デフォルトは false です。 **/
    skipDupCheck?: boolean
    /** 最大256文字。ライブリスナーにエコーされるため、クライアントは自分自身のブロードキャストを無視できます。 **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** 必須。FastComments または SSO ユーザー ID。 **/
    fromUserId: string
    title?: string
    /** HTML。保存時にサニタイズされます。 **/
    contentHTML?: string
    /** ユーザーから取得した表示名を上書きします。 **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 作成 レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** 失敗時に含まれます。 **/
    reason?: string
    feedPost?: FeedPost; // 成功時に作成された完全な投稿を返します。
}
[inline-code-end]