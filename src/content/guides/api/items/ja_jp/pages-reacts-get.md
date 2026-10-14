[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

ページ上の各リアクションのカウントを返し、オプションでリアクションしたユーザーも返します。リアクションごとにエントリが1つあり、リアクションIDとともに最新のものが最初に表示されます。これは [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) のサーバーサイド版です。

`includeUserInfo` は返されるユーザーを制御します：

- `public` はプロフィール活動を非公開にしているユーザーを除外します。
- `include-private` はそれらのユーザーも含めます。リストが公開されない場合にのみ使用してください。

匿名ユーザーは決して含まれず、最大100件のリアクションが検索されます。存在しないページは `counts` がなく、`users` リストは空になります。

[inline-code-attrs-start title = 'ページリアクト cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'ページリアクト リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omit to only get the counts. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'ページリアクト レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    /** Count per reaction id, for example {"heart": 12, "laugh": 3}. Not set when the page has no reactions. **/
    counts?: Record<string, number>
    /** One entry per react, most recent first. Only set when includeUserInfo is passed. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** The reaction id, for example "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### 役立つヒント

`urlId` のような値は URI エンコードすることを忘れないでください。

---