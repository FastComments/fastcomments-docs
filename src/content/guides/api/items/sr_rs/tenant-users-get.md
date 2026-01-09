[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Овај API користи пагинацију, обезбеђену преко `skip` query параметра. TenantUsers се враћају у страницама по `100`, сортирани по `signUpDate`, `username` и `id`.

Трошак зависи од броја враћених TenantUsers: `1 credit per 10` враћених TenantUsers.

[inline-code-attrs-start title = 'Пример cURL захтева за TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Број TenantUsers које треба прескочити за пагинацију. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** Укључује се у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Укључује се у случају неуспеха. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]