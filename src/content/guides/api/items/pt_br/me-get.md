[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

Descreve a credencial que faz a requisição: o tenant ao qual pertence e, para tokens de acesso OAuth, o usuário que autorizou a aplicação. Integrações a utilizam para testar uma conexão e rotulá‑la.

Com uma chave de API a resposta identifica apenas o tenant. Com um token bearer OAuth também inclui o usuário autorizador e os escopos que foram concedidos.

[inline-code-attrs-start title = 'Exemplo cURL de Me'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta Me'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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