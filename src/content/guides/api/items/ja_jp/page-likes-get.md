[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

ページのいいね数と、現在のユーザーがそのページにいいねしたかどうかを返します。まだ存在しないページは `likeCount` が `0` になります。

`includeUserInfo=public` を指定すると、ページにいいねしたユーザーを取得できます（最新のものが先）。プライベートなプロフィール活動を持つユーザーや匿名ユーザーは除外され、最大 100 件のいいねが取得されます。ここでは `public` のみが受け付けられ、`include-private` を指定すると `422` が返され、API キーを使用して [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) から利用可能です。SSO が必要なページでは、`includeUserInfo` はページを閲覧できないユーザーに対して `403` を返します。

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
    /** ページにいいねしたユーザーを含めるには 'public' を設定してください。 **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'ページのいいね レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** 失敗時に含まれます。 **/
    reason?: string
    likeCount: number
    /** リクエストを行ったユーザーがページにいいねしたかどうか。 **/
    didLike: boolean
    /** ページ上のトップレベルコメント数。 **/
    commentCount: number
    /** このページのライブ更新を購読するために使用される ID。 **/
    urlIdWS: string
    /** 最新のものが先。includeUserInfo が渡された場合にのみ設定されます。 **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]