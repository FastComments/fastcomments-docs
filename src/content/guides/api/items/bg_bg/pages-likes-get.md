[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

Връща броя на харесванията на страница и, по желание, потребителите, които я харесаха, най‑новите първи. Това е сървърната версия на [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get).

`includeUserInfo` контролира кои потребители се връщат:

- `public` оставя потребителите, които са направили активността на профила си частна.
- `include-private` ги включва. Използвайте това само когато списъкът не се показва на публично ниво.

Анонимните потребители никога не се включват и се проверяват до 100 харесвания. Страници, които не съществуват, връщат `likeCount` от `0` и празен списък `users`.

[inline-code-attrs-start title = 'Пример cURL за харесвания на страница'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за харесвания на страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omit to only get the like count. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговор за харесвания на страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    likeCount?: number
    /** Most recent first. Only set when includeUserInfo is passed. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Полезен съвет

Не забравяйте да URI кодирате стойностите като `urlId`.

---