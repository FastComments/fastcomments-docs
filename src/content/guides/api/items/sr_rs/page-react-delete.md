[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Уклања једну од реакција тренутног корисника са странице. Ако корисник није додао ту реакцију, захтев се сматра успешним са кодом `no-react` и не мења број.

[inline-code-attrs-start title = 'Пример cURL захтева за брисање реакције странице'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за брисање реакције странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** Идентификатор реакције. **/
    id: string
    /** URI‑кодиран JSON вашег SSO објекта. Изоставите за анонимне кориснике. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за брисање реакције странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' када корисник није додао ову реакцију. У супротном је укључено у случају грешке. **/
    code?: 'no-react' | string
    /** Укључено у случају грешке. **/
    reason?: string
}
[inline-code-end]