[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Враћа имена корисника који су додали реакцију на страницу, сортирана абецедно. Претражује се до 100 реакција, а анонимни корисници се не укључују. На страницама које захтевају SSO, корисници који не могу да виде страницу добијају `403` са кодом `sso-required` или `page-forbidden`.

Да бисте такође добили аватаре и ID‑ове корисника, користите [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) са `includeUserInfo=public`.

[inline-code-attrs-start title = 'Пример cURL захтева за кориснике реакција'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за кориснике реакција'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** Идентификатор реакције. **/
    id: string
    /** URI‑кодиран JSON вашег SSO објекта. Изоставите за анонимне кориснике. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за кориснике реакција'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: string
    /** Укључено у случају неуспеха. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]