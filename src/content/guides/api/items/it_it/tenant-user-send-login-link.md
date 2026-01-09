[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Questa route fornisce la possibilità di inviare un link di accesso a un singolo `TenantUser`.

Utile quando si creano utenti in batch e non si vuole dover spiegare loro come effettuare il login su FastComments.com. Questo invierà loro semplicemente un "link magico" per accedere che scade dopo `30 days`.

Esistono le seguenti restrizioni per inviare un link di accesso a un `TenantUser`:
- Il `TenantUser` deve già esistere.
- Devi avere accesso per gestire il `Tenant` a cui appartiene il `TenantUser`.

Possiamo inviare un link di accesso a un `TenantUser` come segue:

[inline-code-attrs-start title = 'Esempio cURL link di accesso TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

Questo invierà un'email come `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Struttura richiesta link di accesso TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura risposta link di accesso TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Incluso in caso di errore. **/
    reason?: string
}
[inline-code-end]