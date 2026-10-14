[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Повертає кількість лайків на сторінці та чи поточний користувач поставив лайк. Сторінки, які ще не існують, повертають `likeCount` зі значенням `0`.

Передайте `includeUserInfo=public`, щоб також отримати користувачів, які поставили лайк сторінці, у порядку від найновіших. Користувачі з приватною активністю профілю та анонімні користувачі виключаються, і шукаються до 100 лайків. Тут приймається лише `public`; `include-private` повертає `422` і доступний через [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) з API‑ключем. На сторінках, які вимагають SSO, `includeUserInfo` повертає `403` для користувачів, які не можуть переглянути сторінку.

[inline-code-attrs-start title = 'Приклад cURL для Page Likes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту Page Likes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who liked the page. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді Page Likes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Included on failure. **/
    reason?: string
    likeCount: number
    /** Whether the user making the request has liked the page. **/
    didLike: boolean
    /** The number of top level comments on the page. **/
    commentCount: number
    /** The id used to subscribe to live updates for this page. **/
    urlIdWS: string
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