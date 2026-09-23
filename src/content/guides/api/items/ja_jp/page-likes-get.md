[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

ページのいいね数と、現在のユーザーがそのページにいいねしたかどうかを返します。まだ存在しないページは `likeCount` が `0` になります。

[inline-code-attrs-start title = 'ページのいいね cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'ページのいいね リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** SSO オブジェクトの URI エンコードされた JSON。匿名ユーザーの場合は省略してください。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'ページのいいね レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: string
    /** 失敗時に含まれます。 **/
    reason?: string
    likeCount: number
    /** リクエストを行ったユーザーがページにいいねしたかどうか。 **/
    didLike: boolean
    /** ページ上のトップレベルコメントの数。 **/
    commentCount: number
    /** このページのライブ更新を購読するために使用される ID。 **/
    urlIdWS: string
}
[inline-code-end]