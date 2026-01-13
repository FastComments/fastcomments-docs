[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Esta rota fornece a remoção de um único usuário SSO pelo seu id.

Observe que recarregar o widget de comentários com um payload para este usuário simplesmente recriará o usuário de forma transparente.

A remoção dos comentários do usuário é possível via o `deleteComments` query parameter. Note que se isso for true:

1. Todos os comentários do usuário serão excluídos ao vivo.
2. All __child__ (now orphan) comments will be deleted or anonymized based on each comment's associated page configuration. For example if thread deletion mode is "anonymize", then replies will remain, and the user's comments will be anonymized. This only applies when `commentDeleteMode` is `Remove` (the default value).
3. O `creditsCost` passa a ser `2`.

### Comentários Anonimizados

Você pode manter os comentários do usuário, mas simplesmente anonimizá-los definindo `commentDeleteMode=1`.

Se os comentários do usuário forem anonimizados, os seguintes valores são definidos como null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` e `isDeletedUser` são definidos como `true`.

Ao renderizar, o widget de comentários usará `DELETED_USER_PLACEHOLDER` (padrão: "[deleted]") para o nome do usuário e `DELETED_CONTENT_PLACEHOLDER` para o comentário. Estes podem ser personalizados via a UI de Personalização do Widget.

### Exemplos

[inline-code-attrs-start title = 'Exemplo cURL de remoção de SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Requisição de Remoção de SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // padrão
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Você pode definir isto como true para também excluir os comentários do usuário. Isso duplicará o custo em créditos. **/
    deleteComments?: 'true' | 'false'
    /** Você pode definir isto conforme desejar para determinar como tratar os comentários do usuário. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de Remoção de SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Incluído em caso de falha. **/
    reason?: string
    user?: SSOUser; // Retornamos o usuário removido em caso de sucesso.
}
[inline-code-end]