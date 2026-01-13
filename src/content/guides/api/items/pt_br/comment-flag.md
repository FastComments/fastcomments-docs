[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Este endpoint da API permite sinalizar um comentário para um usuário específico.

Notas:

- Essa chamada deve sempre ser feita no contexto de um usuário. O usuário pode ser um FastComments.com User, SSO User, ou Tenant User.
- Se um limite de sinalizações-para-ocultar estiver definido, o comentário será automaticamente ocultado ao vivo depois de ter sido sinalizado o número de vezes definido.
- Depois que for automaticamente desaprovado (oculto) - o comentário só poderá ser re-aprovado por um administrador ou moderador. Remover a sinalização não re-aprovará o comentário.

[inline-code-attrs-start title = 'Exemplo cURL de sinalização de comentário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Para sinalização anônima, devemos especificar um `anonUserId`. Isso pode ser um ID que representa a sessão anônima, ou um UUID aleatório.
Isso nos permite suportar a sinalização e a remoção da sinalização de comentários mesmo que um usuário não esteja logado. Dessa forma, o comentário pode ser marcado como
sinalizado quando os comentários são buscados com o mesmo `anonUserId`.

[inline-code-attrs-start title = 'Exemplo cURL de sinalização de comentário anônimo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Estrutura da Requisição de Sinalização de Comentário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Sinalização de Comentário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Incluído em caso de falha. **/
    reason?: string
    /** O comentário foi desaprovado (oculto) devido a ter sido sinalizado muitas vezes? **/
    wasUnapproved?: boolean;
}
[inline-code-end]