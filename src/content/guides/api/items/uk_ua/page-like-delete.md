[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Видаляє лайк поточного користувача зі сторінки. Якщо користувач не лайкнув сторінку, запит успішно завершується з кодом `not-liked` і не змінює лічильник.

[inline-code-attrs-start title = 'Приклад cURL для скасування лайка сторінки'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту скасування лайка сторінки'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** URI закодований у JSON вашого об’єкта SSO. Пропустіть для анонімних користувачів. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді скасування лайка сторінки'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked' коли користувач не лайкнув сторінку. Інакше включається у випадку помилки. **/
    code?: 'not-liked' | string
    /** Включається у випадку помилки. **/
    reason?: string
}
[inline-code-end]