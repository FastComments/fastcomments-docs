[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Цей API повертає орендарів (tenants), якими керує ваш орендар.

Пагінація здійснюється за допомогою параметра запиту `skip`. Орендарі повертаються сторінками по `100`, відсортовані за `signUpDate` та `id`.

Вартість залежить від кількості повернених орендарів — `1 кредит за 10` повернених орендарів.

[inline-code-attrs-start title = 'Приклад cURL для Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Ви можете визначити параметри `meta` в об\'єктах `Tenant` і виконувати запити для пошуку відповідних орендарів. Наприклад, для ключа `someKey` та значення meta `some-value` ми можемо
створити JSON-об\'єкт із цією парою ключ/значення, а потім URI-кодувати його як параметр запиту для фільтрації:

[inline-code-attrs-start title = 'Приклад cURL запиту Tenant за meta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Кількість орендарів, які слід пропустити для пагінації. **/
    skip?: number
    /** Фільтрувати за параметрами meta. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Включається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включається у разі помилки. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]