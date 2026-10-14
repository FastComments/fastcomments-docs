[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

ページ上の各リアクションのカウントと、現在のユーザーが追加したリアクションを返します。

`includeUserInfo=public` を指定すると、リアクションしたユーザーも取得できます。リアクションごとにリアクション ID とともに 1 エントリが返され、最新のものが最初に来ます。プライベートプロファイルのアクティビティを持つユーザーや匿名ユーザーは除外され、最大 100 件のリアクションが取得されます。ここでは `public` のみが受け入れられます。`include-private` を使用すると `422` が返され、API キーを使用した [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) から利用可能です。SSO が必要なページでは、`includeUserInfo` はページを閲覧できないユーザーに対して `403` を返します。

[inline-code-attrs-start title = 'ページリアクト cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'ページリアクト リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** SSO オブジェクトの URI エンコードされた JSON。匿名ユーザーの場合は省略してください。 **/
    sso?: string
    /** 'public' に設定すると、リアクションしたユーザーを含めます。 **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'ページリアクト レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** 失敗時に含まれます。 **/
    reason?: string
    /** リアクション ID ごとのカウント、例: {"heart": 12, "laugh": 3}。ページにリアクションがない場合は設定されません。 **/
    counts?: Record<string, number>
    /** リクエストを行ったユーザーが追加したリアクション ID。追加がない場合は設定されません。 **/
    reactedIds?: string[]
    /** リアクションごとに 1 エントリ、最新のものが最初に来ます。includeUserInfo が渡された場合にのみ設定されます。 **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** リアクション ID、例: "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]