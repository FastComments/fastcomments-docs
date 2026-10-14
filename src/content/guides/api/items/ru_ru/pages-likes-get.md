[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

Возвращает количество лайков на странице, а при желании — список пользователей, поставивших лайк, в порядке от самых новых. Это серверная версия [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get).

`includeUserInfo` контролирует, какие пользователи возвращаются:

- `public` исключает пользователей, сделавших свою активность профиля приватной.
- `include-private` включает их. Используйте только в том случае, если список не показывается публично.

Анонимные пользователи никогда не включаются, и проверяется до 100 лайков. Страницы, которые не существуют, возвращают `likeCount` равный `0` и пустой список `users`.

[inline-code-attrs-start title = 'Пример cURL запроса для лайков страницы'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса лайков страницы'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omit to only get the like count. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа лайков страницы'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

#### Полезный совет

Не забудьте URI‑кодировать значения, такие как `urlId`.