[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Повертає кількість лайків на сторінці та чи поточний користувач поставив лайк. Сторінки, які ще не існують, повертають `likeCount` рівний `0`.

[inline-code-attrs-start title = 'Приклад cURL запиту для лайків сторінки'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту лайків сторінки'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI‑закодований JSON вашого об’єкта SSO. Пропустіть для анонімних користувачів. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді лайків сторінки'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Включено у випадку помилки. **/
    code?: string
    /** Включено у випадку помилки. **/
    reason?: string
    likeCount: number
    /** Чи користувач, який робить запит, поставив лайк сторінці. **/
    didLike: boolean
    /** Кількість коментарів верхнього рівня на сторінці. **/
    commentCount: number
    /** Ідентифікатор, який використовується для підписки на живі оновлення цієї сторінки. **/
    urlIdWS: string
}
[inline-code-end]