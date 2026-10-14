[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

Возвращает количество каждой реакции на странице, а также при желании пользователей, которые отреагировали, в виде одной записи на реакцию с её идентификатором, самые новые первыми. Это серверная версия [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get).

`includeUserInfo` управляет тем, какие пользователи возвращаются:

- `public` исключает пользователей, сделавших свою активность профиля приватной.
- `include-private` включает их. Используйте это только когда список не показывается публично.

Анонимные пользователи никогда не включаются, и ищется до 100 реакций. Страницы, которые не существуют, возвращают отсутствие `counts` и пустой список `users`.

[inline-code-attrs-start title = 'Пример cURL запроса Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Пропустить, чтобы получить только подсчёты. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Включается при ошибке. **/
    reason?: string
    /** Количество по идентификатору реакции, например {"heart": 12, "laugh": 3}. Не задаётся, если на странице нет реакций. **/
    counts?: Record<string, number>
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

#### Полезный совет

Не забудьте URI‑кодировать значения, такие как `urlId`.