[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Уклања лајк тренутног корисника са странице. Ако корисник није лајковао страницу, захтев се успешно завршава са кодом `not-liked` и не мења број.

[inline-code-attrs-start title = 'Пример cURL захтева за уклањање лајка странице'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за уклањање лајка странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** URI‑кодиран JSON вашег SSO објекта. Изоставите за анонимне кориснике. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за уклањање лајка странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked' када корисник није лајковао страницу. У супротном се укључује при неуспеху. **/
    code?: 'not-liked' | string
    /** Укључено при неуспеху. **/
    reason?: string
}
[inline-code-end]