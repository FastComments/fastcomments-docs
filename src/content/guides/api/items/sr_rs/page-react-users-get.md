[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Враћа имена корисника који су додали реакцију на страницу, сортирана абецедно. Претражује се до 100 реакција, а анонимни корисници нису укључени.

[inline-code-attrs-start title = 'Пример cURL захтева за кориснике реакција на страницу'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за кориснике реакција на страницу'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** Идентификатор реакције. **/
    id: string
    /** URI‑кодиран JSON вашег SSO објекта. Изоставите за анонимне кориснике. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за кориснике реакција на страницу'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Укључено у случају грешке. **/
    code?: string
    /** Укључено у случају грешке. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]