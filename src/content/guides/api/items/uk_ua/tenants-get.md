[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Цей API повертає tenants, якими керує ваш tenant.

Пагінація здійснюється за допомогою параметра запиту `skip`. Tenants повертаються сторінками по `100`, впорядковані за `signUpDate` та `id`.

Вартість залежить від кількості повернутих tenants і становить `1 credit per 10` tenants.

[inline-code-attrs-start title = 'Приклад cURL для Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Ви можете визначати параметри `meta` в об'єктах `Tenant` і виконувати запит для пошуку відповідних tenants. Наприклад, для ключа `someKey` та значення meta `some-value`, ми можемо
construct a JSON object with this key/value pair and then URI encode it as a query param to filter:

[inline-code-attrs-start title = 'Приклад cURL запиту Tenant за Meta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Кількість tenants, які слід пропустити для пагінації. **/
    skip?: number
    /** Фільтрувати за параметрами meta. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Додається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Додається у разі помилки. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]

---