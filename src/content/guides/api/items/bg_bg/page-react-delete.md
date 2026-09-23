[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Премахва една от реакциите на текущия потребител от страница. Ако потребителят не е добавил тази реакция, заявката се счита за успешна с код `no-react` и не променя броя.

[inline-code-attrs-start title = 'Пример за cURL заявка за изтриване на реакция на страница'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за изтриване на реакция на страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** Идентификаторът на реакцията. **/
    id: string
    /** URI кодиран JSON на вашия SSO обект. Пропуснете за анонимни потребители. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговор за изтриване на реакция на страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' когато потребителят не е добавил тази реакция. В противен случай се включва при неуспех. **/
    code?: 'no-react' | string
    /** Включено при неуспех. **/
    reason?: string
}
[inline-code-end]