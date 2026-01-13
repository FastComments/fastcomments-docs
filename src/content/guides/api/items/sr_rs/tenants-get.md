[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Овај API враћа tenant-ове које управља ваш tenant.

Пагинација се обезбеђује помоћу query параметра `skip`. Tenant-ови се враћају по страницама од `100`, уређени по `signUpDate` и `id`.

Трошак зависи од броја враћених tenant-ова; кошта `1 credit per 10` враћених tenant-ова.

[inline-code-attrs-start title = 'Пример cURL захтева за Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Можете дефинисати `meta` параметре на `Tenant` објектима и извршити упит за одговарајуће tenant-ове. На пример, за кључ `someKey` и meta вредност `some-value`, можемо конструисати JSON објекат са овим паром кључ/вредност и затим га URI-кодирати као query параметар да бисмо филтрирали:

[inline-code-attrs-start title = 'Пример cURL захтева за упит Tenant-а по meta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Број tenants које треба прескочити за пагинацију. **/
    skip?: number
    /** Филтрирај по meta параметрима. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Укључено у случају неуспеха. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]