[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Este endpoint da API fornece a capacidade de desbloquear um usuário que escreveu um determinado comentário. Ele suporta o desbloqueio de comentários escritos por FastComments.com Users, SSO Users, and Tenant Users.

Ele suporta um parâmetro de corpo `commentIdsToCheck` para verificar se quaisquer outros comentários potencialmente visíveis no cliente devem ser bloqueados/desbloqueados após esta ação ser executada.

Notas:

- Esta chamada deve sempre ser feita no contexto de um usuário. O usuário pode ser um FastComments.com User, SSO User, ou Tenant User.
- O `userId` na requisição é o usuário que está *realizando o desbloqueio*. Por exemplo: `User A` quer desbloquear `User B`. Passe `userId=User A` e o id do comentário que `User B` escreveu.
- Comentários completamente anônimos (sem user id, sem email) não podem ser bloqueados e um erro será retornado.

[inline-code-attrs-start title = 'Exemplo cURL de Desbloqueio de Comentário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Exemplo cURL de Desbloqueio de Comentário Anônimo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Desbloqueio de Comentário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Desbloqueio de Comentário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Incluído em caso de falha. **/
    reason?: string
    /** Se commentIdsToCheck estiver definido, entradas neste mapa com true ainda estão bloqueadas. Se false, você pode querer reexibir os comentários para o usuário para que ele não precise atualizar. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]