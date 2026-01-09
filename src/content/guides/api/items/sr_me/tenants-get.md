[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Овај API враћа tenants којима управља ваш tenant.

Пагинација се обезбеђује помоћу query параметра `skip`. Tenants се враћају страницама по `100`, сортирано по `signUpDate` и `id`.

Трошак се заснива на броју враћених tenants и износи `1 credit per 10` враћених tenants.

[inline-code-attrs-start title = 'Пример cURL захтева за Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Можете дефинисати `meta` параметре на `Tenant` објектима и тражити одговарајуће tenants. На пример, за кључ `someKey` и meta вредност `some-value`, можемо конструисати JSON објекат са тим паром кључ/вредност и потом га URI енкодовати као query параметар за филтрирање:

[inline-code-attrs-start title = 'Пример cURL захтева за претрагу Tenant по meta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Број tenants које треба прескочити за потребе пагинације. **/
    skip?: number
    /** Филтрирање по meta параметрима. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Укључено у случају грешке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Укључено у случају грешке. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]

---