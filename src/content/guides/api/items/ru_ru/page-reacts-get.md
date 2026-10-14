[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Возвращает количество каждой реакции на странице и какие реакции добавил текущий пользователь.

Передайте `includeUserInfo=public`, чтобы также получить пользователей, которые отреагировали, в виде одной записи на реакцию с её идентификатором, самые новые первыми. Пользователи с приватной активностью профиля и анонимные пользователи исключаются, и ищется до 100 реакций. Здесь принимается только `public`; `include-private` возвращает `422` и доступен через [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) с API‑ключом. На страницах, требующих SSO, `includeUserInfo` возвращает `403` для пользователей, которые не могут просматривать страницу.

[inline-code-attrs-start title = 'Пример cURL для Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI‑закодированный JSON вашего объекта SSO. Оставьте пустым для анонимных пользователей. **/
    sso?: string
    /** Установите 'public', чтобы включить пользователей, которые отреагировали. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Включается при ошибке. **/
    reason?: string
    /** Количество по идентификатору реакции, например {"heart": 12, "laugh": 3}. Не задаётся, если на странице нет реакций. **/
    counts?: Record<string, number>
    /** Идентификаторы реакций, которые добавил пользователь, сделавший запрос. Не задаётся, если они не добавлены. **/
    reactedIds?: string[]
    /** Одна запись на реакцию, самые новые первыми. Устанавливается только при передаче includeUserInfo. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** Идентификатор реакции, например "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]