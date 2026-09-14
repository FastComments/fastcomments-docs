[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Esta rota cria um único `FeedPost`. Cada postagem tem um autor, portanto `fromUserId` é obrigatório e deve ser o ID de um usuário FastComments ou SSO existente na conta.

[inline-code-attrs-start title = 'Exemplo cURL de Criação de FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&isLive=true&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "fromUserId": "some-user-id",
    "title": "Release 2.0 is out",
    "contentHTML": "<p>Read the notes and tell us what you think.</p>",
    "tags": ["releases"],
    "links": [
        {
            "url": "https://example.com/releases/2.0",
            "title": "Release notes",
            "description": "Everything that changed in 2.0."
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição de Criação de FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Envia a postagem para feeds que estão abertos em um navegador agora. O padrão é false. **/
    isLive?: boolean
    /** Executa a postagem através do mecanismo de spam antes de salvar. O padrão é false. **/
    doSpamCheck?: boolean
    /** Ignora a verificação de conteúdo repetido que ocorre como parte do doSpamCheck. O padrão é false. **/
    skipDupCheck?: boolean
    /** Até 256 caracteres. Ecoado para ouvintes ao vivo para que um cliente possa ignorar sua própria transmissão. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Obrigatório. Um ID de usuário FastComments ou SSO. **/
    fromUserId: string
    title?: string
    /** HTML. Sanitizado ao salvar. **/
    contentHTML?: string
    /** Substitui o nome de exibição obtido do usuário. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de Criação de FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Incluído em caso de falha. **/
    reason?: string
    feedPost?: FeedPost; // Retornamos a postagem completa criada em caso de sucesso.
}
[inline-code-end]