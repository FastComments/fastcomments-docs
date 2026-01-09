[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Овај API користи пагинацију, обезбјеђену параметром упита `skip`. TenantUsers се враћају у страницама по `100`, сортирани по `signUpDate`, `username` и `id`.

Трошкови се заснивају на броју враћених TenantUsers, и износе `1 credit per 10` враћених TenantUsers.

[inline-code-attrs-start title = 'Пример cURL захтева за TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Број TenantUsers које треба прескочити за пагинацију. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** Укључено при неуспеху. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Укључено при неуспеху. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]

---