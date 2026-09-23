[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Лайкает страницу от имени текущего пользователя. Каждый пользователь может лайкнуть страницу только один раз: повторный лайк проходит успешно с кодом `already-liked` и не изменяет счётчик.

Страница будет создана, если она ещё не существует. Передайте `title`, чтобы установить или обновить заголовок страницы.

[inline-code-attrs-start title = 'Пример cURL запроса лайка страницы'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса лайка страницы'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Устанавливает заголовок страницы. **/
    title?: string
    /** URI‑закодированный JSON вашего SSO‑объекта. Не указывайте для анонимных пользователей. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа лайка страницы'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' когда пользователь уже лайкнул страницу. В противном случае включается при ошибке. **/
    code?: 'already-liked' | string
    /** Включается при ошибке. **/
    reason?: string
}
[inline-code-end]