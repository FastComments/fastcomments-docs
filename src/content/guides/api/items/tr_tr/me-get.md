[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

İsteği yapan kimlik bilgilerini tanımlar: ait olduğu kiracı ve OAuth erişim belirteçleri için
uygulamayı yetkilendiren kullanıcı. Entegrasyonlar bunu bir bağlantıyı test etmek ve etiketlemek için kullanır.

Bir API anahtarıyla yanıt yalnızca kiracıyı tanımlar. Bir OAuth taşıyıcı belirteciyle ayrıca yetkilendiren kullanıcı ve verilen kapsamlar da taşınır.

[inline-code-attrs-start title = 'Me cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Me Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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