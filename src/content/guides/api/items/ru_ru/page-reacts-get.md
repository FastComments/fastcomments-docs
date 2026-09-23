[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Возвращает количество каждой реакции на странице и какие реакции добавил текущий пользователь.

[inline-code-attrs-start title = 'Пример cURL запроса Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI‑закодированный JSON вашего объекта SSO. Пропустите для анонимных пользователей. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: string
    /** Включается при ошибке. **/
    reason?: string
    /** Количество по идентификатору реакции, например {"heart": 12, "laugh": 3}. Не задаётся, если на странице нет реакций. **/
    counts?: Record<string, number>
    /** Идентификаторы реакций, которые пользователь, сделавший запрос, добавил. Не задаётся, если он не добавил ни одной. **/
    reactedIds?: string[]
}
[inline-code-end]

---