[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Повертає кількість кожної реакції на сторінці та які реакції додав поточний користувач.

[inline-code-attrs-start title = 'Приклад cURL для Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI закодований JSON вашого об’єкта SSO. Пропустіть для анонімних користувачів. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Включено у випадку помилки. **/
    code?: string
    /** Включено у випадку помилки. **/
    reason?: string
    /** Кількість для кожного id реакції, наприклад {"heart": 12, "laugh": 3}. Не встановлюється, коли на сторінці немає реакцій. **/
    counts?: Record<string, number>
    /** ID реакцій, які додав користувач, що робить запит. Не встановлюється, якщо жодної не додано. **/
    reactedIds?: string[]
}
[inline-code-end]