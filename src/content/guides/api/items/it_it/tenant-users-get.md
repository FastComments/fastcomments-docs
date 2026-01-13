[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Questa API utilizza la paginazione, fornita dal parametro di query `skip`. TenantUsers vengono restituiti in pagine da `100`, ordinati per `signUpDate`, `username` e `id`.

Il costo è basato sul numero di tenant users restituiti, con un costo di `1 credit per 10` tenant users restituiti.

[inline-code-attrs-start title = 'Esempio cURL per TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Il numero di tenant users da saltare per la paginazione. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluso in caso di errore. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]

---