[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Возвращает имена пользователей, которые добавили реакцию к странице, отсортированные в алфавитном порядке. Поиск осуществляется до 100 реакций, а анонимные пользователи не включаются.

[inline-code-attrs-start title = 'Пример cURL запроса Page React Users'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса Page React Users'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** Идентификатор реакции. **/
    id: string
    /** URI‑закодированный JSON вашего объекта SSO. Опустите для анонимных пользователей. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа Page React Users'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: string
    /** Включается при ошибке. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]

---