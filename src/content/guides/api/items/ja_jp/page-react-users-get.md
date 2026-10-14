[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

ページにリアクションを追加したユーザーの名前をアルファベット順に返します。最大 100 件のリアクションが検索され、匿名ユーザーおよびプライベートプロフィールのユーザーは含まれません。SSO が必要なページでは、ページを閲覧できないユーザーは `403` とコード `sso-required` または `page-forbidden` が返されます。

アバターとユーザー ID も取得するには、`includeUserInfo=public` を付けて [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) を使用してください。

[inline-code-attrs-start title = 'ページリアクションユーザー cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'ページリアクションユーザー リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** リアクション ID。 **/
    id: string
    /** SSO オブジェクトの URI エンコードされた JSON。匿名ユーザーの場合は省略してください。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'ページリアクションユーザー レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: string
    /** 失敗時に含まれます。 **/
    reason?: string
    userNames: string[]
}
[inline-code-end]