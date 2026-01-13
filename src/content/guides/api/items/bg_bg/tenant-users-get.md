[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Този API използва пагинация, предоставена чрез параметъра на заявката `skip`. TenantUsers се връщат на страници от `100`, подредени по `signUpDate`, `username` и `id`.

Цената се базира на броя върнати tenant потребители, струващи `1 кредит на 10` върнати tenant потребители.

[inline-code-attrs-start title = 'Пример за TenantUser с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenant users to skip for pagination. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]
