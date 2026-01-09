[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Este endpoint de API permite excluir um comentário.

Observações:

- Esta API pode atualizar o widget de comentários "live" se desejado (isso aumenta `creditsCost` de `1` para `2`).
- Esta API excluirá todos os comentários filhos.

[inline-code-attrs-start title = 'Exemplo cURL de DELETE de Comentário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Estrutura da Requisição DELETE de Comentário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** O usuário realizando a atualização. Se desejado, pode ser usado para verificar se ele pode excluir o comentário.  **/
    contextUserId?: string
	/** Se o comentário deve ser excluído "ao vivo" para usuários visualizando instâncias do widget de comentários com o mesmo urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta DELETE de Comentário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Incluído em caso de falha. **/
    reason?: string
}
[inline-code-end]