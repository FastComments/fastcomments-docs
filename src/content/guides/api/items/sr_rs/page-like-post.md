[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Лајкује страницу као тренутни корисник. Сваки корисник може лајковати страницу само једном: поновно лајковање успева са кодом `already-liked` и не мења број.

Страница се креира ако још не постоји. Проследите `title` да поставите или ажурирате наслов странице.

[inline-code-attrs-start title = 'cURL пример за лајк странице'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за лајк странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Поставља наслов странице. **/
    title?: string
    /** URI‑кодиран JSON вашег SSO објекта. Изоставите за анонимне кориснике. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за лајк странице'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' када је корисник већ лајковао страницу. У супротном се укључује при неуспеху. **/
    code?: 'already-liked' | string
    /** Укључено при неуспеху. **/
    reason?: string
}
[inline-code-end]