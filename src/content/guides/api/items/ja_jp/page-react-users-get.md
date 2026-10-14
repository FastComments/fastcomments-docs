[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

ページにリアクションを追加したユーザーの名前をアルファベット順に返します。最大100件のリアクションを検索し、匿名ユーザーは含まれません。SSO が必要なページでは、ページを閲覧できないユーザーにはコード `sso-required` または `page-forbidden` の `403` が返されます。

アバターとユーザーIDも取得するには、`includeUserInfo=public` を付けて [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) を使用してください。

[inline-code-attrs-start title = 'ページリアクトユーザー cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'ページリアクトユーザー リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'ページリアクトユーザー レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: string
    /** Included on failure. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]

---