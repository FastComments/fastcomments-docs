[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Връща имената на потребителите, които са добавили реакция към страница, подредени по азбучен ред. Търсят се до 100 реакции, а анонимните потребители не се включват.

[inline-code-attrs-start title = 'Пример cURL за потребители на реакция към страница'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за потребители на реакция към страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** Идентификаторът на реакцията. **/
    id: string
    /** URI кодиран JSON на вашия SSO обект. Пропуснете за анонимни потребители. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговор за потребители на реакция към страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Включено при неуспех. **/
    code?: string
    /** Включено при неуспех. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]