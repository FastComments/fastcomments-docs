[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

To API używa paginacji, realizowanej za pomocą parametru zapytania `skip`. TenantUsers są zwracane w stronach po `100`, posortowane według `signUpDate`, `username` i `id`.

Koszt opiera się na liczbie zwróconych tenant users — `1 credit per 10` zwróconych tenant users.

[inline-code-attrs-start title = 'Przykład cURL dla TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Liczba tenant users do pominięcia przy paginacji. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]

---