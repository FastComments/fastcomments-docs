[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Додає реакцію до сторінки від імені поточного користувача. Користувач може додати кожен ідентифікатор реакції лише один раз: повторне додавання успішне з кодом `already-reacted` і не змінює лічильник. Користувач може додати кілька різних реакцій до однієї сторінки.

Ідентифікатори реакцій вибираються вами і можуть мати до 36 символів. Сторінка створюється, якщо ще не існує. Передайте `title`, щоб встановити або оновити заголовок сторінки.

[inline-code-attrs-start title = 'Приклад cURL запиту реакції на сторінку'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту реакції на сторінку'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** The reaction id, up to 36 characters. **/
    id: string
    /** Sets the page's title. **/
    title?: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді реакції на сторінку'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted' when the user had already added this reaction. 'react-id-too-long' (HTTP 422) when the id is over 36 characters. Otherwise included on failure. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]