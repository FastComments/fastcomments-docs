[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Этот API возвращает tenants, которыми управляет ваш tenant.

Пагинация предоставляется с помощью параметра запроса `skip`. Tenants возвращаются страницами по `100`, упорядоченных по `signUpDate` и `id`.

Стоимость зависит от количества возвращаемых tenants, составляя `1 credit per 10` возвращенных tenants.

[inline-code-attrs-start title = 'Пример cURL-запроса для Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Вы можете задать параметры `meta` в объектах `Tenant` и выполнять запрос для соответствующих tenants. Например, для ключа `someKey` и значения meta `some-value`, мы можем сформировать JSON-объект с этой парой ключ/значение и затем URI-кодировать его как параметр запроса для фильтрации:

[inline-code-attrs-start title = 'Пример cURL-запроса: фильтрация Tenant по meta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Количество tenants, которые нужно пропустить для пагинации. **/
    skip?: number
    /** Фильтр по параметрам meta. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Включается при ошибке. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]

---