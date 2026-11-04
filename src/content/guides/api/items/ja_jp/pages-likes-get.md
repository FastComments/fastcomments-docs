[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

ページへの「いいね」の数、およびオプションで「いいね」をしたユーザー(新しい順)を返します。これは [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get) のサーバーサイド版です。

`includeUserInfo` は返されるユーザーを制御します:

- `public` は、プロフィールのアクティビティを非公開にしているユーザーを除外します。
- `include-private` はそれらのユーザーを含めます。このリストが公開されない場合にのみ使用してください。

匿名ユーザーは決して含まれず、最大100件の「いいね」が検索されます。存在しないページは `likeCount` が `0` で空の `users` リストを返します。

[inline-code-attrs-start title = 'Page Likes cURLの例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Page Likes リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** いいねの数のみ取得する場合は省略します。 **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Likes レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** 失敗時に含まれます。 **/
    reason?: string
    likeCount?: number
    /** 新しい順。includeUserInfo が指定された場合のみ設定されます。 **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### 役立つヒント

`urlId` などの値はURIエンコードすることを忘れないでください。