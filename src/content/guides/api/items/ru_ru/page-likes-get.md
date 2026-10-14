[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Возвращает количество лайков на странице и то, поставил ли текущий пользователь лайк. Страницы, которые ещё не существуют, возвращают `likeCount` со значением `0`.

Передайте `includeUserInfo=public`, чтобы также получить пользователей, поставивших лайк странице, в порядке от самых новых. Пользователи с приватной активностью профиля и анонимные пользователи исключаются, и ищутся до 100 лайков. Здесь принимается только `public`; `include-private` возвращает `422` и доступен через [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) с API‑ключом. На страницах, требующих SSO, `includeUserInfo` возвращает `403` для пользователей, которые не могут просматривать страницу.

[inline-code-attrs-start title = 'Пример cURL запроса лайков страницы'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса лайков страницы'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI‑закодированный JSON вашего объекта SSO. Оставьте пустым для анонимных пользователей. **/
    sso?: string
    /** Установите 'public', чтобы включить пользователей, поставивших лайк странице. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа лайков страницы'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Включается при ошибке. **/
    reason?: string
    likeCount: number
    /** Указывает, поставил ли пользователь, сделавший запрос, лайк странице. **/
    didLike: boolean
    /** Количество комментариев верхнего уровня на странице. **/
    commentCount: number
    /** Идентификатор, используемый для подписки на живые обновления этой страницы. **/
    urlIdWS: string
    /** Сначала самые новые. Устанавливается только при передаче includeUserInfo. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]