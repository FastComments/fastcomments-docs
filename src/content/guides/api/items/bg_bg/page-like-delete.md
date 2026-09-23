[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Премахва харесването на текущия потребител от страница. Ако потребителят не е харесал страницата, заявката се счита за успешна с код `not-liked` и не променя броя.

[inline-code-attrs-start title = 'Пример за cURL за отмяна на харесване'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за отмяна на харесване'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** URI кодиран JSON на вашия SSO обект. Пропуснете за анонимни потребители. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговор за отмяна на харесване'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked' когато потребителят не е харесал страницата. В противен случай се включва при неуспех. **/
    code?: 'not-liked' | string
    /** Включено при неуспех. **/
    reason?: string
}
[inline-code-end]