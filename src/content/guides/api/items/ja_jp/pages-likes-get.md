[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

ページのいいね数を返し、オプションでいいねしたユーザーを最新順で返します。これは [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get) のサーバーサイド版です。

`includeUserInfo` は返されるユーザーを制御します:

- `public` は、プロフィール活動を非公開にしているユーザーを除外します。
- `include-private` はそれらを含めます。リストが公開されない場合にのみ使用してください。

匿名ユーザーは決して含まれず、最大 100 件のいいねが取得されます。存在しないページは `likeCount` が `0` となり、`users` リストは空になります。

[inline-code-attrs-start title = 'ページいいね cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'ページいいね リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** カウントのみ取得する場合は省略してください。 **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'ページいいね レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** 失敗時に含まれます。 **/
    reason?: string
    likeCount?: number
    /** 最新のものが最初に来ます。includeUserInfo が渡された場合にのみ設定されます。 **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Helpful Tip

`urlId` のような値は URI エンコードすることを忘れないでください。