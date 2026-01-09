[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

Este endpoint da API fornece a capacidade de remover a sinalização (un-flag) de um comentário para um usuário específico.

Observações:

- Esta chamada deve sempre ser feita no contexto de um usuário. O usuário pode ser um FastComments.com User, SSO User, ou Tenant User.
- Após um comentário ser desaprovado automaticamente (oculto) - o comentário só pode ser reaprovado por um administrador ou moderador. Remover a sinalização (un-flagging) não irá reaprovar o comentário.

[inline-code-attrs-start title = 'Exemplo cURL de remoção de sinalização de comentário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Para sinalização anônima, devemos especificar um `anonUserId`. Isso pode ser um ID que representa a sessão anônima, ou um UUID aleatório.

[inline-code-attrs-start title = 'Exemplo cURL de remoção de sinalização de comentário anônimo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Estrutura da Requisição de Remoção de Sinalização de Comentário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Remoção de Sinalização de Comentário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Incluído em caso de falha. **/
    reason?: string
}
[inline-code-end]

---