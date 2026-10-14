[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Връща имената на потребителите, които са добавили реакция към страница, подредени по азбучен ред. До 100 реакции се проверяват и анонимните потребители не се включват. На страници, които изискват SSO, потребителите, които не могат да видят страницата, получават `403` с код `sso-required` или `page-forbidden`.

За да получите и аватари и идентификатори на потребителите, използвайте [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) с `includeUserInfo=public`.

[inline-code-attrs-start title = 'Пример cURL за потребители на реакциите към страница'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за потребители на реакциите към страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговор за потребители на реакциите към страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: string
    /** Included on failure. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]