[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

Descrive le credenziali che effettuano la richiesta: il tenant a cui appartengono e, per i token di accesso OAuth, l'
utente che ha autorizzato l'applicazione. Le integrazioni lo usano per testare una connessione e etichettarla.

Con una chiave API la risposta identifica solo il tenant. Con un token bearer OAuth trasporta anche l'
utente autorizzatore e gli scope concessi.

[inline-code-attrs-start title = 'Esempio cURL Me'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta Me'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface MeResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenantId: string
    tenantName: string
    /** How the request was authenticated. **/
    authType: 'api-key' | 'oauth'
    /** The scopes the credential holds. API keys hold both. **/
    scopes: ('read' | 'write')[]
    /** Only present for OAuth tokens. **/
    userId?: string
    username?: string
    email?: string
}
[inline-code-end]