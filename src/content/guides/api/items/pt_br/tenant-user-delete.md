[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Esta rota fornece a remoção de um `TenantUser` por id.

A exclusão dos comentários do usuário é possível através do parâmetro de query `deleteComments`. Observe que se isso for verdadeiro:

1. Todos os comentários do usuário serão excluídos imediatamente.
2. Todos os __child__ (agora órfãos) comentários serão excluídos ou anonimizados com base na configuração de página associada a cada comentário. Por exemplo se o modo de exclusão de thread for "anonymize", então as respostas permanecerão, e os comentários do usuário serão anonimizados. Isso se aplica somente quando `commentDeleteMode` é `Remove` (o valor padrão).
3. O `creditsCost` passa a ser `2`.

### Comentários anonimizados

Você pode manter os comentários do usuário, mas simplesmente anonimizá-los definindo `commentDeleteMode=1`.

Se os comentários do usuário forem anonimizados, então os seguintes valores serão definidos como null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` e `isDeletedUser` são definidos como `true`.

Ao renderizar, o widget de comentários usará `DELETED_USER_PLACEHOLDER` (padrão: "[deleted]") para o nome do usuário e `DELETED_CONTENT_PLACEHOLDER` para o comentário. Estes podem ser personalizados através da UI de Personalização do Widget.

### Exemplos

[inline-code-attrs-start title = 'Exemplo cURL: Remoção de TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da requisição de remoção de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // padrão
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Você pode definir isto como 'true' para também excluir os comentários do usuário. Isso dobrará o custo de créditos. **/
    deleteComments?: 'true' | 'false'
    /** Você pode definir isto conforme desejado para determinar como lidar com os comentários do usuário. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da resposta de remoção de TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Incluído em caso de falha. **/
    reason?: string
}
[inline-code-end]

---