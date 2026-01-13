[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Este endpoint da API fornece a capacidade de bloquear um usuário que escreveu um determinado comentário. Ele suporta bloqueio de comentários escritos por FastComments.com Users, SSO Users e Tenant Users.

Ele suporta um parâmetro de corpo `commentIdsToCheck` para verificar se quaisquer outros comentários potencialmente visíveis no cliente devem ser bloqueados/desbloqueados após esta ação ser realizada.

Notas:

- Esta chamada deve sempre ser feita no contexto de um usuário. O usuário pode ser um FastComments.com User, SSO User ou Tenant User.
- O `userId` na requisição é o usuário que está *fazendo o bloqueio*. Por exemplo: `User A` quer bloquear `User B`. Passe `userId=User A` e o id do comentário que `User B` escreveu.
- Comentários completamente anônimos (sem user id, sem email) não podem ser bloqueados e um erro será retornado.

[inline-code-attrs-start title = 'Exemplo cURL de bloqueio de comentário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Para bloqueio anônimo, devemos especificar um `anonUserId`. Isso pode ser um ID que representa a sessão anônima, ou um UUID aleatório.
Isso nos permite suportar o bloqueio de comentários mesmo que um usuário não esteja logado, buscando os comentários com o mesmo `anonUserId`.

[inline-code-attrs-start title = 'Exemplo cURL de bloqueio de comentário anônimo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da requisição de bloqueio de comentário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da resposta de bloqueio de comentário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Incluído em caso de falha. **/
    reason?: string
    /** Se commentIdsToCheck estiver definido, entradas neste mapa com true também são bloqueadas. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---