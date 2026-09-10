[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

Opisuje poverilnico, ki izvaja zahtevo: najemnika, ki ji pripada, in za žetone OAuth dostopa tudi
uporabnika, ki je odobril aplikacijo. Integracije jo uporabljajo za testiranje povezave in njeno označevanje.

Pri uporabi API ključa odgovor identificira le najemnika. Pri uporabi OAuth nosilnega žetona prav tako prenaša
avtoriziranega uporabnika in dodeljene obsege.

[inline-code-attrs-start title = 'Primer cURL zahteve Me'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora Me'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface MeResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Vključeno ob napaki. **/
    reason?: string
    tenantId: string
    tenantName: string
    /** Kako je bila zahteva avtenticirana. **/
    authType: 'api-key' | 'oauth'
    /** Obsegi, ki jih ima poverilnica. API ključi vsebujejo oba. **/
    scopes: ('read' | 'write')[]
    /** Prisotno le za OAuth žetone. **/
    userId?: string
    username?: string
    email?: string
}
[inline-code-end]