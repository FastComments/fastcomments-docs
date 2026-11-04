[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Este endpoint de API fornece a capacidade de atualizar um único comentário.

Observações:

- Esta API pode atualizar o widget de comentários "ao vivo" se desejado (isso aumenta o `creditsCost` base de `1` para `2`).
  - Isso pode tornar a migração de comentários entre páginas "ao vivo" (alterando `urlId`).
  - Migrações custam `2` créditos adicionais, pois as páginas são pré-calculadas e isso é intensivo em CPU.
- Diferente da API de criação, esta API NÃO criará automaticamente objetos de usuário em nosso sistema se o email for fornecido.
- Comentários atualizados via esta API ainda podem ser verificados quanto a spam, se desejado.
- Configurações como comprimento máximo do comentário, se configuradas através da página de administração de Regras de Personalização, serão aplicadas aqui.
- Para permitir que os usuários atualizem o texto de seus comentários, você pode simplesmente especificar `comment` no corpo da requisição. Nós geraremos o `commentHTML` resultante.
  - Se você definir tanto `comment` quanto `commentHTML`, não geraremos o HTML automaticamente.
  - Se o usuário adicionar menções ou hashtags em seu novo texto, elas ainda serão processadas como na API `POST`.
- Ao atualizar `commenterEmail` em um comentário, é melhor também especificar `userId`. Caso contrário, você deve garantir que o usuário com este email pertença ao seu tenant, ou a requisição falhará.
- Se o comentário de destino estiver bloqueado (`isLocked: true`), a requisição será rejeitada com `code: 'locked'`. Desbloqueie o comentário primeiro, atualize-o e, se desejado, bloqueie novamente.


[inline-code-attrs-start title = 'Exemplo mínimo de cURL para PATCH de comentário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da requisição PATCH de comentário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** O usuário realizando a atualização. Se desejado, pode ser usado para verificar se ele pode editar o comentário.  **/
    contextUserId?: string
	/** Devemos verificar se o novo comentário parece spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Se o comentário deve aparecer "ao vivo" para usuários visualizando instâncias do widget de comentários com o mesmo urlId. NOTA: Dobra o custo em créditos de 1 para 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da resposta PATCH de comentário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Incluído em caso de falha. **/
    reason?: string
}
[inline-code-end]