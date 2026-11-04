[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Esta rota fornece a remoção de um único usuário SSO pelo seu id.

Observe que carregar o widget de comentários novamente com um payload para este usuário simplesmente recriará o usuário de forma transparente.

É possível excluir os comentários do usuário através do parâmetro de consulta `deleteComments`. Observe que, se for verdadeiro:

1. Todos os comentários do usuário serão excluíados em tempo real.
2. Todos os comentários __filhos__ (agora órfãos) serão excluídos ou anonimizados com base na configuração de página associada a cada comentário. Por exemplo, se o modo de exclusão de thread for "anonymize", as respostas permanecerão e os comentários do usuário serão anonimizados. Isso se aplica apenas quando `commentDeleteMode` é `Remove` (o valor padrão).
3. O `creditsCost` se torna `2`.

### Comentários Anonimizados

Você pode manter os comentários do usuário, mas simplesmente anonimizá-los definindo `commentDeleteMode=1`.

Se os comentários do usuário forem anonimizados, os seguintes valores serão definidos como nulos:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` e `isDeletedUser` são definidos como `true`.

Ao renderizar, o widget de comentários usará `DELETED_USER_PLACEHOLDER` (padrão: "[deleted]") para o nome do usuário e `DELETED_CONTENT_PLACEHOLDER` para o comentário. Eles podem ser personalizados através da UI de Personalização do Widget.

### Exemplos

[inline-code-attrs-start title = 'Exemplo de cURL para Remoção de SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Remoção de SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // padrão
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Você pode definir isso como true para também excluir os comentários do usuário. Isso dobrará o custo em créditos. **/
    deleteComments?: 'true' | 'false'
    /** Você pode definir isso conforme desejado para determinar como tratar os comentários do usuário. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Remoção de SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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