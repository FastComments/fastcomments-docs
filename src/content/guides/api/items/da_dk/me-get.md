[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

Beskriver legitimationsoplysningerne, der laver anmodningen: den lejer, den tilhører, og for OAuth-adgangstokens, brugeren der autoriserede applikationen. Integrationer bruger den til at teste en forbindelse og mærke den.

Med en API-nøgle identificerer svaret kun lejeren. Med et OAuth-bærer-token medfører det også den autoriserende bruger og de tildelte scopes.

[inline-code-attrs-start title = 'Me cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Me Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface MeResponse {
    status: 'success' | 'failed'
    /** Inkluderet ved fejl. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Inkluderet ved fejl. **/
    reason?: string
    tenantId: string
    tenantName: string
    /** Hvordan anmodningen blev autentificeret. **/
    authType: 'api-key' | 'oauth'
    /** De scopes legitimationsoplysningerne har. API-nøgler har begge. **/
    scopes: ('read' | 'write')[]
    /** Kun til stede for OAuth-tokens. **/
    userId?: string
    username?: string
    email?: string
}
[inline-code-end]