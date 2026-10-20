[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Повертає імена користувачів, які додали реакцію до сторінки, відсортовані в алфавітному порядку. Перевіряються до 100 реакцій, а анонімні користувачі та користувачі з приватною активністю профілю не включаються. На сторінках, які вимагають SSO, користувачі, які не можуть переглянути сторінку, отримують `403` з кодом `sso-required` або `page-forbidden`.

Щоб також отримати аватари та ідентифікатори користувачів, використовуйте [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) з `includeUserInfo=public`.

[inline-code-attrs-start title = 'Приклад cURL запиту Page React Users'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту Page React Users'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді Page React Users'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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