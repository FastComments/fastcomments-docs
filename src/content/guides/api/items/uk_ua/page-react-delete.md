[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Видаляє одну з реакцій поточного користувача зі сторінки. Якщо користувач не додав цю реакцію, запит успішно завершується з кодом `no-react` і не змінює лічильник.

[inline-code-attrs-start title = 'Приклад cURL для видалення реакції на сторінці'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту видалення реакції на сторінці'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** Ідентифікатор реакції. **/
    id: string
    /** URI закодований JSON вашого SSO об’єкта. Пропустіть для анонімних користувачів. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді на видалення реакції на сторінці'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' коли користувач не додав цю реакцію. Інакше включається при помилці. **/
    code?: 'no-react' | string
    /** Включено при помилці. **/
    reason?: string
}
[inline-code-end]