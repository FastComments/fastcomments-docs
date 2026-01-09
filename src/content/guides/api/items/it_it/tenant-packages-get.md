[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Questa API utilizza la paginazione, fornita dal parametro di query `skip`. I TenantPackages vengono restituiti in pagine da `100`, ordinati per `createdAt` e `id`.

Il costo si basa sul numero di tenant packages restituiti, pari a `1 credit per 10` tenant packages restituiti.

[inline-code-attrs-start title = 'Esempio cURL per TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Il numero di tenant packages da saltare per la paginazione. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluso in caso di errore. **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]