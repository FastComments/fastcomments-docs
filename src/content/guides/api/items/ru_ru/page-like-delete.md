[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Удаляет лайк текущего пользователя со страницы. Если пользователь не ставил лайк странице, запрос завершается успешно с кодом `not-liked` и количество не изменяется.

[inline-code-attrs-start title = 'Пример cURL для снятия лайка со страницы'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса снятия лайка со страницы'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа снятия лайка со страницы'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked' when the user had not liked the page. Otherwise included on failure. **/
    code?: 'not-liked' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]