[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Този API връща tenant-и, които се управляват от вашия tenant.

Пагинацията се предоставя чрез параметъра на заявката `skip`. Tenant-ите се връщат на страници от `100`, подредени по `signUpDate` и `id`.

Цената се базира на броя върнати tenant-и, струващи `1 кредит на 10` върнати tenant-а.

[inline-code-attrs-start title = 'Пример за Tenant с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Можете да дефинирате `meta` параметри на обектите `Tenant` и да търсите съвпадащи tenant-и. Например, за ключа `someKey` и мета стойността `some-value`, можем да
конструираме JSON обект с тази двойка ключ/стойност и след това да го URI кодираме като параметър на заявката за филтриране:

[inline-code-attrs-start title = 'Пример за заявка на Tenant по Meta с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenants to skip for pagination. **/
    skip?: number
    /** Filter by meta params. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]
