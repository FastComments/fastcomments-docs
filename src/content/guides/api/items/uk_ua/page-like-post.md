[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Лайкає сторінку від імені поточного користувача. Кожен користувач може лайкнути сторінку лише один раз: повторне лайкання успішне з кодом `already-liked` і не змінює лічильник.

Сторінка створюється, якщо ще не існує. Передайте `title`, щоб встановити або оновити заголовок сторінки.

[inline-code-attrs-start title = 'Приклад cURL для лайку сторінки'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту лайку сторінки'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Встановлює заголовок сторінки. **/
    title?: string
    /** URI‑закодований JSON вашого SSO‑об’єкта. Пропустіть для анонімних користувачів. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді лайку сторінки'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' коли користувач вже лайкнув сторінку. Інакше включається при помилці. **/
    code?: 'already-liked' | string
    /** Включається при помилці. **/
    reason?: string
}
[inline-code-end]