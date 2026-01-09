[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Deze API gebruikt paginering, geleverd door de `skip` queryparameter. TenantUsers worden geretourneerd in pagina's van `100`, gesorteerd op `signUpDate`, `username` en `id`.

De kosten zijn gebaseerd op het aantal TenantUsers dat wordt geretourneerd; de kosten zijn `1 credit per 10` geretourneerde TenantUsers.

[inline-code-attrs-start title = 'TenantUser cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Aanvraagstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Het aantal TenantUsers om over te slaan voor paginering. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** Wordt opgenomen bij een fout. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Wordt opgenomen bij een fout. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]