[api-resource-header-start name = 'SSOUser'; route = 'GET /api/v1/sso-users'; creditsCost = 10; api-resource-header-end]

Questa route restituisce gli utenti SSO in pagine da `100`. La paginazione è fornita dal parametro `skip`. Gli utenti sono ordinati per il loro `signUpDate` e `id`.

[inline-code-attrs-start title = 'Esempio cURL SSOUsers'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta SSOUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta SSOUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di fallimento. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluso in caso di fallimento. **/
    reason?: string
    users: SSOUser[]
}
[inline-code-end]